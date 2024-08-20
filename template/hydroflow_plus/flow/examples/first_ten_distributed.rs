use hydro_deploy::{Deployment, HydroflowCrate};
use hydroflow_plus_cli_integration::DeployProcessSpec;

#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();

    let flow = hydroflow_plus::FlowBuilder::new();
    let (p1, p2) = flow::first_ten_distributed::first_ten_distributed(&flow);

    let _nodes = flow
        .with_default_optimize()
        .with_process(
            &p1,
            DeployProcessSpec::new({
                HydroflowCrate::new(".", deployment.Localhost())
                    .bin("first_ten_distributed")
                    .profile("dev")
            }),
        )
        .with_process(
            &p2,
            DeployProcessSpec::new({
                HydroflowCrate::new(".", deployment.Localhost())
                    .bin("first_ten_distributed")
                    .profile("dev")
            }),
        )
        .deploy(&mut deployment);

    deployment.run_ctrl_c().await.unwrap();
}
