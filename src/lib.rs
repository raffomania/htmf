use std::borrow::Cow;

pub mod declare;

type Attr<'a> = (&'static str, Cow<'a, str>);

#[derive(PartialEq, Eq, Debug)]
pub enum Element<'a> {
    Tag {
        tag: &'static str,
        attrs: Vec<Attr<'a>>,
        children: Vec<Element<'a>>,
    },
    Fragment {
        children: Vec<Element<'a>>,
    },
    Text(Cow<'a, str>),
    Document {
        children: Vec<Element<'a>>,
    },
}

impl<'a> Element<'a> {
    pub fn with<C>(mut self, children: C) -> Element<'a>
    where
        C: Into<Vec<Element<'a>>>,
    {
        let mut children = children.into();
        self.children_mut().append(&mut children);
        self
    }

    pub fn to_html(&'a self) -> String {
        match self {
            Element::Tag {
                tag,
                attrs,
                children,
            } => {
                let mut children_html = if !children.is_empty() { "\n" } else { "" }.to_string();
                children_html.push_str(
                    &children
                        .iter()
                        .map(|c| c.to_html())
                        .collect::<Vec<_>>()
                        .join("\n"),
                );

                // Indent all children
                children_html = children_html.replace("\n", "\n    ");

                if !children.is_empty() {
                    children_html.push('\n');
                }

                let attrs_space = if !attrs.is_empty() { " " } else { "" };
                let attrs_html: String = attrs
                    .iter()
                    .map(|(k, v)| format!(r#"{k}="{v}""#))
                    .collect::<Vec<_>>()
                    .join(" ");

                format!("<{tag}{attrs_space}{attrs_html}>{children_html}</{tag}>")
            }
            Element::Text(t) => t.to_string(),
            Element::Document { children } => {
                let children_html = children
                    .iter()
                    .map(|c| c.to_html())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("<!doctype html>\n{children_html}")
            }
            Element::Fragment { children } => children
                .iter()
                .map(|c| c.to_html())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use declare::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn base() {
        let doc_immutable = document().with([html().class("w-full h-full").with([
            head().with([
                link().rel("stylesheet").href("/assets/preflight.css"),
                link().rel("stylesheet").href("/assets/railwind.css"),
                script().src("/assets/htmx.1.9.9.js"),
                meta().name("color-scheme").content("dark"),
                meta()
                    .name("viewport")
                    .content("width=device-width,initial-scale=1"),
            ]),
            body().class("w-full h-full text-gray-200 bg-neutral-800"),
        ])]);
        let html = doc_immutable.to_html();
        insta::assert_snapshot!(html);

        // let mut doc_mutable = document();
        // let h = doc_mutable.html().class("w-full h-full");
        // h.head().with([
        //     link().rel("stylesheet").href("/assets/preflight.css"),
        //     link().rel("stylesheet").href("/assets/railwind.css"),
        //     script().src("/assets/htmx.1.9.9.js"),
        //     meta().name("color-scheme").content("dark"),
        //     meta()
        //         .name("viewport")
        //         .content("width=device-width,initial-scale=1"),
        // ]);
        // h.body().class("w-full h-full text-gray-200 bg-neutral-800");
        // assert_eq!(doc_immutable, doc_mutable);
        // let html_mutable = doc_mutable.to_html();
        // assert_eq!(html, html_mutable)
    }
}
