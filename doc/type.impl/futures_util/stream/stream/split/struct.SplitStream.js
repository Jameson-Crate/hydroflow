(function() {
    var type_impls = Object.fromEntries([["hydroflow",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-SplitStream%3CS%3E\" class=\"impl\"><a href=\"#impl-Debug-for-SplitStream%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for SplitStream&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","hydroflow::util::udp::UdpFramedStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SplitStream%3CS%3E\" class=\"impl\"><a href=\"#impl-SplitStream%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; SplitStream&lt;S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_pair_of\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">is_pair_of</a>&lt;Item&gt;(&amp;self, other: &amp;SplitSink&lt;S, Item&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Returns <code>true</code> if the <code>SplitStream&lt;S&gt;</code> and <code>SplitSink&lt;S&gt;</code> originate from the same call to <code>StreamExt::split</code>.</p>\n</div></details></div></details>",0,"hydroflow::util::udp::UdpFramedStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SplitStream%3CS%3E\" class=\"impl\"><a href=\"#impl-SplitStream%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; SplitStream&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.reunite\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">reunite</a>&lt;Item&gt;(\n    self,\n    other: SplitSink&lt;S, Item&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;S, ReuniteError&lt;S, Item&gt;&gt;<div class=\"where\">where\n    S: Sink&lt;Item&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Attempts to put the two “halves” of a split <code>Stream + Sink</code> back\ntogether. Succeeds only if the <code>SplitStream&lt;S&gt;</code> and <code>SplitSink&lt;S&gt;</code> are\na matching pair originating from the same call to <code>StreamExt::split</code>.</p>\n</div></details></div></details>",0,"hydroflow::util::udp::UdpFramedStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Stream-for-SplitStream%3CS%3E\" class=\"impl\"><a href=\"#impl-Stream-for-SplitStream%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; Stream for SplitStream&lt;S&gt;<div class=\"where\">where\n    S: Stream,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Item</a> = &lt;S as Stream&gt;::Item</h4></section></summary><div class='docblock'>Values yielded by the stream.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.poll_next\" class=\"method trait-impl\"><a href=\"#method.poll_next\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">poll_next</a>(\n    self: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;&amp;mut SplitStream&lt;S&gt;&gt;,\n    cx: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html\" title=\"struct core::task::wake::Context\">Context</a>&lt;'_&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html\" title=\"enum core::task::poll::Poll\">Poll</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&lt;S as Stream&gt;::Item&gt;&gt;</h4></section></summary><div class='docblock'>Attempt to pull out the next value of this stream, registering the\ncurrent task for wakeup if the value is not yet available, and returning\n<code>None</code> if the stream is exhausted. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.size_hint\" class=\"method trait-impl\"><a href=\"#method.size_hint\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">size_hint</a>(&amp;self) -&gt; (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;)</h4></section></summary><div class='docblock'>Returns the bounds on the remaining length of the stream. <a>Read more</a></div></details></div></details>","Stream","hydroflow::util::udp::UdpFramedStream"],["<section id=\"impl-Unpin-for-SplitStream%3CS%3E\" class=\"impl\"><a href=\"#impl-Unpin-for-SplitStream%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for SplitStream&lt;S&gt;</h3></section>","Unpin","hydroflow::util::udp::UdpFramedStream"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[7113]}