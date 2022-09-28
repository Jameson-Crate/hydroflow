(function() {var implementors = {
"hydroflow":[["impl&lt;Hof&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/builder/build/pull_handoff/struct.HandoffPullBuild.html\" title=\"struct hydroflow::builder::build::pull_handoff::HandoffPullBuild\">HandoffPullBuild</a>&lt;Hof&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Hof: <a class=\"trait\" href=\"hydroflow/scheduled/handoff/trait.Handoff.html\" title=\"trait hydroflow::scheduled::handoff::Handoff\">Handoff</a>,</span>"],["impl&lt;Hof, In&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/builder/build/push_handoff/struct.HandoffPushBuild.html\" title=\"struct hydroflow::builder::build::push_handoff::HandoffPushBuild\">HandoffPushBuild</a>&lt;Hof, In&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Hof: <a class=\"trait\" href=\"hydroflow/scheduled/handoff/trait.Handoff.html\" title=\"trait hydroflow::scheduled::handoff::Handoff\">Handoff</a> + <a class=\"trait\" href=\"hydroflow/scheduled/handoff/trait.CanReceive.html\" title=\"trait hydroflow::scheduled::handoff::CanReceive\">CanReceive</a>&lt;In&gt;,</span>"],["impl&lt;Out&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/builder/surface/push_start/struct.StartPushSurface.html\" title=\"struct hydroflow::builder::surface::push_start::StartPushSurface\">StartPushSurface</a>&lt;Out&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/builder/prelude/struct.HydroflowBuilder.html\" title=\"struct hydroflow::builder::prelude::HydroflowBuilder\">HydroflowBuilder</a>"],["impl&lt;L&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/compiled/pull/struct.BatchJoinState.html\" title=\"struct hydroflow::compiled::pull::BatchJoinState\">BatchJoinState</a>&lt;L&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"hydroflow/lang/lattice/trait.LatticeRepr.html\" title=\"trait hydroflow::lang::lattice::LatticeRepr\">LatticeRepr</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;L::<a class=\"associatedtype\" href=\"hydroflow/lang/lattice/trait.LatticeRepr.html#associatedtype.Repr\" title=\"type hydroflow::lang::lattice::LatticeRepr::Repr\">Repr</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span>"],["impl&lt;V1, V2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/compiled/pull/struct.CrossJoinState.html\" title=\"struct hydroflow::compiled::pull::CrossJoinState\">CrossJoinState</a>&lt;V1, V2&gt;"],["impl&lt;K, L&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/compiled/pull/struct.HalfHashJoinState.html\" title=\"struct hydroflow::compiled::pull::HalfHashJoinState\">HalfHashJoinState</a>&lt;K, L&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"hydroflow/lang/lattice/trait.LatticeRepr.html\" title=\"trait hydroflow::lang::lattice::LatticeRepr\">LatticeRepr</a>,</span>"],["impl&lt;K, V1, V2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/compiled/pull/struct.JoinState.html\" title=\"struct hydroflow::compiled::pull::JoinState\">JoinState</a>&lt;K, V1, V2&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/lang/monotonic_map/struct.MonotonicMap.html\" title=\"struct hydroflow::lang::monotonic_map::MonotonicMap\">MonotonicMap</a>&lt;K, V&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"hydroflow/lang/clear/trait.Clear.html\" title=\"trait hydroflow::lang::clear::Clear\">Clear</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/scheduled/flow_graph/struct.FlowEdgeSet.html\" title=\"struct hydroflow::scheduled::flow_graph::FlowEdgeSet\">FlowEdgeSet</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/scheduled/flow_graph/struct.FlowPartitionedGraph.html\" title=\"struct hydroflow::scheduled::flow_graph::FlowPartitionedGraph\">FlowPartitionedGraph</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/scheduled/flow_graph/struct.FlowGraph.html\" title=\"struct hydroflow::scheduled::flow_graph::FlowGraph\">FlowGraph</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/scheduled/graph/struct.Hydroflow.html\" title=\"struct hydroflow::scheduled::graph::Hydroflow\">Hydroflow</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow::scheduled::handoff::TeeingHandoff\">TeeingHandoff</a>&lt;T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/builder/prelude/struct.VecHandoff.html\" title=\"struct hydroflow::builder::prelude::VecHandoff\">VecHandoff</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'static,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/scheduled/input/struct.Buffer.html\" title=\"struct hydroflow::scheduled::input::Buffer\">Buffer</a>&lt;T&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow/scheduled/query/struct.Query.html\" title=\"struct hydroflow::scheduled::query::Query\">Query</a>"]],
"hydroflow_lang":[["impl&lt;V, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/di_mul_graph/struct.DiMulGraph.html\" title=\"struct hydroflow_lang::graph::di_mul_graph::DiMulGraph\">DiMulGraph</a>&lt;V, E&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://docs.rs/slotmap/1.0.6/slotmap/trait.Key.html\" title=\"trait slotmap::Key\">Key</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://docs.rs/slotmap/1.0.6/slotmap/trait.Key.html\" title=\"trait slotmap::Key\">Key</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/flat_graph/struct.FlatGraph.html\" title=\"struct hydroflow_lang::graph::flat_graph::FlatGraph\">FlatGraph</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/ops/struct.OperatorWriteOutput.html\" title=\"struct hydroflow_lang::graph::ops::OperatorWriteOutput\">OperatorWriteOutput</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/partitioned_graph/struct.PartitionedGraph.html\" title=\"struct hydroflow_lang::graph::partitioned_graph::PartitionedGraph\">PartitionedGraph</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/serde_graph/struct.SerdeGraph.html\" title=\"struct hydroflow_lang::graph::serde_graph::SerdeGraph\">SerdeGraph</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/struct.GraphNodeId.html\" title=\"struct hydroflow_lang::graph::GraphNodeId\">GraphNodeId</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/struct.GraphEdgeId.html\" title=\"struct hydroflow_lang::graph::GraphEdgeId\">GraphEdgeId</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/graph/struct.GraphSubgraphId.html\" title=\"struct hydroflow_lang::graph::GraphSubgraphId\">GraphSubgraphId</a>"],["impl&lt;K:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"hydroflow_lang/union_find/struct.UnionFind.html\" title=\"struct hydroflow_lang::union_find::UnionFind\">UnionFind</a>&lt;K&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://docs.rs/slotmap/1.0.6/slotmap/trait.Key.html\" title=\"trait slotmap::Key\">Key</a>,</span>"]],
"pusherator":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"pusherator/struct.InputBuild.html\" title=\"struct pusherator::InputBuild\">InputBuild</a>&lt;T&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()