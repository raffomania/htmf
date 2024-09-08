use std::borrow::Cow;

use crate::attr::Attr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PureElement<'e> {
    pub(crate) children: Vec<PureElement<'e>>,
    pub(crate) tag: &'static str,
    pub(crate) attrs: Vec<Attr<'e>>,
    // TODO refactor PureElement to be a proper enum
    pub(crate) text: Option<Cow<'e, str>>,
}

impl<'e> PureElement<'e> {
    pub fn to_html(&'e self) -> String {
        if let Some(text) = &self.text {
            return text.to_string();
        }

        let mut children_html = if !self.children.is_empty() { "\n" } else { "" }.to_string();
        children_html.push_str(
            &self
                .children
                .iter()
                .map(|c| c.to_html())
                .collect::<Vec<_>>()
                .join("\n"),
        );

        // Indent all children
        children_html = children_html.replace("\n", "\n    ");

        if !self.children.is_empty() {
            children_html.push('\n');
        }

        let attrs_space = if !self.attrs.is_empty() { " " } else { "" };
        let attrs_html: String = self
            .attrs
            .iter()
            .map(|(k, v)| format!(r#"{k}="{v}""#))
            .collect::<Vec<_>>()
            .join(" ");

        let tag = self.tag;
        format!("<{tag}{attrs_space}{attrs_html}>{children_html}</{tag}>")
    }
}

impl<'e> From<Element<'e>> for PureElement<'e> {
    fn from(Element { element, parent: _ }: Element<'e>) -> Self {
        PureElement {
            children: element.children,
            tag: element.tag,
            attrs: element.attrs,
            text: None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Path<'e> {
    Top,
    Node {
        tag: &'static str,
        attrs: Vec<Attr<'e>>,
        left: Vec<PureElement<'e>>,
        parent: Box<Path<'e>>,
        right: Vec<PureElement<'e>>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Element<'e> {
    pub(crate) element: PureElement<'e>,
    pub(crate) parent: Path<'e>,
}

impl<'e> Element<'e> {
    pub fn new(tag: &'static str) -> Element<'e> {
        Element {
            element: PureElement {
                children: Vec::new(),
                tag,
                attrs: Vec::new(),
                text: None,
            },
            parent: Path::Top,
        }
    }

    pub fn with<C>(mut self, children: C) -> Element<'e>
    where
        C: Into<Vec<Element<'e>>>,
    {
        let mut children = children.into().into_iter().map(PureElement::from).collect();
        self.element.children.append(&mut children);
        self
    }

    pub fn attr<C>(mut self, name: &'static str, value: C) -> Element<'e>
    where
        C: Into<Cow<'e, str>>,
    {
        self.element.attrs.push((name, value.into()));
        self
    }

    // TODO find a better name, wtf
    pub(crate) fn into_new_child(self, tag: &'static str) -> Element<'e> {
        let parent = Path::Node {
            tag: self.element.tag,
            attrs: self.element.attrs,
            left: self.element.children,
            parent: Box::new(self.parent),
            right: Vec::new(),
        };
        Element {
            element: PureElement {
                tag,
                attrs: Vec::new(),
                children: Vec::new(),
                text: None,
            },
            parent,
        }
    }

    pub fn up(mut self) -> Option<Element<'e>> {
        match self.parent {
            Path::Top => None,
            Path::Node {
                mut left,
                parent,
                mut right,
                tag,
                attrs,
            } => {
                left.append(&mut self.element.children);
                left.append(&mut right);
                let children = left;
                Some(Element {
                    parent: *parent,
                    element: PureElement {
                        children,
                        tag,
                        attrs,
                        text: None,
                    },
                })
            }
        }
    }

    pub fn build(self) -> PureElement<'e> {
        match &self.parent {
            Path::Top => self.element,
            Path::Node {
                left: _,
                parent: _,
                right: _,
                tag: _,
                attrs: _,
            } => self.up().unwrap().build(),
        }
    }

    pub fn to_html(self) -> String {
        let root = self.build();
        root.to_html()
    }
}
