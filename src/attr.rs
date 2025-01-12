use std::fmt::Write;

use crate::escape;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Attr(pub(crate) &'static str, pub(crate) String);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Attrs(pub(crate) Vec<Attr>);

impl Attrs {
    pub fn attr<C>(mut self, name: &'static str, value: C) -> Attrs
    where
        C: Into<String>,
    {
        self.0.push(Attr(name, value.into()));
        self
    }
}

impl std::fmt::Display for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, Attr(k, v)) in self.0.iter().enumerate() {
            escape::write_escaped_html(f, k);
            f.write_char('=')?;
            f.write_char('"')?;
            escape::write_escaped_html(f, v);
            f.write_char('"')?;

            if i < self.0.len() - 1 {
                f.write_char(' ')?;
            }
        }

        std::fmt::Result::Ok(())
    }
}

pub trait IntoAttrs {
    fn into_attrs(self) -> Attrs;
}

impl IntoAttrs for Attr {
    fn into_attrs(self) -> Attrs {
        Attrs(vec![self])
    }
}

impl IntoAttrs for Attrs {
    fn into_attrs(self) -> Attrs {
        self
    }
}

impl<const N: usize> IntoAttrs for [Attrs; N] {
    fn into_attrs(self) -> Attrs {
        Attrs(self.into_iter().flat_map(|attrs| attrs.0).collect())
    }
}
