(function() {var implementors = {};
implementors["arrayvec"] = [{"text":"impl&lt;T, const CAP:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;T, CAP&gt;","synthetic":false,"types":["arrayvec::arrayvec::ArrayVec"]},{"text":"impl&lt;const CAP:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayString.html\" title=\"struct arrayvec::ArrayString\">ArrayString</a>&lt;CAP&gt;","synthetic":false,"types":["arrayvec::array_string::ArrayString"]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"crossbeam_epoch/trait.Pointable.html\" title=\"trait crossbeam_epoch::Pointable\">Pointable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Owned.html\" title=\"struct crossbeam_epoch::Owned\">Owned</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Owned"]}];
implementors["os_str_bytes"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"os_str_bytes/struct.RawOsStr.html\" title=\"struct os_str_bytes::RawOsStr\">RawOsStr</a>&gt; for <a class=\"struct\" href=\"os_str_bytes/struct.RawOsString.html\" title=\"struct os_str_bytes::RawOsString\">RawOsString</a>","synthetic":false,"types":["os_str_bytes::raw_str::RawOsString"]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/core/primitive.slice.html\">[</a>&lt;A as <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt;::<a class=\"associatedtype\" href=\"smallvec/trait.Array.html#associatedtype.Item\" title=\"type smallvec::Array::Item\">Item</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"smallvec/struct.SmallVec.html\" title=\"struct smallvec::SmallVec\">SmallVec</a>&lt;A&gt;","synthetic":false,"types":["smallvec::SmallVec"]}];
implementors["wgpu_core"] = [{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"wgpu_hal/trait.Api.html\" title=\"trait wgpu_hal::Api\">Api</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.unit.html\">()</a>&gt; for <a class=\"struct\" href=\"wgpu_core/binding_model/struct.BindGroup.html\" title=\"struct wgpu_core::binding_model::BindGroup\">BindGroup</a>&lt;A&gt;","synthetic":false,"types":["wgpu_core::binding_model::BindGroup"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"wgpu_hal/trait.Api.html\" title=\"trait wgpu_hal::Api\">Api</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.unit.html\">()</a>&gt; for <a class=\"struct\" href=\"wgpu_core/resource/struct.Buffer.html\" title=\"struct wgpu_core::resource::Buffer\">Buffer</a>&lt;A&gt;","synthetic":false,"types":["wgpu_core::resource::Buffer"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"wgpu_hal/trait.Api.html\" title=\"trait wgpu_hal::Api\">Api</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.unit.html\">()</a>&gt; for <a class=\"struct\" href=\"wgpu_core/resource/struct.TextureView.html\" title=\"struct wgpu_core::resource::TextureView\">TextureView</a>&lt;A&gt;","synthetic":false,"types":["wgpu_core::resource::TextureView"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"wgpu_hal/trait.Api.html\" title=\"trait wgpu_hal::Api\">Api</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.unit.html\">()</a>&gt; for <a class=\"struct\" href=\"wgpu_core/resource/struct.Sampler.html\" title=\"struct wgpu_core::resource::Sampler\">Sampler</a>&lt;A&gt;","synthetic":false,"types":["wgpu_core::resource::Sampler"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"wgpu_hal/trait.Api.html\" title=\"trait wgpu_hal::Api\">Api</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.unit.html\">()</a>&gt; for <a class=\"struct\" href=\"wgpu_core/resource/struct.QuerySet.html\" title=\"struct wgpu_core::resource::QuerySet\">QuerySet</a>&lt;A&gt;","synthetic":false,"types":["wgpu_core::resource::QuerySet"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()