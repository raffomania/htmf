use crate::element::Element;

/// Used for arguments that accept an arbitrary, ordered collection of elements.
pub trait IntoElements<'a> {
    fn into_elements(self) -> Vec<Element<'a>>;
}

impl<'a> IntoElements<'a> for Element<'a> {
    fn into_elements(self) -> Vec<Element<'a>> {
        vec![self]
    }
}

impl<'a> IntoElements<'a> for Vec<Element<'a>> {
    fn into_elements(self) -> Vec<Element<'a>> {
        self
    }
}

impl<'a, const N: usize> IntoElements<'a> for [Element<'a>; N] {
    fn into_elements(self) -> Vec<Element<'a>> {
        self.into_iter().collect()
    }
}
