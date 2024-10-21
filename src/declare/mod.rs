mod all_attrs;
mod all_tags;

use std::borrow::Cow;

pub use crate::builder::Builder;
pub use all_attrs::*;
pub use all_tags::*;

use crate::element::{Element, Path};

pub fn text<'e, C>(value: C) -> Builder<'e>
where
    C: Into<Cow<'e, str>>,
{
    Builder {
        element: Element::Text { text: value.into() },
        parent: Path::Top,
    }
}

impl<'e> Builder<'e> {
    pub fn text<C>(self, value: C) -> Builder<'e>
    where
        C: Into<Cow<'e, str>>,
    {
        self.into_new_child_element(Element::Text { text: value.into() })
    }
}

pub fn fragment<'e>() -> Builder<'e> {
    Builder {
        element: Element::Fragment {
            children: Vec::new(),
        },
        parent: Path::Top,
    }
}

/// Prepend `<!doctype html>` to the given children.
pub fn document<'e>() -> Builder<'e> {
    Builder {
        element: Element::Document {
            children: Vec::new(),
        },
        parent: Path::Top,
    }
}
