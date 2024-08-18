use std::borrow::Cow;

pub mod declare;

type Attr<'a> = (&'static str, Cow<'a, str>);

#[derive(PartialEq, Eq, Debug)]
pub enum Element<'a> {
    Tag {
        tag: String,
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
                let children_html: String = children.iter().map(|c| c.to_html()).collect();
                let attrs_html: String = attrs
                    .iter()
                    .map(|(k, v)| format!(r#"{k}="{v}""#))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("<{tag} {attrs_html}>{children_html}</{tag}>")
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
    fn it_works() {
        let actual_html = a(
            [href("https://www.rafa.ee"), class("link")],
            [text("My Site")],
        )
        .to_html();
        let expected_html = r#"<a href="https://www.rafa.ee" class="link">My Site</a>"#;
        assert_eq!(actual_html, expected_html);
    }
}
