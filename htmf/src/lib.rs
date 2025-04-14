#![doc = include_str!("../README.md")]

pub mod attr;
#[cfg(feature = "unstable-builder")]
pub mod builder;
pub mod declare;
pub mod declare_inline;
pub mod element;
mod escape;
pub mod into_attrs;
pub mod into_elements;
pub mod prelude;
pub mod prelude_inline;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        use declare::*;

        let doc = document().with([html(class("w-full h-full")).with([
            head([]).with([
                link([rel("stylesheet"), href("/assets/preflight.css")]),
                link([rel("stylesheet"), href("/assets/railwind.css")]),
                script(src("/assets/htmx.1.9.9.js")),
                meta([name("color-scheme"), content("dark")]),
                meta([
                    name("viewport"),
                    content("width=device-width,initial-scale=1"),
                ]),
            ]),
            body(class("w-full h-full text-gray-200 bg-neutral-800")).with([
                nothing(),
                p([]).with(text("bonjour")),
                fragment().with([
                    div([]).with(fragment()),
                    label([
                        class("mt-4 text-neutral-400"),
                        for_("credentials[password]"),
                    ])
                    .with("Password"),
                    input([
                        type_("password"),
                        attr("hx-post", "/validate-password"),
                        name("credentials[password]"),
                        class("rounded py-1.5 px-3 mt-2 bg-neutral-900"),
                        required("true"),
                    ]),
                    input([value(10), type_("number")]),
                ]),
            ]),
        ])]);
        let html = doc.clone().to_html();
        insta::assert_snapshot!(html);
        let formatted = doc.clone().to_html_pretty().unwrap();
        insta::assert_snapshot!(formatted);
    }

    #[test]
    fn declare_list() {
        use declare_inline::*;

        let doc = document([html(
            class("w-full h-full"),
            [
                head(
                    [],
                    [
                        link([rel("stylesheet"), href("/assets/preflight.css")]),
                        link([rel("stylesheet"), href("/assets/railwind.css")]),
                        script(src("/assets/htmx.1.9.9.js"), ()),
                        meta([name("color-scheme"), content("dark")]),
                        meta([
                            name("viewport"),
                            content("width=device-width,initial-scale=1"),
                        ]),
                    ],
                ),
                body(
                    class("w-full h-full text-gray-200 bg-neutral-800"),
                    [
                        nothing(),
                        p([], text("bonjour")),
                        fragment([
                            div([], ()),
                            label(
                                [
                                    class("mt-4 text-neutral-400"),
                                    for_("credentials[password]"),
                                ],
                                "Password",
                            ),
                            input([
                                type_("password"),
                                name("credentials[password]"),
                                class("rounded py-1.5 px-3 mt-2 bg-neutral-900"),
                                required("true"),
                            ]),
                        ]),
                    ],
                ),
            ],
        )]);
        let html = doc.clone().to_html_pretty().unwrap();
        insta::assert_snapshot!(html);
    }
}
