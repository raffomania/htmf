use garde::{self, Validate};

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

#[derive(Validate, Debug, Default)]
pub struct Credentials {
    #[garde(alphanumeric, ascii, length(min = 3, max = 50))]
    pub username: String,
    #[garde(length(min = 10, max = 100))]
    pub password: String,
}

#[derive(askama::Template)]
#[template(path = "login.html")]
pub struct Template {
    errors: FormErrors,
    credentials: Credentials,
}

fn main() {
    let credentials = Credentials {
        username: "james".to_string(),
        password: "hunter2".to_string(),
    };

    let html = Template {
        errors: FormErrors(credentials.validate().unwrap_err()),
        credentials,
    };
    println!("{html}");
}
