use crate::Element;

pub struct Builder<'element> {
    pub element: Element<'element>,
    pub parent: Option<Box<Builder<'element>>>,
}

impl<'element> Builder<'element> {
    pub fn with<C>(mut self, children: C) -> Self
    where
        C: Into<Vec<Element<'element>>>,
    {
        let mut children = children.into();
        self.element.children_mut().append(&mut children);
        self
    }

    pub fn build(self) -> Element<'element> {
        match self.parent {
            Some(parent) => parent.build_from_child(self.element),
            None => self.element,
        }
    }

    fn build_from_child(mut self, child: Element<'element>) -> Element<'element> {
        self.element = self.element.with([child]);
        match self.parent {
            Some(parent) => parent.build_from_child(self.element),
            None => self.element,
        }
    }
}

impl<'element> From<Builder<'element>> for Element<'element> {
    fn from(value: Builder<'element>) -> Self {
        value.build()
    }
}
