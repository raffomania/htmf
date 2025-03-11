use crate::element::Element;

/// Used for arguments that accept an arbitrary, ordered collection of elements.
pub trait IntoElements {
    fn into_elements(self) -> Vec<Element>;
}

impl<E> IntoElements for E
where
    E: Into<Element>,
{
    fn into_elements(self) -> Vec<Element> {
        let element: Element = self.into();
        if let Element::Fragment { children } = element {
            return children;
        }

        vec![element]
    }
}

impl<const N: usize, E> IntoElements for [E; N]
where
    E: Into<Element>,
{
    fn into_elements(self) -> Vec<Element> {
        self.into_iter().map(Into::into).collect()
    }
}
