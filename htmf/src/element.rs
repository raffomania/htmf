use std::fmt::{Debug, Write};

use crate::{
    attr::{Attr, Attrs},
    declare, escape,
    into_elements::IntoElements,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Element {
    Tag {
        children: Vec<Element>,
        tag: &'static str,
        attrs: Attrs,
    },
    LeafTag {
        tag: &'static str,
        attrs: Attrs,
    },
    Fragment {
        children: Vec<Element>,
    },
    Document {
        children: Vec<Element>,
    },
    Text {
        text: String,
    },
    Nothing,
}

impl Element {
    pub fn to_html(&self) -> String {
        self.to_string()
    }

    #[cfg(feature = "pretty-print")]
    pub fn to_html_pretty(
        &self,
    ) -> Result<String, markup_fmt::FormatError<std::convert::Infallible>> {
        let raw = self.to_html();
        markup_fmt::format_text(
            &raw,
            markup_fmt::Language::Html,
            &markup_fmt::config::FormatOptions::default(),
            |code, _| Ok::<_, std::convert::Infallible>(code.into()),
        )
        .inspect_err(|e| {
            println!("{raw}");
            println!("{e}");
        })
    }

    pub fn with<C>(mut self, new_children: C) -> Self
    where
        C: IntoElements,
    {
        let mut new_children = new_children.into_elements();
        if let Some(children) = self.children_mut() {
            children.append(&mut new_children);
        }
        self
    }

    pub fn attr<C>(mut self, name: &'static str, value: C) -> Self
    where
        C: Into<String>,
    {
        if let Some(attrs) = self.attrs_mut() {
            attrs.push(Attr(name, value.into()));
        }
        self
    }

    pub fn write_html(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        match self {
            Element::Tag {
                children,
                tag,
                attrs,
            } => {
                f.write_char('<')?;
                escape::write_escaped_html(f, tag);

                if !attrs.0.is_empty() {
                    f.write_char(' ')?
                };

                std::fmt::Display::fmt(attrs, f)?;

                f.write_char('>')?;

                Self::write_children_html(f, children, indent + 1)?;

                f.write_char('<')?;
                f.write_char('/')?;
                escape::write_escaped_html(f, tag);
                f.write_char('>')?;
            }
            Element::LeafTag { tag, attrs } => {
                f.write_char('<')?;
                escape::write_escaped_html(f, tag);

                if !attrs.0.is_empty() {
                    f.write_char(' ')?
                };

                std::fmt::Display::fmt(attrs, f)?;

                f.write_char('/')?;
                f.write_char('>')?;
            }
            Element::Fragment { children } => {
                Self::write_children_html(f, children, indent)?;
            }
            Element::Text { text } => {
                escape::write_escaped_html(f, text);
            }
            Element::Document { children } => {
                f.write_str("<!doctype html>")?;
                Self::write_children_html(f, children, indent)?;
            }
            Element::Nothing => {}
        };

        Ok(())
    }

    fn write_children_html(
        f: &mut std::fmt::Formatter<'_>,
        children_with_empty: &[Element],
        indent: usize,
    ) -> std::fmt::Result {
        let children = children_with_empty
            .iter()
            .filter(|c| !matches!(c, Element::Nothing));

        for child in children {
            child.write_html(f, indent)?;
        }

        Ok(())
    }

    pub(crate) fn children_mut(&mut self) -> Option<&mut Vec<Element>> {
        match self {
            Element::Tag {
                children,
                tag: _,
                attrs: _,
            } => Some(children),
            Element::Text { text: _ } => None,
            Element::Fragment { children } => Some(children),
            Element::Document { children } => Some(children),
            Element::Nothing => None,
            Element::LeafTag { .. } => None,
        }
    }

    pub(crate) fn attrs_mut(&mut self) -> Option<&mut Vec<Attr>> {
        match self {
            Element::Tag {
                children: _,
                tag: _,
                attrs,
            } => Some(&mut attrs.0),
            Element::Text { text: _ } => None,
            Element::Fragment { children: _ } => None,
            Element::Document { children: _ } => None,
            Element::Nothing => None,
            Element::LeafTag { attrs, .. } => Some(&mut attrs.0),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_html(f, 0)
    }
}

impl From<()> for Element {
    fn from(_value: ()) -> Self {
        Element::Nothing
    }
}

impl From<String> for Element {
    fn from(value: String) -> Self {
        declare::text(value)
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        declare::text(value)
    }
}

impl From<&String> for Element {
    fn from(value: &String) -> Self {
        declare::text(value)
    }
}

impl<E> From<Vec<E>> for Element
where
    E: Into<Element>,
{
    fn from(value: Vec<E>) -> Self {
        // Don't call `with()` here to avoid infinitely nested fragments
        Element::Fragment {
            children: value.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<Option<Element>> for Element {
    fn from(value: Option<Element>) -> Self {
        value.unwrap_or(Element::Nothing)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn nothing_element() {
        assert_eq!(nothing().to_html(), "");
        let doc = body([]).with(nothing());
        assert_eq!(doc.to_html(), body([]).to_html());
    }
}
