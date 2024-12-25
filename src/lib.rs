pub mod attr;
#[cfg(feature = "unstable-builder")]
pub mod builder;
pub mod declare;
pub mod element;
pub mod into_elements;
pub mod prelude;

#[cfg(test)]
mod tests {
    use super::*;

    use declare::*;

    #[test]
    fn base() {
        let doc = document().with([html(class("w-full h-full")).with([
            head([]).with([
                link(rel("stylesheet").href("/assets/preflight.css")),
                link(rel("stylesheet").href("/assets/railwind.css")),
                script(src("/assets/htmx.1.9.9.js")),
                meta(name("color-scheme").content("dark")),
                meta(name("viewport").content("width=device-width,initial-scale=1")),
            ]),
            body(class("w-full h-full text-gray-200 bg-neutral-800")),
        ])]);
        let html = doc.clone().to_html();
        insta::assert_snapshot!(html);

        #[cfg(feature = "unstable-builder")]
        {
            use pretty_assertions::assert_eq;

            let doc_builder = document()
                .html(class("w-full h-full"))
                .with([head([]).with([
                    link(rel("stylesheet").href("/assets/preflight.css")),
                    link(rel("stylesheet").href("/assets/railwind.css")),
                    script(src("/assets/htmx.1.9.9.js")),
                    meta(name("color-scheme").content("dark")),
                    meta(name("viewport").content("width=device-width,initial-scale=1")),
                ])])
                .body(class("w-full h-full text-gray-200 bg-neutral-800"))
                .into_root_element();
            let builder_html = doc_builder.to_html();
            assert_eq!(html, builder_html);
            assert_eq!(doc, doc_builder);
        }
    }
}
