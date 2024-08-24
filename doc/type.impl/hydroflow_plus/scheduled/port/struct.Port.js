(function() {var type_impls = {
"hydroflow_plus":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Port%3CRECV,+TeeingHandoff%3CT%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hydroflow/scheduled/port.rs.html#52\">source</a><a href=\"#impl-Port%3CRECV,+TeeingHandoff%3CT%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"struct\" href=\"hydroflow_plus/scheduled/port/struct.Port.html\" title=\"struct hydroflow_plus::scheduled::port::Port\">Port</a>&lt;<a class=\"enum\" href=\"hydroflow_plus/scheduled/port/enum.RECV.html\" title=\"enum hydroflow_plus::scheduled::port::RECV\">RECV</a>, <a class=\"struct\" href=\"hydroflow_plus/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow_plus::scheduled::handoff::TeeingHandoff\">TeeingHandoff</a>&lt;T&gt;&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"docblock\"><p>Methods for <a href=\"hydroflow_plus/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow_plus::scheduled::handoff::TeeingHandoff\"><code>TeeingHandoff</code></a> teeing and dropping.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.tee\" class=\"method\"><a class=\"src rightside\" href=\"src/hydroflow/scheduled/port.rs.html#54\">source</a><h4 class=\"code-header\">pub fn <a href=\"hydroflow_plus/scheduled/port/struct.Port.html#tymethod.tee\" class=\"fn\">tee</a>(&amp;self, hf: &amp;mut <a class=\"struct\" href=\"hydroflow_plus/scheduled/graph/struct.Hydroflow.html\" title=\"struct hydroflow_plus::scheduled::graph::Hydroflow\">Hydroflow</a>&lt;'_&gt;) -&gt; <a class=\"struct\" href=\"hydroflow_plus/scheduled/port/struct.Port.html\" title=\"struct hydroflow_plus::scheduled::port::Port\">Port</a>&lt;<a class=\"enum\" href=\"hydroflow_plus/scheduled/port/enum.RECV.html\" title=\"enum hydroflow_plus::scheduled::port::RECV\">RECV</a>, <a class=\"struct\" href=\"hydroflow_plus/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow_plus::scheduled::handoff::TeeingHandoff\">TeeingHandoff</a>&lt;T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Tees this <a href=\"hydroflow_plus/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow_plus::scheduled::handoff::TeeingHandoff\"><code>TeeingHandoff</code></a>, given the <a href=\"hydroflow_plus/scheduled/graph/struct.Hydroflow.html\" title=\"struct hydroflow_plus::scheduled::graph::Hydroflow\"><code>Hydroflow</code></a> instance it belongs to.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method\"><a class=\"src rightside\" href=\"src/hydroflow/scheduled/port.rs.html#63\">source</a><h4 class=\"code-header\">pub fn <a href=\"hydroflow_plus/scheduled/port/struct.Port.html#tymethod.drop\" class=\"fn\">drop</a>(self, hf: &amp;mut <a class=\"struct\" href=\"hydroflow_plus/scheduled/graph/struct.Hydroflow.html\" title=\"struct hydroflow_plus::scheduled::graph::Hydroflow\">Hydroflow</a>&lt;'_&gt;)</h4></section></summary><div class=\"docblock\"><p>Marks this output of a <a href=\"hydroflow_plus/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow_plus::scheduled::handoff::TeeingHandoff\"><code>TeeingHandoff</code></a> as dropped so that no more data will be sent to\nit, given the <a href=\"hydroflow_plus/scheduled/graph/struct.Hydroflow.html\" title=\"struct hydroflow_plus::scheduled::graph::Hydroflow\"><code>Hydroflow</code></a> instance it belongs to.</p>\n<p>It is recommended to not not use this method and instead simply avoid teeing a\n<a href=\"hydroflow_plus/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow_plus::scheduled::handoff::TeeingHandoff\"><code>TeeingHandoff</code></a> when it is not needed.</p>\n</div></details></div></details>",0,"hydroflow_plus::scheduled::port::RecvPort"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()