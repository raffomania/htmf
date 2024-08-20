use std::borrow::Cow;

use crate::{Attr, Element};

pub fn href(target: &str) -> Attr<'_> {
    ("href", target.into())
}

pub fn class(target: &str) -> Attr<'_> {
    ("class", target.into())
}

pub fn text<'a, C>(content: C) -> Element<'a>
where
    C: Into<Cow<'a, str>>,
{
    Element::Text(content.into())
}
