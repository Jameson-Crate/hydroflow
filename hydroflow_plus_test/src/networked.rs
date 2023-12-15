use hydroflow::bytes::BytesMut;
use hydroflow::util::cli::{ConnectedDirect, ConnectedSource, HydroCLI};
use hydroflow_plus::node::{CLIRuntimeNode, HfConnectable, HfNode};
use hydroflow_plus::scheduled::graph::Hydroflow;
use hydroflow_plus::HfBuilder;
use stageleft::{q, Quoted, RuntimeData};

pub fn networked_basic<'a, N0: HfNode<'a>, N1: HfNode<'a>>(
    graph: &'a HfBuilder<'a>,
    cli: RuntimeData<&'a HydroCLI>,
    node_zero: N0,
    node_one: N1,
) where
    N0: HfConnectable<'a, N1>,
{
    let source_zero = graph.source_stream(
        node_zero,
        q!({
            cli.port("node_zero_input")
                .connect_local_blocking::<ConnectedDirect>()
                .into_source()
        }),
    );

    source_zero
        .map(q!(|v| v.unwrap().freeze()))
        .send_to(node_one)
        .for_each(q!(|v: Result<BytesMut, _>| {
            println!(
                "node one received: {:?}",
                std::str::from_utf8(&v.unwrap()).unwrap()
            );
        }));
}

#[stageleft::entry]
pub fn networked_basic_runtime<'a>(
    graph: &'a HfBuilder<'a>,
    cli: RuntimeData<&'a HydroCLI>,
    node_id: RuntimeData<usize>,
) -> impl Quoted<'a, Hydroflow<'a>> {
    let node_zero = CLIRuntimeNode::new(0, cli);
    let node_one = CLIRuntimeNode::new(1, cli);
    networked_basic(graph, cli, &node_zero, &node_one);
    graph.build(node_id)
}

#[stageleft::runtime]
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use hydro_cli::core::hydroflow_crate::ports::HydroflowSource;
    use hydro_cli::core::Deployment;
    use hydroflow::futures::SinkExt;
    use hydroflow::util::cli::{ConnectedDirect, ConnectedSink};
    use hydroflow_plus_cli_integration::CLIDeployNode;

    #[tokio::test]
    async fn networked_basic() {
        let mut deployment = Deployment::new();
        let localhost = deployment.Localhost();

        let sender = deployment.CustomService(localhost.clone(), vec![]);

        let service_zero = CLIDeployNode::new(
            0,
            deployment.HydroflowCrate(
                ".",
                localhost.clone(),
                None,
                Some("networked_basic".into()),
                Some("dev".into()),
                None,
                Some(vec!["0".into()]),
                None,
                vec![],
            ),
        );

        let service_one = CLIDeployNode::new(
            1,
            deployment.HydroflowCrate(
                ".",
                localhost.clone(),
                None,
                Some("networked_basic".into()),
                Some("dev".into()),
                None,
                Some(vec!["1".into()]),
                None,
                vec![],
            ),
        );

        let builder = hydroflow_plus::HfBuilder::new();
        super::networked_basic(&builder, Default::default(), &service_zero, &service_one);

        let mut sender_port = sender.read().await.create_port(&sender);

        let mut node_zero_input = service_zero
            .underlying
            .read()
            .await
            .get_port("node_zero_input".to_string(), &service_zero.underlying);

        sender_port.send_to(&mut node_zero_input);

        deployment.deploy().await.unwrap();

        let mut connection = sender_port
            .server_port()
            .await
            .instantiate()
            .connect::<ConnectedDirect>()
            .await
            .into_sink();

        let service_1_stdout = service_one.underlying.read().await.stdout().await;

        deployment.start().await.unwrap();

        connection.send("hello world!".into()).await.unwrap();

        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), service_1_stdout.recv())
                .await
                .unwrap()
                .unwrap(),
            "node one received: \"hello world!\""
        );
    }
}
