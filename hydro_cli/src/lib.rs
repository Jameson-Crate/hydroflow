use std::ops::DerefMut;
use std::sync::Arc;

use async_channel::Receiver;
use bytes::Bytes;
use futures::{Future, SinkExt, StreamExt};
use hydroflow_cli_integration::{
    ConnectedBidi, ConnectedSink, ConnectedSource, DynSink, DynStream, ServerOrBound,
};
use pyo3::exceptions::{PyException, PyStopAsyncIteration};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use pyo3::{create_exception, wrap_pymodule};
use pythonize::pythonize;
use tokio::sync::oneshot::Sender;
use tokio::sync::RwLock;

use crate::core::hydroflow_crate::ports::HydroflowSource;

mod cli;
pub mod core;

create_exception!(hydro_cli_core, AnyhowError, PyException);

#[pyclass]
struct SafeCancelToken {
    cancel_tx: Option<Sender<()>>,
}

#[pymethods]
impl SafeCancelToken {
    fn safe_cancel(&mut self) {
        if let Some(token) = self.cancel_tx.take() {
            let _ = token.send(());
        }
    }
}

static CONVERTERS_MODULE: std::sync::RwLock<Option<Py<PyModule>>> = std::sync::RwLock::new(None);

fn interruptible_future_to_py<F, T>(py: Python<'_>, fut: F) -> PyResult<&PyAny>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: IntoPy<PyObject>,
{
    let module = CONVERTERS_MODULE
        .read()
        .unwrap()
        .clone()
        .unwrap()
        .into_ref(py);

    let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel::<()>();

    let base_coro = pyo3_asyncio::tokio::future_into_py(py, async move {
        tokio::select! {
            biased;
            _ = cancel_rx => Ok(None),
            r = fut => r.map(|o| Some(o))
        }
    })?;

    module.call_method1(
        "coroutine_to_safely_cancellable",
        (
            base_coro,
            SafeCancelToken {
                cancel_tx: Some(cancel_tx),
            },
        ),
    )
}

#[pyclass]
#[derive(Clone)]
pub struct AnyhowWrapper {
    pub underlying: Arc<RwLock<Option<anyhow::Error>>>,
}

#[pymethods]
impl AnyhowWrapper {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{:?}",
            self.underlying.try_read().unwrap().as_ref().unwrap()
        ))
    }
}

#[pyclass(subclass)]
#[derive(Clone)]
struct HydroflowSink {
    underlying: Arc<RwLock<dyn crate::core::hydroflow_crate::ports::HydroflowSink>>,
}

#[pyclass(name = "Deployment")]
pub struct Deployment {
    underlying: Arc<RwLock<crate::core::Deployment>>,
}

#[pymethods]
impl Deployment {
    #[new]
    fn new() -> Self {
        Deployment {
            underlying: Arc::new(RwLock::new(crate::core::Deployment::default())),
        }
    }

    #[allow(non_snake_case)]
    fn Localhost(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let arc = self
            .underlying
            .blocking_write()
            .add_host(crate::core::LocalhostHost::new);

        Ok(Py::new(
            py,
            PyClassInitializer::from(Host {
                underlying: arc.clone(),
            })
            .add_subclass(LocalhostHost { underlying: arc }),
        )?
        .into_py(py))
    }

    #[allow(non_snake_case, clippy::too_many_arguments)]
    fn GCPComputeEngineHost(
        &self,
        py: Python<'_>,
        project: String,
        machine_type: String,
        image: String,
        region: String,
        network: GCPNetwork,
        user: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let arc = self.underlying.blocking_write().add_host(|id| {
            crate::core::GCPComputeEngineHost::new(
                id,
                project,
                machine_type,
                image,
                region,
                network.underlying,
                user,
            )
        });

        Ok(Py::new(
            py,
            PyClassInitializer::from(Host {
                underlying: arc.clone(),
            })
            .add_subclass(GCPComputeEngineHost { underlying: arc }),
        )?
        .into_py(py))
    }

    #[allow(non_snake_case)]
    fn CustomService(
        &self,
        py: Python<'_>,
        on: &Host,
        external_ports: Vec<u16>,
    ) -> PyResult<Py<PyAny>> {
        let service = self.underlying.blocking_write().add_service(|id| {
            crate::core::CustomService::new(id, on.underlying.clone(), external_ports)
        });

        Ok(Py::new(
            py,
            PyClassInitializer::from(Service {
                underlying: service.clone(),
            })
            .add_subclass(CustomService {
                underlying: service,
            }),
        )?
        .into_py(py))
    }

    #[allow(non_snake_case, clippy::too_many_arguments)]
    fn HydroflowCrate(
        &self,
        py: Python<'_>,
        src: String,
        on: &Host,
        example: Option<String>,
        features: Option<Vec<String>>,
        args: Option<Vec<String>>,
        display_id: Option<String>,
        external_ports: Option<Vec<u16>>,
    ) -> PyResult<Py<PyAny>> {
        let service = self.underlying.blocking_write().add_service(|id| {
            crate::core::HydroflowCrate::new(
                id,
                src.into(),
                on.underlying.clone(),
                example,
                features,
                args,
                display_id,
                external_ports.unwrap_or_default(),
            )
        });

        Ok(Py::new(
            py,
            PyClassInitializer::from(Service {
                underlying: service.clone(),
            })
            .add_subclass(HydroflowCrate {
                underlying: service,
            }),
        )?
        .into_py(py))
    }

    fn deploy<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        let py_none = py.None();
        interruptible_future_to_py(py, async move {
            underlying.write().await.deploy().await.map_err(|e| {
                AnyhowError::new_err(AnyhowWrapper {
                    underlying: Arc::new(RwLock::new(Some(e))),
                })
            })?;
            Ok(py_none)
        })
    }

    fn start<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        let py_none = py.None();
        interruptible_future_to_py(py, async move {
            underlying.write().await.start().await;
            Ok(py_none)
        })
    }
}

#[pyclass(subclass)]
pub struct Host {
    underlying: Arc<RwLock<dyn crate::core::Host>>,
}

#[pyclass(extends=Host, subclass)]
struct LocalhostHost {
    underlying: Arc<RwLock<crate::core::LocalhostHost>>,
}

#[pymethods]
impl LocalhostHost {
    fn client_only(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let arc = Arc::new(RwLock::new(
            self.underlying.try_read().unwrap().client_only(),
        ));

        Ok(Py::new(
            py,
            PyClassInitializer::from(Host {
                underlying: arc.clone(),
            })
            .add_subclass(LocalhostHost { underlying: arc }),
        )?
        .into_py(py))
    }
}

#[pyclass]
#[derive(Clone)]
struct GCPNetwork {
    underlying: Arc<RwLock<crate::core::gcp::GCPNetwork>>,
}

#[pymethods]
impl GCPNetwork {
    #[new]
    fn new(project: String, existing: Option<String>) -> Self {
        GCPNetwork {
            underlying: Arc::new(RwLock::new(crate::core::gcp::GCPNetwork::new(
                project, existing,
            ))),
        }
    }
}

#[pyclass(extends=Host, subclass)]
struct GCPComputeEngineHost {
    underlying: Arc<RwLock<crate::core::GCPComputeEngineHost>>,
}

#[pymethods]
impl GCPComputeEngineHost {
    #[getter]
    fn internal_ip(&self) -> String {
        self.underlying
            .blocking_read()
            .launched
            .as_ref()
            .unwrap()
            .internal_ip
            .clone()
    }

    #[getter]
    fn external_ip(&self) -> Option<String> {
        self.underlying
            .blocking_read()
            .launched
            .as_ref()
            .unwrap()
            .external_ip
            .clone()
    }

    #[getter]
    fn ssh_key_path(&self) -> String {
        self.underlying
            .blocking_read()
            .launched
            .as_ref()
            .unwrap()
            .ssh_key_path()
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[pyclass(subclass)]
pub struct Service {
    underlying: Arc<RwLock<dyn crate::core::Service>>,
}

#[pymethods]
impl Service {
    fn stop<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        let py_none = py.None();
        interruptible_future_to_py(py, async move {
            underlying.write().await.stop().await.unwrap();
            Ok(py_none)
        })
    }
}

#[pyclass]
struct PyReceiver {
    receiver: Arc<Receiver<String>>,
}

#[pymethods]
impl PyReceiver {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'p>(&self, py: Python<'p>) -> Option<&'p PyAny> {
        let my_receiver = self.receiver.clone();
        Some(
            interruptible_future_to_py(py, async move {
                let underlying = my_receiver.recv();
                underlying
                    .await
                    .map_err(|_| PyStopAsyncIteration::new_err(()))
            })
            .unwrap(),
        )
    }
}

#[pyclass(extends=Service, subclass)]
struct CustomService {
    underlying: Arc<RwLock<crate::core::CustomService>>,
}

#[pymethods]
impl CustomService {
    fn client_port(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let arc = Arc::new(RwLock::new(
            crate::core::custom_service::CustomClientPort::new(Arc::downgrade(&self.underlying)),
        ));

        Ok(Py::new(
            py,
            PyClassInitializer::from(HydroflowSink {
                underlying: arc.clone(),
            })
            .add_subclass(CustomClientPort { underlying: arc }),
        )?
        .into_py(py))
    }
}

#[pyclass(extends=HydroflowSink, subclass)]
#[derive(Clone)]
struct CustomClientPort {
    underlying: Arc<RwLock<crate::core::custom_service::CustomClientPort>>,
}

#[pymethods]
impl CustomClientPort {
    fn send_to(&mut self, to: &HydroflowSink) {
        self.underlying
            .try_write()
            .unwrap()
            .send_to(to.underlying.try_write().unwrap().deref_mut());
    }

    fn tagged(&self, tag: u32) -> TaggedSource {
        TaggedSource {
            underlying: Arc::new(RwLock::new(
                crate::core::hydroflow_crate::ports::TaggedSource {
                    source: self.underlying.clone(),
                    tag,
                },
            )),
        }
    }

    fn server_port<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        interruptible_future_to_py(py, async move {
            let underlying = underlying.read().await;
            Ok(ServerPort {
                underlying: Some(underlying.server_port().await),
            })
        })
    }
}

#[pyclass(extends=Service, subclass)]
struct HydroflowCrate {
    underlying: Arc<RwLock<crate::core::HydroflowCrate>>,
}

#[pymethods]
impl HydroflowCrate {
    fn stdout<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        interruptible_future_to_py(py, async move {
            let underlying = underlying.read().await;
            Ok(PyReceiver {
                receiver: Arc::new(underlying.stdout().await),
            })
        })
    }

    fn stderr<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        interruptible_future_to_py(py, async move {
            let underlying = underlying.read().await;
            Ok(PyReceiver {
                receiver: Arc::new(underlying.stderr().await),
            })
        })
    }

    fn exit_code<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        interruptible_future_to_py(py, async move {
            let underlying = underlying.read().await;
            Ok(underlying.exit_code().await)
        })
    }

    #[getter]
    fn ports(&self) -> HydroflowCratePorts {
        HydroflowCratePorts {
            underlying: self.underlying.clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct HydroflowCratePorts {
    underlying: Arc<RwLock<crate::core::HydroflowCrate>>,
}

#[pymethods]
impl HydroflowCratePorts {
    fn __getattribute__(&self, name: String, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let arc = Arc::new(RwLock::new(
            self.underlying
                .try_read()
                .unwrap()
                .get_port(name, &self.underlying),
        ));

        Ok(Py::new(
            py,
            PyClassInitializer::from(HydroflowSink {
                underlying: arc.clone(),
            })
            .add_subclass(HydroflowCratePort { underlying: arc }),
        )?
        .into_py(py))
    }
}

#[pyclass(extends=HydroflowSink, subclass)]
#[derive(Clone)]
struct HydroflowCratePort {
    underlying: Arc<RwLock<crate::core::hydroflow_crate::ports::HydroflowPortConfig>>,
}

#[pymethods]
impl HydroflowCratePort {
    fn merge(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let arc = Arc::new(RwLock::new(
            self.underlying.try_read().unwrap().clone().merge(),
        ));

        Ok(Py::new(
            py,
            PyClassInitializer::from(HydroflowSink {
                underlying: arc.clone(),
            })
            .add_subclass(HydroflowCratePort { underlying: arc }),
        )?
        .into_py(py))
    }

    fn send_to(&mut self, to: &mut HydroflowSink) {
        self.underlying
            .try_write()
            .unwrap()
            .send_to(to.underlying.try_write().unwrap().deref_mut());
    }

    fn tagged(&self, tag: u32) -> TaggedSource {
        TaggedSource {
            underlying: Arc::new(RwLock::new(
                crate::core::hydroflow_crate::ports::TaggedSource {
                    source: self.underlying.clone(),
                    tag,
                },
            )),
        }
    }
}

#[pyfunction]
fn demux(mapping: &PyDict) -> HydroflowSink {
    HydroflowSink {
        underlying: Arc::new(RwLock::new(
            crate::core::hydroflow_crate::ports::DemuxSink {
                demux: mapping
                    .into_iter()
                    .map(|(k, v)| {
                        let k = k.extract::<u32>().unwrap();
                        let v = v.extract::<HydroflowSink>().unwrap();
                        (k, v.underlying)
                    })
                    .collect(),
            },
        )),
    }
}

#[pyclass(subclass)]
#[derive(Clone)]
struct TaggedSource {
    underlying: Arc<RwLock<crate::core::hydroflow_crate::ports::TaggedSource>>,
}

#[pymethods]
impl TaggedSource {
    fn send_to(&mut self, to: &mut HydroflowSink) {
        self.underlying
            .try_write()
            .unwrap()
            .send_to(to.underlying.try_write().unwrap().deref_mut());
    }

    fn tagged(&self, tag: u32) -> TaggedSource {
        TaggedSource {
            underlying: Arc::new(RwLock::new(
                crate::core::hydroflow_crate::ports::TaggedSource {
                    source: self.underlying.clone(),
                    tag,
                },
            )),
        }
    }
}

#[pyclass(extends=HydroflowSink, subclass)]
#[derive(Clone)]
struct HydroflowNull {
    underlying: Arc<RwLock<crate::core::hydroflow_crate::ports::NullSourceSink>>,
}

#[pymethods]
impl HydroflowNull {
    fn send_to(&mut self, to: &mut HydroflowSink) {
        self.underlying
            .try_write()
            .unwrap()
            .send_to(to.underlying.try_write().unwrap().deref_mut());
    }

    fn tagged(&self, tag: u32) -> TaggedSource {
        TaggedSource {
            underlying: Arc::new(RwLock::new(
                crate::core::hydroflow_crate::ports::TaggedSource {
                    source: self.underlying.clone(),
                    tag,
                },
            )),
        }
    }
}

#[pyfunction]
fn null(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let arc = Arc::new(RwLock::new(
        crate::core::hydroflow_crate::ports::NullSourceSink,
    ));

    Ok(Py::new(
        py,
        PyClassInitializer::from(HydroflowSink {
            underlying: arc.clone(),
        })
        .add_subclass(HydroflowNull { underlying: arc }),
    )?
    .into_py(py))
}

#[pyclass]
struct ServerPort {
    underlying: Option<ServerOrBound>,
}

#[pymethods]
impl ServerPort {
    fn json(&self, py: Python<'_>) -> Py<PyAny> {
        if let ServerOrBound::Server(server) = self.underlying.as_ref().unwrap() {
            pythonize(py, &server).unwrap()
        } else {
            panic!()
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_source<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.take().unwrap();
        interruptible_future_to_py(py, async move {
            Ok(PythonStream {
                underlying: Arc::new(RwLock::new(
                    underlying.connect::<ConnectedBidi>().await.into_source(),
                )),
            })
        })
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_sink<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.take().unwrap();
        interruptible_future_to_py(py, async move {
            let connected = underlying.connect::<ConnectedBidi>().await.into_sink();

            Ok(PythonSink {
                underlying: Arc::new(RwLock::new(connected)),
            })
        })
    }
}

#[pyclass]
#[derive(Clone)]
struct PythonSink {
    underlying: Arc<RwLock<DynSink<Bytes>>>,
}

#[pymethods]
impl PythonSink {
    fn send<'p>(&mut self, data: Py<PyBytes>, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.underlying.clone();
        let bytes = Bytes::from(data.as_bytes(py).to_vec());
        interruptible_future_to_py(py, async move {
            underlying.write().await.send(bytes).await?;
            Ok(())
        })
    }
}

#[pyclass]
#[derive(Clone)]
struct PythonStream {
    underlying: Arc<RwLock<DynStream>>,
}

#[pymethods]
impl PythonStream {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'p>(&self, py: Python<'p>) -> Option<&'p PyAny> {
        let underlying = self.underlying.clone();
        Some(
            interruptible_future_to_py(py, async move {
                let read_res = underlying.write().await.next().await;
                read_res
                    .and_then(|b| b.ok().map(|b| b.to_vec()))
                    .map(Ok)
                    .unwrap_or(Err(PyStopAsyncIteration::new_err(())))
            })
            .unwrap(),
        )
    }
}

#[pymodule]
pub fn _core(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    *CONVERTERS_MODULE.write().unwrap() = Some(
        PyModule::from_code(
            py,
            r#"
import asyncio
async def coroutine_to_safely_cancellable(c, cancel_token):
    try:
        return await asyncio.shield(c)
    except asyncio.CancelledError:
        cancel_token.safe_cancel()
        await c
        raise asyncio.CancelledError()
"#,
            "converters",
            "converters",
        )?
        .into(),
    );

    module.add("AnyhowError", py.get_type::<AnyhowError>())?;
    module.add_class::<AnyhowWrapper>()?;

    module.add_class::<HydroflowSink>()?;
    module.add_class::<Deployment>()?;

    module.add_class::<Host>()?;
    module.add_class::<LocalhostHost>()?;

    module.add_class::<GCPNetwork>()?;
    module.add_class::<GCPComputeEngineHost>()?;

    module.add_class::<Service>()?;
    module.add_class::<CustomService>()?;
    module.add_class::<CustomClientPort>()?;
    module.add_class::<HydroflowCrate>()?;
    module.add_class::<HydroflowCratePort>()?;

    module.add_class::<ServerPort>()?;
    module.add_class::<PythonSink>()?;
    module.add_class::<PythonStream>()?;

    module.add_function(wrap_pyfunction!(demux, module)?)?;
    module.add_function(wrap_pyfunction!(null, module)?)?;

    module.add_wrapped(wrap_pymodule!(cli::cli))?;

    Ok(())
}
