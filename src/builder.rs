use std::borrow::Cow;

use crate::{attr::Attrs, element::Element, into_elements::IntoElements};

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

// TODO: it's annoying to switch all signatures from Element to Builder when turning on the `unstable-builder` flag.
// Make the `parent` field conditional instead?
// If we use `Builder` as the "main" public
// type, rename it to `Element` and rename `Element` to something else?
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Builder<'e> {
    pub(crate) element: Element<'e>,
    pub(crate) parent: Path<'e>,
}

impl<'e> Builder<'e> {
    pub fn new_tag(tag: &'static str, attrs: Attrs<'e>) -> Builder<'e> {
        Builder {
            element: Element::Tag {
                children: Vec::new(),
                tag,
                attrs,
            },
            parent: Path::Top,
        }
    }

    pub fn with<C>(mut self, new_children: C) -> Builder<'e>
    where
        C: IntoElements<'e>,
    {
        self.element = self.element.with(new_children);
        self
    }

    pub fn attr<C>(mut self, name: &'static str, value: C) -> Builder<'e>
    where
        C: Into<Cow<'e, str>>,
    {
        self.element = self.element.attr(name, value);
        self
    }

    pub(crate) fn into_new_child_tag(self, new_tag: &'static str, attrs: Attrs<'e>) -> Builder<'e> {
        let new_element = Element::Tag {
            tag: new_tag,
            attrs,
            children: Vec::new(),
        };
        self.into_new_child_element(new_element)
    }

    pub(crate) fn into_new_child_element(self, element: Element<'e>) -> Builder<'e> {
        let parent = match self.element {
            Element::Tag {
                children,
                tag,
                attrs,
            } => Path::Tag {
                tag,
                attrs,
                left: children,
                parent: Box::new(self.parent),
                right: Vec::new(),
            },
            Element::Fragment { children } => Path::Fragment {
                left: children,
                parent: Box::new(self.parent),
                right: Vec::new(),
            },
            Element::Document { children } => Path::Document {
                left: children,
                parent: Box::new(self.parent),
                right: Vec::new(),
            },
            Element::Text { .. } => return self,
        };

        Builder { element, parent }
    }

    pub fn up(self) -> Option<Builder<'e>> {
        match self.parent {
            Path::Top => None,
            Path::Tag {
                left,
                parent,
                mut right,
                tag,
                attrs,
            } => {
                let mut parents_children = left;
                parents_children.push(self.element);
                parents_children.append(&mut right);

                Some(Builder {
                    parent: *parent,
                    element: Element::Tag {
                        children: parents_children,
                        tag,
                        attrs,
                    },
                })
            }
            Path::Fragment {
                left,
                parent,
                mut right,
            } => {
                let mut parents_children = left;
                parents_children.push(self.element);
                parents_children.append(&mut right);

                Some(Builder {
                    parent: *parent,
                    element: Element::Fragment {
                        children: parents_children,
                    },
                })
            }
            Path::Document {
                left,
                parent,
                mut right,
            } => {
                let mut parents_children = left;
                parents_children.push(self.element);
                parents_children.append(&mut right);

                Some(Builder {
                    parent: *parent,
                    element: Element::Document {
                        children: parents_children,
                    },
                })
            }
        }
    }

    pub(crate) fn into_root_element(self) -> Element<'e> {
        match &self.parent {
            Path::Top => self.element,
            Path::Tag { .. } | Path::Document { .. } | Path::Fragment { .. } => {
                self.up().unwrap().into_root_element()
            }
        }
    }

    pub fn to_html(self) -> String {
        let root = self.into_root_element();
        root.to_html()
    }
}

// -- Trait impls

impl<'e> From<Builder<'e>> for Element<'e> {
    fn from(element: Builder<'e>) -> Self {
        element.into_root_element()
    }
}

impl<'e> From<Element<'e>> for Builder<'e> {
    fn from(element: Element<'e>) -> Self {
        Builder {
            element,
            parent: Path::Top,
        }
    }
}

impl<'a> IntoElements<'a> for Builder<'a> {
    fn into_elements(self) -> Vec<Element<'a>> {
        vec![self.into()]
    }
}

impl<'a> IntoElements<'a> for Vec<Builder<'a>> {
    fn into_elements(self) -> Vec<Element<'a>> {
        self.into_iter().map(Element::from).collect()
    }
}

impl<'a, const N: usize> IntoElements<'a> for [Builder<'a>; N] {
    fn into_elements(self) -> Vec<Element<'a>> {
        self.into_iter().map(Element::from).collect()
    }
}
