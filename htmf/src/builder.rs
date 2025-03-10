use crate::{attr::Attrs, element::Element, into_elements::IntoElements};

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Path {
    Top,
    Tag {
        tag: &'static str,
        attrs: Attrs,
        left: Vec<Element>,
        parent: Box<Path>,
        right: Vec<Element>,
    },
    LeafTag {
        tag: &'static str,
        attrs: Attrs,
        parent: Box<Path>,
    },
    Fragment {
        left: Vec<Element>,
        parent: Box<Path>,
        right: Vec<Element>,
    },
    Document {
        left: Vec<Element>,
        parent: Box<Path>,
        right: Vec<Element>,
    },
}

// TODO: it's annoying to switch all signatures from Element to Builder when turning on the `unstable-builder` flag.
// Make the `parent` field conditional instead?
// If we use `Builder` as the "main" public
// type, rename it to `Element` and rename `Element` to something else?
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Builder {
    pub(crate) element: Element,
    pub(crate) parent: Path,
}

impl Builder {
    pub fn with<C>(mut self, new_children: C) -> Builder
    where
        C: IntoElements,
    {
        self.element = self.element.with(new_children);
        self
    }

    pub fn attr<C>(mut self, name: &'static str, value: C) -> Builder
    where
        C: Into<String>,
    {
        self.element = self.element.attr(name, value);
        self
    }

    pub(crate) fn into_new_child_tag(self, new_tag: &'static str, attrs: Attrs) -> Builder {
        let new_element = Element::Tag {
            tag: new_tag,
            attrs,
            children: Vec::new(),
        };
        self.into_new_child_element(new_element)
    }

    pub(crate) fn into_new_child_element(self, element: Element) -> Builder {
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
            Element::LeafTag { tag, attrs } => Path::LeafTag {
                tag,
                attrs,
                parent: Box::new(self.parent),
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
            Element::Nothing => return self,
        };

        Builder { element, parent }
    }

    pub fn up(self) -> Option<Builder> {
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
            Path::LeafTag { tag, attrs, parent } => Some(Builder {
                parent: *parent,
                element: Element::LeafTag { tag, attrs },
            }),
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

    pub(crate) fn into_root_element(self) -> Element {
        match &self.parent {
            Path::Top => self.element,
            Path::Tag { .. }
            | Path::LeafTag { .. }
            | Path::Document { .. }
            | Path::Fragment { .. } => self.up().unwrap().into_root_element(),
        }
    }

    pub fn to_html(self) -> String {
        let root = self.into_root_element();
        root.to_html()
    }
}

// -- Trait impls

impl From<Builder> for Element {
    fn from(element: Builder) -> Self {
        element.into_root_element()
    }
}

impl From<Element> for Builder {
    fn from(element: Element) -> Self {
        Builder {
            element,
            parent: Path::Top,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn base() {
        let doc = document().with([html(class("w-full h-full")).with([
            head([]).with([meta([name("color-scheme"), content("dark")])]),
            body(class("w-full h-full text-gray-200 bg-neutral-800")).with(
                main_(class("sm:overflow-y-auto sm:grow"))
                    .with([p([]).with("My cool content"), aside(id("nav"))]),
            ),
        ])]);
        let html = doc.clone().to_html();
        use pretty_assertions::assert_eq;

        let doc_builder = document()
            .html(class("w-full h-full"))
            .with([head([]).with([meta([name("color-scheme"), content("dark")])])])
            .body(class("w-full h-full text-gray-200 bg-neutral-800"))
            .main_(class("sm:overflow-y-auto sm:grow"))
            .with([p([]).with("My cool content"), aside(id("nav"))])
            .into_root_element();
        let builder_html = doc_builder.to_html();
        assert_eq!(html, builder_html);
        assert_eq!(doc, doc_builder);
    }
}
