use std::borrow::Cow;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Attr<'a>(pub(crate) &'static str, pub(crate) Cow<'a, str>);

pub trait IntoAttrs<'a> {
    fn into_attrs(self) -> Vec<Attr<'a>>;
}

impl<'a, Iter> IntoAttrs<'a> for Iter
where
    Iter: IntoIterator<Item = Attr<'a>>,
{
    fn into_attrs(self) -> Vec<Attr<'a>> {
        self.into_iter().collect()
    }
}

impl<'a> IntoAttrs<'a> for Attr<'a> {
    fn into_attrs(self) -> Vec<Attr<'a>> {
        vec![self]
    }
}
