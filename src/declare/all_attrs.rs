use std::borrow::Cow;

use crate::attr::Attr;
use crate::element::Element;

// Take care to name the parameter `value`
// to disable rust analyzer inlay hints
macro_rules! define_attr_function {
    ($name:ident) => {
        pub fn $name<'a, C>(value: C) -> Attr<'a>
        where
            C: Into<Cow<'a, str>>,
        {
            Attr(stringify!($name), value.into())
        }
    };
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
    ($name:ident) => {
        pub fn $name<C>(mut self, value: C) -> Element<'a>
        where
            C: Into<Cow<'a, str>>,
        {
            if let Some(attrs) = self.element.attrs_mut() {
                attrs.push($name(value));
            }
            self
        }
    };
    ($name:ident, $value:literal) => {
        pub fn $name<C>(mut self) -> Element<'a> {
            if let Some(attrs) = self.element.attrs_mut() {
                attrs.push($name());
            }
            self
        }
    };
}

impl<'a> Element<'a> {
    define_attr_method!(accept);
    define_attr_method!(accept_charset);
    define_attr_method!(action);
    define_attr_method!(alt);
    define_attr_method!(aria_checked);
    define_attr_method!(aria_current);
    define_attr_method!(aria_disabled, "true");
    define_attr_method!(aria_hidden, "true");
    define_attr_method!(aria_invalid, "true");
    define_attr_method!(aria_label);
    define_attr_method!(aria_labelledby);
    define_attr_method!(aria_placeholder);
    define_attr_method!(aria_readonly);
    define_attr_method!(aria_required);
    define_attr_method!(async_, "true");
    define_attr_method!(autocapitalize);
    define_attr_method!(autocomplete);
    define_attr_method!(autofocus, "true");
    define_attr_method!(autoplay, "true");
    define_attr_method!(capture);
    define_attr_method!(charset);
    define_attr_method!(checked, "true");
    define_attr_method!(cite_attr);
    define_attr_method!(class);
    define_attr_method!(content);
    define_attr_method!(contenteditable, "true");
    define_attr_method!(crossorigin);
    define_attr_method!(defer, "true");
    define_attr_method!(disabled, "true");
    define_attr_method!(draggable, "true");
    define_attr_method!(enctype);
    define_attr_method!(for_);
    define_attr_method!(formaction);
    define_attr_method!(height);
    define_attr_method!(href);
    define_attr_method!(http_equiv);
    define_attr_method!(id);
    define_attr_method!(integrity);
    define_attr_method!(lang);
    define_attr_method!(loop_, "true");
    define_attr_method!(maxlength);
    define_attr_method!(method);
    define_attr_method!(minlength);
    define_attr_method!(name);
    define_attr_method!(placeholder);
    define_attr_method!(preload, "true");
    define_attr_method!(property);
    define_attr_method!(readonly, "true");
    define_attr_method!(rel);
    define_attr_method!(required);
    define_attr_method!(role);
    define_attr_method!(selected, "true");
    define_attr_method!(src);
    define_attr_method!(style);
    define_attr_method!(tabindex);
    define_attr_method!(target);
    define_attr_method!(title);
    define_attr_method!(type_);
    define_attr_method!(value);
    define_attr_method!(width);
}

define_attr_function!(accept);
define_attr_function!(accept_charset);
define_attr_function!(action);
define_attr_function!(alt);
define_attr_function!(aria_checked);
define_attr_function!(aria_current);
define_attr_function!(aria_disabled, "aria_disabled", "true");
define_attr_function!(aria_hidden, "aria_hidden", "true");
define_attr_function!(aria_invalid, "aria_invalid", "true");
define_attr_function!(aria_label);
define_attr_function!(aria_labelledby);
define_attr_function!(aria_placeholder);
define_attr_function!(aria_readonly);
define_attr_function!(aria_required);
define_attr_function!(async_, "async", "true");
define_attr_function!(autocapitalize);
define_attr_function!(autocomplete);
define_attr_function!(autofocus, "autofocus", "true");
define_attr_function!(autoplay, "autoplay", "true");
define_attr_function!(capture);
define_attr_function!(charset);
define_attr_function!(checked, "checked", "true");
define_attr_function!(cite_attr);
define_attr_function!(class);
define_attr_function!(content);
define_attr_function!(contenteditable, "contenteditable", "true");
define_attr_function!(crossorigin);
define_attr_function!(defer, "defer", "true");
define_attr_function!(disabled, "disabled", "true");
define_attr_function!(draggable, "draggable", "true");
define_attr_function!(enctype);
define_attr_function!(for_, "for");
define_attr_function!(formaction);
define_attr_function!(height);
define_attr_function!(href);
define_attr_function!(http_equiv);
define_attr_function!(id);
define_attr_function!(integrity);
define_attr_function!(lang);
define_attr_function!(loop_, "loop", "true");
define_attr_function!(maxlength);
define_attr_function!(method);
define_attr_function!(minlength);
define_attr_function!(name);
define_attr_function!(placeholder);
define_attr_function!(preload, "preload", "true");
define_attr_function!(property);
define_attr_function!(readonly, "readonly", "true");
define_attr_function!(rel);
define_attr_function!(required);
define_attr_function!(role);
define_attr_function!(selected, "selected", "true");
define_attr_function!(src);
define_attr_function!(style);
define_attr_function!(tabindex);
define_attr_function!(target);
define_attr_function!(title);
define_attr_function!(type_, "type");
define_attr_function!(value);
define_attr_function!(width);
