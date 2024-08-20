mod attrs;
mod tags;

use std::borrow::Cow;

pub use attrs::*;
pub use tags::*;

use crate::Element;

pub fn text<'a, C>(content: C) -> Element<'a>
where
    C: Into<Cow<'a, str>>,
{
    Element::Text(content.into())
}
