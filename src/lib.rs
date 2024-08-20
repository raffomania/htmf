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
    Text(Cow<'a, str>),
}

impl<'a> Element<'a> {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use declare::*;

    #[test]
    fn base() {
        let actual_html = html(
            [class("w-full h-full")],
            [
                head(
                    [],
                    [
                        link([rel("stylesheet"), href("/assets/preflight.css")]),
                        link([rel("stylesheet"), href("/assets/railwind.css")]),
                        script([src("/assets/htmx.1.9.9.js")], []),
                        meta([name("color-scheme"), content("dark")]),
                        meta([
                            name("viewport"),
                            content("width=device-width,initial-scale=1"),
                        ]),
                    ],
                ),
                body([class("w-full h-full text-gray-200 bg-neutral-800")], []),
            ],
        )
        .to_html();
        insta::assert_snapshot!(actual_html);
    }
}
