(function() {var implementors = {};
implementors["hydroflow"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/lang/collections/struct.Single.html\" title=\"struct hydroflow::lang::collections::Single\">Single</a>&lt;T&gt;","synthetic":false,"types":["hydroflow::lang::collections::Single"]},{"text":"impl&lt;I:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/lang/collections/struct.Iter.html\" title=\"struct hydroflow::lang::collections::Iter\">Iter</a>&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>,&nbsp;</span>","synthetic":false,"types":["hydroflow::lang::collections::Iter"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/lang/collections/struct.Array.html\" title=\"struct hydroflow::lang::collections::Array\">Array</a>&lt;T, N&gt;","synthetic":false,"types":["hydroflow::lang::collections::Array"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/lang/collections/struct.MaskedArray.html\" title=\"struct hydroflow::lang::collections::MaskedArray\">MaskedArray</a>&lt;T, N&gt;","synthetic":false,"types":["hydroflow::lang::collections::MaskedArray"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/scheduled/handoff/struct.TeeingHandoff.html\" title=\"struct hydroflow::scheduled::handoff::TeeingHandoff\">TeeingHandoff</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'static,&nbsp;</span>","synthetic":false,"types":["hydroflow::scheduled::handoff::tee::TeeingHandoff"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/scheduled/input/struct.Buffer.html\" title=\"struct hydroflow::scheduled::input::Buffer\">Buffer</a>&lt;T&gt;","synthetic":false,"types":["hydroflow::scheduled::input::Buffer"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/scheduled/net/struct.Message.html\" title=\"struct hydroflow::scheduled::net::Message\">Message</a>","synthetic":false,"types":["hydroflow::scheduled::net::Message"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/scheduled/reactor/struct.Reactor.html\" title=\"struct hydroflow::scheduled::reactor::Reactor\">Reactor</a>","synthetic":false,"types":["hydroflow::scheduled::reactor::Reactor"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"hydroflow/scheduled/state/struct.StateHandle.html\" title=\"struct hydroflow::scheduled::state::StateHandle\">StateHandle</a>&lt;T&gt;","synthetic":false,"types":["hydroflow::scheduled::state::StateHandle"]}];
implementors["relalg"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"relalg/enum.Datum.html\" title=\"enum relalg::Datum\">Datum</a>","synthetic":false,"types":["relalg::Datum"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"relalg/enum.ScalarExpr.html\" title=\"enum relalg::ScalarExpr\">ScalarExpr</a>","synthetic":false,"types":["relalg::ScalarExpr"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()