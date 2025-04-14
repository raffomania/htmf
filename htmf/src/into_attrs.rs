use crate::attr::{Attr, Attrs};

pub trait IntoAttrs {
    fn into_attrs(self) -> Attrs;
}

impl IntoAttrs for Attr {
    fn into_attrs(self) -> Attrs {
        Attrs(vec![self])
    }
}

impl IntoAttrs for () {
    fn into_attrs(self) -> Attrs {
        Attrs(Vec::new())
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

impl IntoAttrs for Option<Attrs> {
    fn into_attrs(self) -> Attrs {
        Attrs(self.into_iter().flat_map(|attrs| attrs.0).collect())
    }
}

impl IntoAttrs for Vec<Attrs> {
    fn into_attrs(self) -> Attrs {
        Attrs(self.into_iter().flat_map(|attrs| attrs.0).collect())
    }
}
