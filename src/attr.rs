use std::borrow::Cow;

pub type Attr<'a> = (&'static str, Cow<'a, str>);
