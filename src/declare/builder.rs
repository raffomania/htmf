use crate::Element;

pub struct Builder<'borrowed, 'element> {
    pub element: &'borrowed mut Element<'element>,
}

impl<'borrowed, 'element> Builder<'borrowed, 'element> {
    pub fn add_children<'temp: 'borrowed, C>(self, children: C) -> Self
    where
        C: Into<Vec<Element<'element>>>,
    {
        let mut children = children.into();
        self.element.children_mut().append(&mut children);
        self
    }
}
