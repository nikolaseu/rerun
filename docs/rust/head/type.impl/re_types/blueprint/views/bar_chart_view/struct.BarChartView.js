(function() {var type_impls = {
"re_space_view_bar_chart":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DerefMut-for-BarChartView\" class=\"impl\"><a href=\"#impl-DerefMut-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/ops/deref/trait.DerefMut.html\" title=\"trait core::ops::deref::DerefMut\">DerefMut</a> for BarChartView</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref_mut\" class=\"method trait-impl\"><a href=\"#method.deref_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut\" class=\"fn\">deref_mut</a>(&amp;mut self) -&gt; &amp;mut PlotLegend</h4></section></summary><div class='docblock'>Mutably dereferences the value.</div></details></div></details>","DerefMut","re_space_view_bar_chart::space_view_class::ViewType"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CT%3E-for-BarChartView\" class=\"impl\"><a href=\"#impl-From%3CT%3E-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for BarChartView<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;PlotLegend&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(v: T) -&gt; BarChartView</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<T>","re_space_view_bar_chart::space_view_class::ViewType"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BarChartView\" class=\"impl\"><a href=\"#impl-Debug-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for BarChartView</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","re_space_view_bar_chart::space_view_class::ViewType"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-View-for-BarChartView\" class=\"impl\"><a href=\"#impl-View-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl View for BarChartView</h3></section></summary><div class=\"impl-items\"><section id=\"method.identifier\" class=\"method trait-impl\"><a href=\"#method.identifier\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">identifier</a>() -&gt; SpaceViewClassIdentifier</h4></section></div></details>","View","re_space_view_bar_chart::space_view_class::ViewType"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-BarChartView\" class=\"impl\"><a href=\"#impl-Clone-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for BarChartView</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; BarChartView</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.76.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","re_space_view_bar_chart::space_view_class::ViewType"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-BarChartView\" class=\"impl\"><a href=\"#impl-Deref-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for BarChartView</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.76.0/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = PlotLegend</h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; &amp;PlotLegend</h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","re_space_view_bar_chart::space_view_class::ViewType"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Borrow%3CPlotLegend%3E-for-BarChartView\" class=\"impl\"><a href=\"#impl-Borrow%3CPlotLegend%3E-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;PlotLegend&gt; for BarChartView</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.borrow\" class=\"method trait-impl\"><a href=\"#method.borrow\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/borrow/trait.Borrow.html#tymethod.borrow\" class=\"fn\">borrow</a>(&amp;self) -&gt; &amp;PlotLegend</h4></section></summary><div class='docblock'>Immutably borrows from an owned value. <a href=\"https://doc.rust-lang.org/1.76.0/core/borrow/trait.Borrow.html#tymethod.borrow\">Read more</a></div></details></div></details>","Borrow<PlotLegend>","re_space_view_bar_chart::space_view_class::ViewType"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SizeBytes-for-BarChartView\" class=\"impl\"><a href=\"#impl-SizeBytes-for-BarChartView\" class=\"anchor\">§</a><h3 class=\"code-header\">impl SizeBytes for BarChartView</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.heap_size_bytes\" class=\"method trait-impl\"><a href=\"#method.heap_size_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">heap_size_bytes</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Returns the total size of <code>self</code> on the heap, in bytes.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_pod\" class=\"method trait-impl\"><a href=\"#method.is_pod\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_pod</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Is <code>Self</code> just plain old data? <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.total_size_bytes\" class=\"method trait-impl\"><a href=\"#method.total_size_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">total_size_bytes</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Returns the total size of <code>self</code> in bytes, accounting for both stack and heap space.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.stack_size_bytes\" class=\"method trait-impl\"><a href=\"#method.stack_size_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">stack_size_bytes</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Returns the total size of <code>self</code> on the stack, in bytes. <a>Read more</a></div></details></div></details>","SizeBytes","re_space_view_bar_chart::space_view_class::ViewType"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()