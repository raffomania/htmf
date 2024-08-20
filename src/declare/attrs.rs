use std::borrow::Cow;

use crate::Attr;

macro_rules! define_attr_function {
    ($name:ident) => {
        pub fn $name<'a, C>(content: C) -> Attr<'a>
        where
            C: Into<Cow<'a, str>>,
        {
            (stringify!($name), content.into())
        }
    };
    ($name:ident, $value:literal) => {
        pub fn $name<'a>() -> Attr<'a> {
            (stringify!($name), $value.into())
        }
    };
}

define_attr_function!(accept);
define_attr_function!(accept_charset);
define_attr_function!(action);
define_attr_function!(alt);
define_attr_function!(aria_checked);
define_attr_function!(aria_current);
define_attr_function!(aria_disabled, "true");
define_attr_function!(aria_hidden, "true");
define_attr_function!(aria_invalid, "true");
define_attr_function!(aria_label);
define_attr_function!(aria_labelledby);
define_attr_function!(aria_placeholder);
define_attr_function!(aria_readonly);
define_attr_function!(aria_required);
define_attr_function!(async_, "true");
define_attr_function!(autocapitalize);
define_attr_function!(autocomplete);
define_attr_function!(autofocus, "true");
define_attr_function!(autoplay, "true");
define_attr_function!(capture);
define_attr_function!(charset);
define_attr_function!(checked, "true");
define_attr_function!(cite_attr);
define_attr_function!(class);
define_attr_function!(content);
define_attr_function!(contenteditable, "true");
define_attr_function!(crossorigin);
define_attr_function!(defer, "true");
define_attr_function!(disabled, "true");
define_attr_function!(draggable, "true");
define_attr_function!(enctype);
define_attr_function!(for_);
define_attr_function!(formaction);
define_attr_function!(height);
define_attr_function!(href);
define_attr_function!(http_equiv);
define_attr_function!(id);
define_attr_function!(integrity);
define_attr_function!(lang);
define_attr_function!(loop_, "true");
define_attr_function!(maxlength);
define_attr_function!(method);
define_attr_function!(minlength);
define_attr_function!(name);
define_attr_function!(placeholder);
define_attr_function!(preload, "true");
define_attr_function!(property);
define_attr_function!(readonly, "true");
define_attr_function!(rel);
define_attr_function!(required);
define_attr_function!(role);
define_attr_function!(selected, "true");
define_attr_function!(src);
define_attr_function!(style);
define_attr_function!(tabindex);
define_attr_function!(target);
define_attr_function!(title);
define_attr_function!(type_);
define_attr_function!(value);
define_attr_function!(width);
