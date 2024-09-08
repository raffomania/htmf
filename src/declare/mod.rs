mod all_attrs;
mod all_tags;

use std::borrow::Cow;

pub use all_attrs::*;
pub use all_tags::*;

use crate::element::{Element, Path, PureElement};

pub fn text<'a, C>(value: C) -> Element<'a>
where
    C: Into<Cow<'a, str>>,
{
    Element {
        element: PureElement {
            children: Vec::new(),
            tag: "",
            attrs: Vec::new(),
            text: Some(value.into()),
        },
        parent: Path::Top,
    }
}

// TODO re-add this
// pub fn fragment<'a, C>(value: C) -> Element<'a>
// where
//     C: Into<Vec<Element<'a>>>,
// {
//     Element::Fragment {
//         children: value.into(),
//     }
// }

// TODO re-add this
// Prepend `<!doctype html>` to the given children.
// pub fn document<'a>() -> Element<'a> {
// }
