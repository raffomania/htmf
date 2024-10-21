use std::borrow::Cow;

use crate::{
    attr::Attr,
    element::{Element, Path},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Builder<'e> {
    pub(crate) element: Element<'e>,
    pub(crate) parent: Path<'e>,
}

impl<'e> Builder<'e> {
    pub fn new_tag(tag: &'static str) -> Builder<'e> {
        Builder {
            element: Element::Tag {
                children: Vec::new(),
                tag,
                attrs: Vec::new(),
            },
            parent: Path::Top,
        }
    }

    pub fn with<C>(mut self, new_children: C) -> Builder<'e>
    where
        C: Into<Vec<Builder<'e>>>,
    {
        let mut new_children = new_children.into().into_iter().map(Element::from).collect();
        if let Some(children) = self.element.children_mut() {
            children.append(&mut new_children);
        }
        self
    }

    pub fn attr<C>(mut self, name: &'static str, value: C) -> Builder<'e>
    where
        C: Into<Cow<'e, str>>,
    {
        if let Some(attrs) = self.element.attrs_mut() {
            attrs.push(Attr(name, value.into()));
        }
        self
    }

    pub(crate) fn into_new_child_tag(self, new_tag: &'static str) -> Builder<'e> {
        let new_element = Element::Tag {
            tag: new_tag,
            attrs: Vec::new(),
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

    pub fn into_root_element(self) -> Element<'e> {
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
