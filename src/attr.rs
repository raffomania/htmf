use std::borrow::Cow;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Attr<'a>(pub(crate) &'static str, pub(crate) Cow<'a, str>);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Attrs<'a>(pub(crate) Vec<Attr<'a>>);

impl<'a> Attrs<'a> {
    pub fn attr<C>(mut self, name: &'static str, value: C) -> Attrs<'a>
    where
        C: Into<Cow<'a, str>>,
    {
        self.0.push(Attr(name, value.into()));
        self
    }
}

pub trait IntoAttrs<'a> {
    fn into_attrs(self) -> Attrs<'a>;
}

impl<'a> IntoAttrs<'a> for Attr<'a> {
    fn into_attrs(self) -> Attrs<'a> {
        Attrs(vec![self])
    }
}

impl<'a> IntoAttrs<'a> for Attrs<'a> {
    fn into_attrs(self) -> Attrs<'a> {
        self
    }
}

impl<'a, const N: usize> IntoAttrs<'a> for [Attrs<'a>; N] {
    fn into_attrs(self) -> Attrs<'a> {
        Attrs(self.into_iter().flat_map(|attrs| attrs.0).collect())
    }
}
