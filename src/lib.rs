pub mod attr;
pub mod declare;
pub mod element;

#[cfg(test)]
mod tests {
    use super::*;

    use declare::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn base() {
        let doc_immutable = document()
            .with([html().class("w-full h-full").with([
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
            ])])
            .build();
        let html_immutable = doc_immutable.clone().to_html();
        insta::assert_snapshot!(html_immutable);

        let doc_mut = document()
            .html()
            .class("w-full h-full")
            .with([head().with([
                link().rel("stylesheet").href("/assets/preflight.css"),
                link().rel("stylesheet").href("/assets/railwind.css"),
                script().src("/assets/htmx.1.9.9.js"),
                meta().name("color-scheme").content("dark"),
                meta()
                    .name("viewport")
                    .content("width=device-width,initial-scale=1"),
            ])])
            .body()
            .class("w-full h-full text-gray-200 bg-neutral-800")
            .build();
        let html_mutable = doc_mut.to_html();
        assert_eq!(html_immutable, html_mutable);
        assert_eq!(doc_immutable, doc_mut);
    }
}
