use hydroflow_plus::hydroflow::bytes::Bytes;
use hydroflow_plus_kvs_flow::my_example_flow;

fn main() {
    let (_send, recv) = hydroflow_plus::hydroflow::util::unbounded_channel::<Bytes>();
    let mut flow = my_example_flow!(recv);
    flow.run_tick();
}
