use std::path::PathBuf;

#[test]
pub(crate) fn test_login_page() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/tests/login.html");
    let converted = crate::convert_file(&d).unwrap();

    insta::assert_snapshot!(converted);
}
