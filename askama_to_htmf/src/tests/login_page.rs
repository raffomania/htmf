use std::path::PathBuf;

use crate::Args;

#[test]
pub(crate) fn test_login_page() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/tests/login.html");
    let converted = crate::convert_file(&Args {
        input: d,
        inline: false,
    })
    .unwrap();

    insta::assert_snapshot!(converted);
}

#[test]
pub(crate) fn test_login_page_inline() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/tests/login.html");
    let converted = crate::convert_file(&Args {
        input: d,
        inline: true,
    })
    .unwrap();

    insta::assert_snapshot!(converted);
}
