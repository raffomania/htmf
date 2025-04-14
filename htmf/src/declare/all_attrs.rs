use crate::attr::Attr;
use crate::attr::Attrs;

// Take care to name the parameter `value`
// to disable rust analyzer inlay hints
macro_rules! define_attr_function {
    ($name:ident, $key:literal) => {
        pub fn $name<C>(value: C) -> Attrs
        where
            C: ToString,
        {
            Attrs(vec![Attr($key, value.to_string())])
        }
    };
    ($name:ident, $key:literal, $value:literal) => {
        pub fn $name() -> Attrs {
            Attrs(vec![Attr($key, $value.into())])
        }
    };
}

define_attr_function!(accept, "accept");
define_attr_function!(accept_charset, "accept_charset");
define_attr_function!(action, "action");
define_attr_function!(alt, "alt");
define_attr_function!(aria_checked, "aria_checked");
define_attr_function!(aria_current, "aria_current");
define_attr_function!(aria_disabled, "aria_disabled", "true");
define_attr_function!(aria_hidden, "aria_hidden", "true");
define_attr_function!(aria_invalid, "aria_invalid", "true");
define_attr_function!(aria_label, "aria_label");
define_attr_function!(aria_labelledby, "aria_labelledby");
define_attr_function!(aria_placeholder, "aria_placeholder");
define_attr_function!(aria_readonly, "aria_readonly");
define_attr_function!(aria_required, "aria_required");
define_attr_function!(async_, "async", "true");
define_attr_function!(autocapitalize, "autocapitalize");
define_attr_function!(autocomplete, "autocomplete");
define_attr_function!(autofocus, "autofocus", "true");
define_attr_function!(autoplay, "autoplay", "true");
define_attr_function!(capture, "capture");
define_attr_function!(charset, "charset");
define_attr_function!(checked, "checked", "true");
define_attr_function!(cite_attr, "cite_attr");
define_attr_function!(class, "class");
define_attr_function!(content, "content");
define_attr_function!(contenteditable, "contenteditable", "true");
define_attr_function!(crossorigin, "crossorigin");
define_attr_function!(defer, "defer", "true");
define_attr_function!(disabled, "disabled", "true");
define_attr_function!(draggable, "draggable", "true");
define_attr_function!(enctype, "enctype");
define_attr_function!(for_, "for");
define_attr_function!(formaction, "formaction");
define_attr_function!(height, "height");
define_attr_function!(href, "href");
define_attr_function!(http_equiv, "http_equiv");
define_attr_function!(id, "id");
define_attr_function!(integrity, "integrity");
define_attr_function!(lang, "lang");
define_attr_function!(loop_, "loop", "true");
define_attr_function!(maxlength, "maxlength");
define_attr_function!(method, "method");
define_attr_function!(minlength, "minlength");
define_attr_function!(name, "name");
define_attr_function!(placeholder, "placeholder");
define_attr_function!(preload, "preload", "true");
define_attr_function!(property, "property");
define_attr_function!(readonly, "readonly", "true");
define_attr_function!(rel, "rel");
define_attr_function!(required, "required");
define_attr_function!(role, "role");
define_attr_function!(selected, "selected", "true");
define_attr_function!(src, "src");
define_attr_function!(style, "style");
define_attr_function!(tabindex, "tabindex");
define_attr_function!(target, "target");
define_attr_function!(title_attr, "title");
define_attr_function!(type_, "type");
define_attr_function!(value, "value");
define_attr_function!(width, "width");
