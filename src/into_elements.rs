use crate::element::Element;

/// Used for arguments that accept an arbitrary, ordered collection of elements.
pub trait IntoElements {
    fn into_elements(self) -> Vec<Element>;
}

impl IntoElements for Element {
    fn into_elements(self) -> Vec<Element> {
        vec![self]
    }
}

impl IntoElements for String {
    fn into_elements(self) -> Vec<Element> {
        vec![crate::declare::text(self)]
    }
}

impl IntoElements for &str {
    fn into_elements(self) -> Vec<Element> {
        vec![crate::declare::text(self)]
    }
}

impl IntoElements for &String {
    fn into_elements(self) -> Vec<Element> {
        vec![crate::declare::text(self)]
    }
}

impl IntoElements for Vec<Element> {
    fn into_elements(self) -> Vec<Element> {
        self
    }
}

impl<const N: usize> IntoElements for [Element; N] {
    fn into_elements(self) -> Vec<Element> {
        self.into_iter().collect()
    }
}
