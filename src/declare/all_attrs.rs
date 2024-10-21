use std::borrow::Cow;

use crate::attr::Attr;
use crate::builder::Builder;

// Take care to name the parameter `value`
// to disable rust analyzer inlay hints
macro_rules! define_attr_function {
    ($name:ident, $key:literal) => {
        pub fn $name<'a, C>(value: C) -> Attr<'a>
        where
            C: Into<Cow<'a, str>>,
        {
            Attr($key, value.into())
        }
    };
    ($name:ident, $key:literal, $value:literal) => {
        pub fn $name<'a>() -> Attr<'a> {
            Attr($key, $value.into())
        }
    };
}

macro_rules! define_attr_method {
    ($name:ident, $key: literal) => {
        pub fn $name<C>(mut self, value: C) -> Builder<'a>
        where
            C: Into<Cow<'a, str>>,
        {
            if let Some(attrs) = self.element.attrs_mut() {
                attrs.push(Attr($key, value.into()));
            }
            self
        }
    };
    ($name:ident, $key: literal, $value:literal) => {
        pub fn $name<C>(mut self) -> Builder<'a> {
            if let Some(attrs) = self.element.attrs_mut() {
                attrs.push(Attr($key, $value.into()));
            }
            self
        }
    };
}

impl<'a> Builder<'a> {
    define_attr_method!(accept, "accept");
    define_attr_method!(accept_charset, "accept_charset");
    define_attr_method!(action, "action");
    define_attr_method!(alt, "alt");
    define_attr_method!(aria_checked, "aria_checked");
    define_attr_method!(aria_current, "aria_current");
    define_attr_method!(aria_disabled, "aria_disabled", "true");
    define_attr_method!(aria_hidden, "aria_hidden", "true");
    define_attr_method!(aria_invalid, "aria_invalid", "true");
    define_attr_method!(aria_label, "aria_label");
    define_attr_method!(aria_labelledby, "aria_labelledby");
    define_attr_method!(aria_placeholder, "aria_placeholder");
    define_attr_method!(aria_readonly, "aria_readonly");
    define_attr_method!(aria_required, "aria_required");
    define_attr_method!(async_, "async", "true");
    define_attr_method!(autocapitalize, "autocapitalize");
    define_attr_method!(autocomplete, "autocomplete");
    define_attr_method!(autofocus, "autofocus", "true");
    define_attr_method!(autoplay, "autoplay", "true");
    define_attr_method!(capture, "capture");
    define_attr_method!(charset, "charset");
    define_attr_method!(checked, "checked", "true");
    define_attr_method!(cite_attr, "cite_attr");
    define_attr_method!(class, "class");
    define_attr_method!(content, "content");
    define_attr_method!(contenteditable, "contenteditable", "true");
    define_attr_method!(crossorigin, "crossorigin");
    define_attr_method!(defer, "defer", "true");
    define_attr_method!(disabled, "disabled", "true");
    define_attr_method!(draggable, "draggable", "true");
    define_attr_method!(enctype, "enctype");
    define_attr_method!(for_, "for");
    define_attr_method!(formaction, "formaction");
    define_attr_method!(height, "height");
    define_attr_method!(href, "href");
    define_attr_method!(http_equiv, "http_equiv");
    define_attr_method!(id, "id");
    define_attr_method!(integrity, "integrity");
    define_attr_method!(lang, "lang");
    define_attr_method!(loop_, "loop", "true");
    define_attr_method!(maxlength, "maxlength");
    define_attr_method!(method, "method");
    define_attr_method!(minlength, "minlength");
    define_attr_method!(name, "name");
    define_attr_method!(placeholder, "placeholder");
    define_attr_method!(preload, "preload", "true");
    define_attr_method!(property, "property");
    define_attr_method!(readonly, "readonly", "true");
    define_attr_method!(rel, "rel");
    define_attr_method!(required, "required");
    define_attr_method!(role, "role");
    define_attr_method!(selected, "selected", "true");
    define_attr_method!(src, "src");
    define_attr_method!(style, "style");
    define_attr_method!(tabindex, "tabindex");
    define_attr_method!(target, "target");
    define_attr_method!(title, "title");
    define_attr_method!(type_, "type");
    define_attr_method!(value, "value");
    define_attr_method!(width, "width");
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
define_attr_function!(title, "title");
define_attr_function!(type_, "type");
define_attr_function!(value, "value");
define_attr_function!(width, "width");
