use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context as _, Error, Result};
use async_ssh2_lite::ssh2::ErrorCode;
use async_ssh2_lite::{AsyncChannel, AsyncSession, SessionConfiguration};
use async_trait::async_trait;
use futures::io::BufReader as FuturesBufReader;
use futures::stream::FuturesUnordered;
use futures::{AsyncBufReadExt, AsyncWriteExt};
use hydroflow_cli_integration::ServerBindConfig;
use inferno::collapse::perf::Folder;
use inferno::collapse::Collapse;
use nanoid::nanoid;
use tokio::io::BufReader as TokioBufReader;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Handle;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::StreamExt;
use tokio_util::io::SyncIoBridge;

use super::progress::ProgressTracker;
use super::util::async_retry;
use super::{LaunchedBinary, LaunchedHost, ResourceResult, ServerStrategy};
use crate::hydroflow_crate::build::BuildOutput;
use crate::hydroflow_crate::perf_options::PerfOptions;
use crate::util::prioritized_broadcast;

const PERF_OUTFILE: &str = "__profile.perf.data";

struct LaunchedSshBinary {
    _resource_result: Arc<ResourceResult>,
    session: Option<AsyncSession<TcpStream>>,
    channel: AsyncChannel<TcpStream>,
    stdin_sender: mpsc::UnboundedSender<String>,
    stdout_receivers: Arc<Mutex<Vec<mpsc::UnboundedSender<String>>>>,
    stdout_cli_receivers: Arc<Mutex<Option<oneshot::Sender<String>>>>,
    stderr_receivers: Arc<Mutex<Vec<mpsc::UnboundedSender<String>>>>,
    perf: Option<PerfOptions>,
}

#[async_trait]
impl LaunchedBinary for LaunchedSshBinary {
    fn stdin(&self) -> mpsc::UnboundedSender<String> {
        self.stdin_sender.clone()
    }

    fn cli_stdout(&self) -> oneshot::Receiver<String> {
        let mut receivers = self.stdout_cli_receivers.lock().unwrap();

        if receivers.is_some() {
            panic!("Only one CLI stdout receiver is allowed at a time");
        }

        let (sender, receiver) = oneshot::channel::<String>();
        *receivers = Some(sender);
        receiver
    }

    fn stdout(&self) -> mpsc::UnboundedReceiver<String> {
        let mut receivers = self.stdout_receivers.lock().unwrap();
        let (sender, receiver) = mpsc::unbounded_channel::<String>();
        receivers.push(sender);
        receiver
    }

    fn stderr(&self) -> mpsc::UnboundedReceiver<String> {
        let mut receivers = self.stderr_receivers.lock().unwrap();
        let (sender, receiver) = mpsc::unbounded_channel::<String>();
        receivers.push(sender);
        receiver
    }

    fn exit_code(&self) -> Option<i32> {
        // until the program exits, the exit status is meaningless
        if self.channel.eof() {
            self.channel.exit_status().ok()
        } else {
            None
        }
    }

    async fn wait(&mut self) -> Result<i32> {
        self.channel.wait_eof().await.unwrap();
        let exit_code = self.channel.exit_status()?;
        self.channel.wait_close().await.unwrap();

        Ok(exit_code)
    }

    async fn stop(&mut self) -> Result<()> {
        if !self.channel.eof() {
            ProgressTracker::leaf("force stopping".to_owned(), async {
                self.channel.write_all(b"\x03").await?; // `^C`
                self.channel.send_eof().await?;
                self.channel.wait_eof().await?;
                // `exit_status()`
                self.channel.wait_close().await?;
                Result::<_>::Ok(())
            })
            .await?;
        }

        // Run perf post-processing and download perf output.
        if let Some(perf) = &mut self.perf {
            let mut script_channel = self.session.as_ref().unwrap().channel_session().await?;
            let mut fold_er = Folder::from(perf.fold_options.clone().unwrap_or_default());

            let fold_data = ProgressTracker::leaf("perf script & folding".to_owned(), async move {
                let mut stderr_lines = FuturesBufReader::new(script_channel.stderr()).lines();
                let stdout = script_channel.stream(0);

                // Pattern on `()` to make sure no `Result`s are ignored.
                let ((), fold_data, ()) = tokio::try_join!(
                    async move {
                        // Log stderr.
                        while let Some(Ok(s)) = stderr_lines.next().await {
                            ProgressTracker::println(&format!("[perf stderr] {s}"));
                        }
                        Result::<_>::Ok(())
                    },
                    async move {
                        // Download perf output and fold.
                        tokio::task::spawn_blocking(move || {
                            let mut fold_data = Vec::new();
                            fold_er.collapse(
                                SyncIoBridge::new(TokioBufReader::new(stdout)),
                                &mut fold_data,
                            )?;
                            Ok(fold_data)
                        })
                        .await?
                    },
                    async move {
                        // Run command (last!).
                        script_channel
                            .exec(&format!("perf script --symfs=/ -i {PERF_OUTFILE}"))
                            .await?;
                        Ok(())
                    },
                )?;
                Result::<_>::Ok(fold_data)
            })
            .await?;
            // Wrap in Arc to allow sharing data across multiple outputs.
            let fold_data = Arc::new(fold_data);
            let output_tasks =
                FuturesUnordered::<Pin<Box<dyn Future<Output = Result<()>> + Send + Sync>>>::new();

            // fold_outfile
            if let Some(fold_outfile) = perf.fold_outfile.clone() {
                let fold_data = Arc::clone(&fold_data);
                output_tasks.push(Box::pin(async move {
                    let mut reader = &**fold_data;
                    let mut writer = tokio::fs::File::create(fold_outfile).await?;
                    tokio::io::copy_buf(&mut reader, &mut writer).await?;
                    Ok(())
                }));
            };

            // flamegraph_outfile
            if let Some(flamegraph_outfile) = perf.flamegraph_outfile.clone() {
                let mut options = perf.flamegraph_options.map(|f| (f)()).unwrap_or_default();
                let fold_data = Arc::clone(&fold_data);
                output_tasks.push(Box::pin(async move {
                    let writer = tokio::fs::File::create(flamegraph_outfile)
                        .await?
                        .into_std()
                        .await;
                    tokio::task::spawn_blocking(move || {
                        let reader = &**fold_data;
                        inferno::flamegraph::from_lines(
                            &mut options,
                            reader
                                .split(|&b| b == b'\n')
                                .map(std::str::from_utf8)
                                .map(Result::unwrap),
                            writer,
                        )
                    })
                    .await??;
                    Ok(())
                }));
            };

            let errors = output_tasks
                .filter_map(Result::err)
                .collect::<Vec<_>>()
                .await;
            if !errors.is_empty() {
                Err(MultipleErrors { errors })?;
            };

            // TODO(mingwei): re-use this code to download `dtrace` data.
            // let sftp = self.session.as_ref().unwrap().sftp().await?;
            // // Check if perf.data exists
            // match sftp.open(&PathBuf::from(output_file)).await {
            //     Ok(perf_data) => {
            //         // download perf.data
            //         let mut perf_data_local = tokio::fs::File::create(output_file).await?;
            //         tokio::io::copy(&mut perf_data, &mut perf_data_local).await?;
            //     }
            //     Err(Error::Ssh2(ssh2_err)) if ErrorCode::SFTP(2) == ssh2_err.code() => {
            //         // File not found, probably due to some other previous error.
            //         ProgressTracker::println(&format!(
            //             "perf output file {:?} not found on remote machine (SFTP error 2).",
            //             output_file
            //         ));
            //     }
            //     Err(unexpected_err) => Err(unexpected_err)?,
            // }
        };

        Ok(())
    }
}

impl Drop for LaunchedSshBinary {
    fn drop(&mut self) {
        if let Some(session) = self.session.take() {
            tokio::task::block_in_place(|| {
                Handle::current().block_on(session.disconnect(None, "", None))
            })
            .unwrap();
        }
    }
}

#[async_trait]
pub trait LaunchedSshHost: Send + Sync {
    fn get_internal_ip(&self) -> String;
    fn get_external_ip(&self) -> Option<String>;
    fn get_cloud_provider(&self) -> String;
    fn resource_result(&self) -> &Arc<ResourceResult>;
    fn ssh_user(&self) -> &str;

    fn ssh_key_path(&self) -> PathBuf {
        self.resource_result()
            .terraform
            .deployment_folder
            .as_ref()
            .unwrap()
            .path()
            .join(".ssh")
            .join("vm_instance_ssh_key_pem")
    }

    fn server_config(&self, bind_type: &ServerStrategy) -> ServerBindConfig {
        match bind_type {
            ServerStrategy::UnixSocket => ServerBindConfig::UnixSocket,
            ServerStrategy::InternalTcpPort => {
                ServerBindConfig::TcpPort(self.get_internal_ip().clone())
            }
            ServerStrategy::ExternalTcpPort(_) => todo!(),
            ServerStrategy::Demux(demux) => {
                let mut config_map = HashMap::new();
                for (key, underlying) in demux {
                    config_map.insert(*key, LaunchedSshHost::server_config(self, underlying));
                }

                ServerBindConfig::Demux(config_map)
            }
            ServerStrategy::Merge(merge) => {
                let mut configs = vec![];
                for underlying in merge {
                    configs.push(LaunchedSshHost::server_config(self, underlying));
                }

                ServerBindConfig::Merge(configs)
            }
            ServerStrategy::Tagged(underlying, id) => ServerBindConfig::Tagged(
                Box::new(LaunchedSshHost::server_config(self, underlying)),
                *id,
            ),
            ServerStrategy::Null => ServerBindConfig::Null,
        }
    }

    async fn open_ssh_session(&self) -> Result<AsyncSession<TcpStream>> {
        let target_addr = SocketAddr::new(
            self.get_external_ip()
                .as_ref()
                .context(
                    self.get_cloud_provider()
                        + " host must be configured with an external IP to launch binaries",
                )?
                .parse()
                .unwrap(),
            22,
        );

        let res = ProgressTracker::leaf(
            format!(
                "connecting to host @ {}",
                self.get_external_ip().as_ref().unwrap()
            ),
            async_retry(
                &|| async {
                    let mut config = SessionConfiguration::new();
                    config.set_compress(true);

                    let mut session =
                        AsyncSession::<TcpStream>::connect(target_addr, Some(config)).await?;

                    session.handshake().await?;

                    session
                        .userauth_pubkey_file(
                            self.ssh_user(),
                            None,
                            self.ssh_key_path().as_path(),
                            None,
                        )
                        .await?;

                    Ok(session)
                },
                10,
                Duration::from_secs(1),
            ),
        )
        .await?;

        Ok(res)
    }
}

#[async_trait]
impl<T: LaunchedSshHost> LaunchedHost for T {
    fn server_config(&self, bind_type: &ServerStrategy) -> ServerBindConfig {
        LaunchedSshHost::server_config(self, bind_type)
    }

    async fn copy_binary(&self, binary: &BuildOutput) -> Result<()> {
        let session = self.open_ssh_session().await?;

        let sftp = async_retry(
            &|| async { Ok(session.sftp().await?) },
            10,
            Duration::from_secs(1),
        )
        .await?;

        // we may be deploying multiple binaries, so give each a unique name
        let unique_name = &binary.unique_id;

        let user = self.ssh_user();
        let binary_path = PathBuf::from(format!("/home/{user}/hydro-{unique_name}"));

        if sftp.stat(&binary_path).await.is_err() {
            let random = nanoid!(8);
            let temp_path = PathBuf::from(format!("/home/{user}/hydro-{random}"));
            let sftp = &sftp;

            ProgressTracker::rich_leaf(
                format!("uploading binary to {}", binary_path.display()),
                |set_progress, _| {
                    let binary = &binary;
                    let binary_path = &binary_path;
                    async move {
                        let mut created_file = sftp.create(&temp_path).await?;

                        let mut index = 0;
                        while index < binary.bin_data.len() {
                            let written = created_file
                                .write(
                                    &binary.bin_data[index
                                        ..std::cmp::min(index + 128 * 1024, binary.bin_data.len())],
                                )
                                .await?;
                            index += written;
                            set_progress(
                                ((index as f64 / binary.bin_data.len() as f64) * 100.0) as u64,
                            );
                        }
                        let mut orig_file_stat = sftp.stat(&temp_path).await?;
                        orig_file_stat.perm = Some(0o755); // allow the copied binary to be executed by anyone
                        created_file.setstat(orig_file_stat).await?;
                        created_file.close().await?;
                        drop(created_file);

                        match sftp.rename(&temp_path, binary_path, None).await {
                            Ok(_) => {}
                            Err(async_ssh2_lite::Error::Ssh2(e))
                                if e.code() == ErrorCode::SFTP(4) =>
                            {
                                // file already exists
                                sftp.unlink(&temp_path).await?;
                            }
                            Err(e) => return Err(e.into()),
                        }

                        anyhow::Ok(())
                    }
                },
            )
            .await?;
        }
        drop(sftp);

        Ok(())
    }

    async fn launch_binary(
        &self,
        id: String,
        binary: &BuildOutput,
        args: &[String],
        perf: Option<PerfOptions>,
    ) -> Result<Box<dyn LaunchedBinary>> {
        let session = self.open_ssh_session().await?;

        let unique_name = &binary.unique_id;

        let user = self.ssh_user();
        let binary_path = PathBuf::from(format!("/home/{user}/hydro-{unique_name}"));

        let channel = ProgressTracker::leaf(
            format!("launching binary {}", binary_path.display()),
            async {
                let mut channel =
                    async_retry(
                        &|| async {
                            Ok(tokio::time::timeout(
                                Duration::from_secs(60),
                                session.channel_session(),
                            )
                            .await??)
                        },
                        10,
                        Duration::from_secs(1),
                    )
                    .await?;

                let mut command = binary_path.to_str().unwrap().to_owned();
                for arg in args{
                    command.push(' ');
                    command.push_str(&shell_escape::unix::escape(Cow::Borrowed(arg)))
                }
                // Launch with perf if specified, also copy local binary to expected place for perf report to work
                if let Some(PerfOptions { frequency, .. }) = perf.clone() {
                    // Attach perf to the command
                    command = format!(
                        "perf record -F {frequency} --call-graph dwarf,64000 -o {PERF_OUTFILE} {command}",
                    );
                }
                channel.exec(&command).await?;
                anyhow::Ok(channel)
            },
        )
        .await?;

        let (stdin_sender, mut stdin_receiver) = mpsc::unbounded_channel::<String>();
        let mut stdin = channel.stream(0); // stream 0 is stdout/stdin, we use it for stdin
        tokio::spawn(async move {
            while let Some(line) = stdin_receiver.recv().await {
                if stdin.write_all(line.as_bytes()).await.is_err() {
                    break;
                }

                stdin.flush().await.unwrap();
            }
        });

        let id_clone = id.clone();
        let (stdout_cli_receivers, stdout_receivers) =
            prioritized_broadcast(FuturesBufReader::new(channel.stream(0)).lines(), move |s| {
                ProgressTracker::println(&format!("[{id_clone}] {s}"));
            });
        let (_, stderr_receivers) =
            prioritized_broadcast(FuturesBufReader::new(channel.stderr()).lines(), move |s| {
                ProgressTracker::println(&format!("[{id} stderr] {s}"));
            });

        Ok(Box::new(LaunchedSshBinary {
            _resource_result: self.resource_result().clone(),
            session: Some(session),
            channel,
            stdin_sender,
            stdout_cli_receivers,
            stdout_receivers,
            stderr_receivers,
            perf,
        }))
    }

    async fn forward_port(&self, addr: &SocketAddr) -> Result<SocketAddr> {
        let session = self.open_ssh_session().await?;

        let local_port = TcpListener::bind("127.0.0.1:0").await?;
        let local_addr = local_port.local_addr()?;

        let internal_ip = addr.ip().to_string();
        let port = addr.port();

        tokio::spawn(async move {
            #[allow(clippy::never_loop)]
            while let Ok((mut local_stream, _)) = local_port.accept().await {
                let mut channel = session
                    .channel_direct_tcpip(&internal_ip, port, None)
                    .await
                    .unwrap();
                let _ = tokio::io::copy_bidirectional(&mut local_stream, &mut channel).await;
                break;
                // TODO(shadaj): we should be returning an Arc so that we know
                // if anyone wants to connect to this forwarded port
            }

            ProgressTracker::println("[hydro] closing forwarded port");
        });

        Ok(local_addr)
    }
}

#[derive(Debug)]
struct MultipleErrors {
    errors: Vec<Error>,
}
impl std::fmt::Display for MultipleErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if 1 == self.errors.len() {
            self.errors.first().unwrap().fmt(f)
        } else {
            writeln!(f, "({}) errors occured:", self.errors.len())?;
            writeln!(f)?;
            for (i, error) in self.errors.iter().enumerate() {
                write!(f, "({}/{}):", i + 1, self.errors.len())?;
                error.fmt(f)?;
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
impl std::error::Error for MultipleErrors {}
