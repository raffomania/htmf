use crate::{
    attr::{Attr, Attrs},
    into_elements::IntoElements,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Element {
    Tag {
        children: Vec<Element>,
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
}

impl Element {
    pub fn to_html(&self) -> String {
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

    fn children_html(children: &[Element], indent: bool) -> String {
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
        }
    }
}
