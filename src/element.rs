use std::fmt::{Debug, Write};

use crate::{
    attr::{Attr, Attrs},
    declare, escape,
    into_elements::IntoElements,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Element {
    Tag {
        children: Vec<Element>,
        tag: &'static str,
        attrs: Attrs,
    },
    Fragment {
        children: Vec<Element>,
    },
    Document {
        children: Vec<Element>,
    },
    Text {
        text: String,
    },
    Nothing,
}

impl Element {
    pub fn to_html(&self) -> String {
        self.to_string()
    }

    pub fn with<C>(mut self, new_children: C) -> Self
    where
        C: IntoElements,
    {
        let mut new_children = new_children.into_elements();
        if let Some(children) = self.children_mut() {
            children.append(&mut new_children);
        }
        self
    }

    pub fn attr<C>(mut self, name: &'static str, value: C) -> Self
    where
        C: Into<String>,
    {
        if let Some(attrs) = self.attrs_mut() {
            attrs.push(Attr(name, value.into()));
        }
        self
    }

    pub fn write_html(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        enum WriteTask<'e> {
            VisitNode(&'e Element),
            WriteClosingTag(&'e str),
            WriteNewline,
        }

        let mut stack = Vec::new();

        stack.push((WriteTask::VisitNode(self), 0));

        while let Some((task, indent)) = stack.pop() {
            let element = match task {
                WriteTask::VisitNode(element) => element,
                WriteTask::WriteNewline => {
                    f.write_char('\n')?;
                    f.write_str(&" ".repeat(indent * 4))?;
                    continue;
                }
                WriteTask::WriteClosingTag(tag) => {
                    f.write_char('<')?;
                    f.write_char('/')?;
                    escape::write_escaped_html(f, tag);
                    f.write_char('>')?;
                    continue;
                }
            };

            match element {
                Element::Tag {
                    children,
                    tag,
                    attrs,
                } => {
                    f.write_char('\n')?;
                    f.write_str(&" ".repeat(indent * 4))?;
                    f.write_char('<')?;
                    escape::write_escaped_html(f, tag);

                    if !attrs.0.is_empty() {
                        f.write_char(' ')?;
                    };

                    std::fmt::Display::fmt(attrs, f)?;

                    f.write_char('>')?;

                    let children = children
                        .iter()
                        .filter(|c| !matches!(c, Element::Nothing))
                        .rev();
                    stack.push((WriteTask::WriteClosingTag(tag), 0));
                    let mut ending_newline_written = false;
                    for child in children {
                        if !ending_newline_written {
                            stack.push((WriteTask::WriteNewline, indent));
                            ending_newline_written = true;
                        }
                        stack.push((WriteTask::VisitNode(child), indent + 1));
                    }
                }
                Element::Fragment { children } => {
                    for child in children.iter().rev() {
                        stack.push((WriteTask::VisitNode(child), indent));
                    }
                }
                Element::Text { text } => {
                    f.write_char('\n')?;
                    f.write_str(&" ".repeat(indent * 4))?;
                    escape::write_escaped_html(f, text);
                }
                Element::Document { children } => {
                    f.write_str("<!doctype html>\n")?;
                    for child in children.iter().rev() {
                        stack.push((WriteTask::VisitNode(child), indent));
                    }
                }
                Element::Nothing => {}
            };
        }

        std::fmt::Result::Ok(())
    }

    pub(crate) fn children_mut(&mut self) -> Option<&mut Vec<Element>> {
        match self {
            Element::Tag {
                children,
                tag: _,
                attrs: _,
            } => Some(children),
            Element::Text { text: _ } => None,
            Element::Fragment { children } => Some(children),
            Element::Document { children } => Some(children),
            Element::Nothing => None,
        }
    }

    pub(crate) fn attrs_mut(&mut self) -> Option<&mut Vec<Attr>> {
        match self {
            Element::Tag {
                children: _,
                tag: _,
                attrs,
            } => Some(&mut attrs.0),
            Element::Text { text: _ } => None,
            Element::Fragment { children: _ } => None,
            Element::Document { children: _ } => None,
            Element::Nothing => None,
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_html(f)
    }
}

impl From<()> for Element {
    fn from(_value: ()) -> Self {
        Element::Nothing
    }
}

impl From<String> for Element {
    fn from(value: String) -> Self {
        declare::text(value)
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        declare::text(value)
    }
}

impl From<&String> for Element {
    fn from(value: &String) -> Self {
        declare::text(value)
    }
}

impl From<Option<Element>> for Element {
    fn from(value: Option<Element>) -> Self {
        value.unwrap_or(Element::Nothing)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn nothing_element() {
        assert_eq!(nothing().to_html(), "");
        let doc = body([]).with(nothing());
        assert_eq!(doc.to_html(), body([]).to_html());
    }
}
