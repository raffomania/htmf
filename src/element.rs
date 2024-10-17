use std::borrow::Cow;

use crate::attr::Attr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PureElement<'e> {
    Tag {
        children: Vec<PureElement<'e>>,
        tag: &'static str,
        attrs: Vec<Attr<'e>>,
    },
    Text {
        text: Cow<'e, str>,
    },
}

impl<'e> PureElement<'e> {
    pub fn to_html(&'e self) -> String {
        match self {
            PureElement::Tag {
                children,
                tag,
                attrs,
            } => {
                let mut children_html = if !children.is_empty() { "\n" } else { "" }.to_string();
                children_html.push_str(
                    &children
                        .iter()
                        .map(|c| c.to_html())
                        .collect::<Vec<_>>()
                        .join("\n"),
                );

                // Indent all children
                children_html = children_html.replace("\n", "\n    ");

                if !children.is_empty() {
                    children_html.push('\n');
                }

                let attrs_space = if !attrs.is_empty() { " " } else { "" };
                let attrs_html: String = attrs
                    .iter()
                    .map(|(k, v)| format!(r#"{k}="{v}""#))
                    .collect::<Vec<_>>()
                    .join(" ");

                format!("<{tag}{attrs_space}{attrs_html}>{children_html}</{tag}>")
            }
            PureElement::Text { text } => text.to_string(),
        }
    }

    pub(crate) fn children_mut(&mut self) -> Option<&mut Vec<PureElement<'e>>> {
        match self {
            PureElement::Tag {
                children,
                tag: _,
                attrs: _,
            } => Some(children),
            PureElement::Text { text: _ } => None,
        }
    }

    pub(crate) fn attrs_mut(&mut self) -> Option<&mut Vec<Attr<'e>>> {
        match self {
            PureElement::Tag {
                children: _,
                tag: _,
                attrs,
            } => Some(attrs),
            PureElement::Text { text: _ } => None,
        }
    }
}

impl<'e> From<Element<'e>> for PureElement<'e> {
    // TODO can we just ignore the parent like this?
    fn from(Element { element, parent: _ }: Element<'e>) -> Self {
        element
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

// TODO rename to cursor?
// TODO split into TagElement, FragmentElement, etc.
impl<'e> Element<'e> {
    pub fn new(tag: &'static str) -> Element<'e> {
        Element {
            element: PureElement::Tag {
                children: Vec::new(),
                tag,
                attrs: Vec::new(),
            },
            parent: Path::Top,
        }
    }

    pub fn with<C>(mut self, new_children: C) -> Element<'e>
    where
        C: Into<Vec<Element<'e>>>,
    {
        let mut new_children = new_children
            .into()
            .into_iter()
            .map(PureElement::from)
            .collect();
        if let Some(children) = self.element.children_mut() {
            children.append(&mut new_children);
        }
        self
    }

    pub fn attr<C>(mut self, name: &'static str, value: C) -> Element<'e>
    where
        C: Into<Cow<'e, str>>,
    {
        if let Some(attrs) = self.element.attrs_mut() {
            attrs.push((name, value.into()));
        }
        self
    }

    // TODO find a better name, wtf
    pub(crate) fn into_new_child(self, new_tag: &'static str) -> Element<'e> {
        if let PureElement::Tag {
            children,
            tag,
            attrs,
        } = self.element
        {
            let parent = Path::Node {
                tag,
                attrs,
                left: children,
                parent: Box::new(self.parent),
                right: Vec::new(),
            };
            Element {
                element: PureElement::Tag {
                    tag: new_tag,
                    attrs: Vec::new(),
                    children: Vec::new(),
                },
                parent,
            }
        } else {
            self
        }
    }

    pub fn up(self) -> Option<Element<'e>> {
        match self.parent {
            Path::Top => None,
            Path::Node {
                left,
                parent,
                mut right,
                tag,
                attrs,
            } => {
                let mut parents_children = left;
                if let PureElement::Tag {
                    mut children,
                    tag: _,
                    attrs: _,
                } = self.element
                {
                    parents_children.append(&mut children);
                }
                parents_children.append(&mut right);

                Some(Element {
                    parent: *parent,
                    element: PureElement::Tag {
                        children: parents_children,
                        tag,
                        attrs,
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
