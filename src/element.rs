use std::borrow::Cow;

use crate::{
    attr::{Attr, Attrs},
    builder::Builder,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Element<'e> {
    Tag {
        children: Vec<Element<'e>>,
        tag: &'static str,
        attrs: Attrs<'e>,
    },
    Fragment {
        children: Vec<Element<'e>>,
    },
    Document {
        children: Vec<Element<'e>>,
    },
    Text {
        text: Cow<'e, str>,
    },
}

impl<'e> Element<'e> {
    pub fn to_html(&'e self) -> String {
        match self {
            Element::Tag {
                children,
                tag,
                attrs,
            } => {
                let attrs_space = if !attrs.0.is_empty() { " " } else { "" };
                let attrs_html: String = attrs
                    .0
                    .iter()
                    .map(|Attr(k, v)| format!(r#"{k}="{v}""#))
                    .collect::<Vec<_>>()
                    .join(" ");

                let children_html = Self::children_html(children, true);

                format!("<{tag}{attrs_space}{attrs_html}>{children_html}</{tag}>")
            }
            Element::Fragment { children } => Self::children_html(children, false),
            Element::Text { text } => text.to_string(),
            Element::Document { children } => {
                let children_html = Self::children_html(children, false);
                format!("<!doctype html>\n{children_html}")
            }
        }
    }

    fn children_html(children: &[Element<'e>], indent: bool) -> String {
        let mut children_html = if !children.is_empty() { "\n" } else { "" }.to_string();
        children_html.push_str(
            &children
                .iter()
                .map(|c| c.to_html())
                .collect::<Vec<_>>()
                .join("\n"),
        );

        if indent {
            // Indent all children
            children_html = children_html.replace("\n", "\n    ");
        }

        if !children.is_empty() {
            children_html.push('\n');
        }

        children_html
    }

    pub(crate) fn children_mut(&mut self) -> Option<&mut Vec<Element<'e>>> {
        match self {
            Element::Tag {
                children,
                tag: _,
                attrs: _,
            } => Some(children),
            Element::Text { text: _ } => None,
            Element::Fragment { children } => Some(children),
            Element::Document { children } => Some(children),
        }
    }

    pub(crate) fn attrs_mut(&mut self) -> Option<&mut Vec<Attr<'e>>> {
        match self {
            Element::Tag {
                children: _,
                tag: _,
                attrs,
            } => Some(&mut attrs.0),
            Element::Text { text: _ } => None,
            Element::Fragment { children: _ } => None,
            Element::Document { children: _ } => None,
        }
    }
}

impl<'e> From<Builder<'e>> for Element<'e> {
    fn from(element: Builder<'e>) -> Self {
        element.into_root_element()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Path<'e> {
    Top,
    Tag {
        tag: &'static str,
        attrs: Attrs<'e>,
        left: Vec<Element<'e>>,
        parent: Box<Path<'e>>,
        right: Vec<Element<'e>>,
    },
    Fragment {
        left: Vec<Element<'e>>,
        parent: Box<Path<'e>>,
        right: Vec<Element<'e>>,
    },
    Document {
        left: Vec<Element<'e>>,
        parent: Box<Path<'e>>,
        right: Vec<Element<'e>>,
    },
}
