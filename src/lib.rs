use std::borrow::Cow;

pub fn a<'a, const A: usize, S, C>(attrs: [(&'static str, S); A], children: C) -> Element<'a>
where
    S: Into<Cow<'a, str>>,
    C: Into<Vec<Element<'a>>>,
{
    Element::Tag {
        tag: "a".to_string(),
        attrs: attrs.into_iter().map(|(k, v)| (k, v.into())).collect(),
        children: children.into(),
    }
}

pub fn href(target: &str) -> (&'static str, &str) {
    ("href", target)
}

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
                let attrs_html: String =
                    attrs.iter().map(|(k, v)| format!(r#"{k}="{v}""#)).collect();
                format!("<{tag} {attrs_html}>{children_html}</{tag}>")
            }
            Element::Text(t) => t.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let elem = a(
            [href("https://www.rafa.ee")],
            [Element::Text("My Site".into())],
        );
        let expected_html = r#"<a href="https://www.rafa.ee">My Site</a>"#;
        assert_eq!(elem.to_html(), expected_html);
    }
}
