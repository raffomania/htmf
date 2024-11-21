mod all_attrs;
mod all_tags;

use std::borrow::Cow;

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

    impl<'e> Builder<'e> {
        pub fn text<C>(self, value: C) -> Builder<'e>
        where
            C: Into<Cow<'e, str>>,
        {
            self.into_new_child_element(Element::Text { text: value.into() })
        }
    }
}

pub fn text<'e, C>(value: C) -> Element<'e>
where
    C: Into<Cow<'e, str>>,
{
    Element::Text { text: value.into() }
}

/// Prepend `<!doctype html>` to the given children.
pub fn document<'e>() -> Element<'e> {
    Element::Document {
        children: Vec::new(),
    }
}

pub fn fragment<'e>() -> Element<'e> {
    Element::Fragment {
        children: Vec::new(),
    }
}

pub fn attr<'a, C>(name: &'static str, value: C) -> Attrs<'a>
where
    C: Into<Cow<'a, str>>,
{
    Attrs(vec![Attr(name, value.into())])
}
