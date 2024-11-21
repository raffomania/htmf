use garde::{self, Validate};
use htmf::{declare::*, element::Element};

#[derive(Debug)]
pub struct FormErrors(pub garde::Report);

impl FormErrors {
    pub fn filter(&self, path_str: &str) -> Vec<String> {
        self.0
            .iter()
            .filter(|(path, _error)| path.to_string() == path_str)
            .map(|(_path, error)| error.to_string())
            .collect()
    }
}

pub struct Layout {
    pub logged_in_username: String,
    pub lists: Vec<String>,
}

#[derive(Validate, Debug, Default)]
pub struct Credentials {
    #[garde(alphanumeric, ascii, length(min = 3, max = 50))]
    pub username: String,
    #[garde(length(min = 10, max = 100))]
    pub password: String,
}

fn base(children: Vec<Element>) -> Element {
    html(class("w-full h-full")).with([
        head([]).with([
            link([rel("stylesheet"), href("/assets/preflight.css")]),
            link(rel("stylesheet").href("/assets/railwind.css")),
            script(src("/assets/htmx.1.9.9.js")),
            meta(name("color-scheme").content("dark")),
            meta(name("viewport").content("width=device-width,initial-scale=1")),
        ]),
        body(class("w-full h-full text-gray-200 bg-neutral-800")).with(children),
    ])
}

fn login(errors: FormErrors, credentials: Credentials) -> Element<'static> {
    let errors_fragment = |errors: &FormErrors, path| {
        let mut fragment = fragment();
        for message in errors.filter(path) {
            fragment = fragment.with([p(class("text-red-700")).with(text(message))]);
        }

        fragment
    };

    let submit_button = button(type_("submit").class(
        "leading-6 bg-neutral-300 mt-5 font-semibold rounded py-1.5 flex items-center \
         justify-center disabled:bg-neutral-500 text-neutral-900",
    ))
    .with([
        text("Sign in").with(span(class("inline-block w-09 h-4")).with(span(class(
            "block w-4 h-4 -ml-6 border-2 rounded-full border-neutral-900 animate-spin \
             border-t-transparent htmx-indicator",
        )))),
    ]);

    let form_fields = [
        h1(class("text-2xl font-bold tracking-tight text-center"))
            .with(text("Sign in to your account")),
        label(class("mt-10 text-neutral-400").name("credentials[username]")).with(text("Username")),
        errors_fragment(&errors, "username"),
        input([
            type_("text"),
            name("credentials[username]"),
            class("rounded py-1.5 px-3 mt-2 bg-neutral-900"),
            value(credentials.username),
            required("true"),
        ]),
        label(class("mt-4 text-neutral-400").name("credentials[password]")).with(text("Password")),
        errors_fragment(&errors, "password"),
        input(
            type_("password")
                .name("credentials[password]")
                .class("rounded py-1.5 px-3 mt-2 bg-neutral-900")
                .required("true"),
        ),
        errors_fragment(&errors, "root"),
    ];

    base(
        [form([
            action("login"),
            method("post"),
            class("flex flex-col justify-center flex-1 max-w-md min-h-full px-4 mx-auto"),
            attr("hx-boost", "true"),
        ])
        .with(form_fields)
        .with([submit_button])]
        .into(),
    )
}

fn main() {
    let credentials = Credentials {
        username: "hunter".to_string(),
        password: "hunter2".to_string(),
    };
    let errors = FormErrors(credentials.validate().unwrap_err());
    let html = login(errors, credentials).to_html();
    println!("{html}");
}
