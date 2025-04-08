mod all_tags;

pub use super::declare::all_attrs::*;
pub use all_tags::*;

use crate::{
    attr::{Attr, Attrs},
    element::Element,
    into_elements::IntoElements,
};

pub fn text<C>(value: C) -> Element
where
    C: Into<String>,
{
    Element::Text { text: value.into() }
}

/// Prepend `<!doctype html>` to the given children.
pub fn document<Children: IntoElements>(children: Children) -> Element {
    Element::Document {
        children: children.into_elements(),
    }
}

pub fn fragment<Children: IntoElements>(children: Children) -> Element {
    Element::Fragment {
        children: children.into_elements(),
    }
}

pub fn nothing() -> Element {
    Element::Nothing
}

pub fn attr<C>(name: &'static str, value: C) -> Attrs
where
    C: Into<String>,
{
    Attrs(vec![Attr(name, value.into())])
}
