#![doc = include_str!("../README.md")]

pub mod attr;
#[cfg(feature = "unstable-builder")]
pub mod builder;
pub mod declare;
pub mod element;
mod escape;
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
            body(class("w-full h-full text-gray-200 bg-neutral-800"))
                .with([nothing(), p([]).with(text("bonjour"))]),
        ])]);
        let html = doc.clone().to_html();
        insta::assert_snapshot!(html);
    }
}
