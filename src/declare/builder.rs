use crate::Element;

type AccessFn<'element> =
    Box<dyn for<'temp> FnMut(&'temp mut Element<'element>) -> &'temp mut Element<'element>>;

pub(crate) struct Accessor<'element> {
    pub(crate) parent: Option<Box<Accessor<'element>>>,
    pub(crate) this: AccessFn<'element>,
}

impl<'element> Accessor<'element> {
    pub(crate) fn get_mut<'moretemp, 'temp>(
        &'moretemp mut self,
        root: &'temp mut Element<'element>,
    ) -> &'temp mut Element<'element> {
        match &mut self.parent {
            Some(parent) => (self.this)(parent.get_mut(root)),
            None => (self.this)(root),
        }
    }

    pub(crate) fn compose(self, other: AccessFn<'element>) -> Accessor<'element> {
        Accessor {
            parent: Some(Box::new(self)),
            this: other,
        }
    }
}

pub struct Builder<'element> {
    pub(crate) element: Element<'element>,
    pub(crate) accessor: Accessor<'element>,
}

impl<'element> Builder<'element> {
    pub fn with<C>(mut self, children: C) -> Self
    where
        C: Into<Vec<Element<'element>>>,
    {
        let mut children = children.into();
        self.get_cursor_mut().children_mut().append(&mut children);
        self
    }

    pub(crate) fn get_cursor_mut(&mut self) -> &mut Element<'element> {
        self.accessor.get_mut(&mut self.element)
    }
}

impl<'element> From<Builder<'element>> for Element<'element> {
    fn from(value: Builder<'element>) -> Self {
        value.element
    }
}
