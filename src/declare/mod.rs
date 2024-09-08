mod all_attrs;
mod all_tags;

use std::borrow::Cow;

pub use crate::element::Element;
pub use all_attrs::*;
pub use all_tags::*;

use crate::element::{Path, PureElement};

pub fn text<'e, C>(value: C) -> Element<'e>
where
    C: Into<Cow<'e, str>>,
{
    Element {
        element: PureElement::Text { text: value.into() },
        parent: Path::Top,
    }
}

impl<'e> Element<'e> {
    pub fn text<C>(self, value: C) -> Element<'e>
    where
        C: Into<Cow<'e, str>>,
    {
        self.into_new_child_element(PureElement::Text { text: value.into() })
    }
}

pub fn fragment<'e>() -> Element<'e> {
    Element {
        element: PureElement::Fragment {
            children: Vec::new(),
        },
        parent: Path::Top,
    }
}

/// Prepend `<!doctype html>` to the given children.
pub fn document<'e>() -> Element<'e> {
    Element {
        element: PureElement::Document {
            children: Vec::new(),
        },
        parent: Path::Top,
    }
}
