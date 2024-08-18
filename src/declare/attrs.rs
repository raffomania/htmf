use std::borrow::Cow;

use crate::Element;

pub fn href(target: &str) -> (&'static str, &str) {
    ("href", target)
}

pub fn class(target: &str) -> (&'static str, &str) {
    ("class", target)
}

pub fn text<'a, C>(content: C) -> Element<'a>
where
    C: Into<Cow<'a, str>>,
{
    Element::Text(content.into())
}
