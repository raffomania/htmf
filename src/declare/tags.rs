use crate::{Attr, Element};

pub fn a<'a, A, C>(attrs: A, children: C) -> Element<'a>
where
    A: Into<Vec<Attr<'a>>>,
    C: Into<Vec<Element<'a>>>,
{
    Element::Tag {
        tag: "a",
        attrs: attrs.into(),
        children: children.into(),
    }
}
