mod all_attrs;
mod all_tags;

use crate::{
    attr::{Attr, Attrs},
    element::Element,
};

#[cfg(feature = "unstable-builder")]
pub use crate::builder::Builder;
pub use all_attrs::*;
pub use all_tags::*;
#[cfg(feature = "unstable-builder")]
pub use builder::*;

#[cfg(feature = "unstable-builder")]
mod builder {
    use crate::builder::{Builder, Path};
    use crate::{
        attr::{Attr, Attrs},
        element::Element,
    };
    use std::borrow::Cow;

    impl Builder {
        pub fn text<C>(self, value: C) -> Builder
        where
            C: Into<String>,
        {
            self.into_new_child_element(Element::Text { text: value.into() })
        }
    }
}

pub fn text<C>(value: C) -> Element
where
    C: Into<String>,
{
    Element::Text { text: value.into() }
}

/// Prepend `<!doctype html>` to the given children.
pub fn document() -> Element {
    Element::Document {
        children: Vec::new(),
    }
}

pub fn fragment() -> Element {
    Element::Fragment {
        children: Vec::new(),
    }
}

pub fn attr<C>(name: &'static str, value: C) -> Attrs
where
    C: Into<String>,
{
    Attrs(vec![Attr(name, value.into())])
}
