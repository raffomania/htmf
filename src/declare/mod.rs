mod attrs;
mod tags;

use std::borrow::Cow;

pub use attrs::*;
pub use tags::*;

use crate::Element;

pub fn text<'a, C>(value: C) -> Element<'a>
where
    C: Into<Cow<'a, str>>,
{
    Element::Text(value.into())
}

pub fn fragment<'a, C>(value: C) -> Element<'a>
where
    C: Into<Vec<Element<'a>>>,
{
    Element::Fragment {
        children: value.into(),
    }
}
