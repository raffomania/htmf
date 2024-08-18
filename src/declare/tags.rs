use std::borrow::Cow;

use crate::Element;

pub fn a<'a, const A: usize, S, C>(attrs: [(&'static str, S); A], children: C) -> Element<'a>
where
    S: Into<Cow<'a, str>>,
    C: Into<Vec<Element<'a>>>,
{
    Element::Tag {
        tag: "a".to_string(),
        attrs: attrs.into_iter().map(|(k, v)| (k, v.into())).collect(),
        children: children.into(),
    }
}
