use crate::utils::*;

#[test]
fn test_get_string_path() {
    assert_eq!(get_string_path("test.html"), "test.html");
}

#[test]
fn test_get_root_dir() {
    assert!(get_root_dir().contains("static"));
}

#[test]
fn test_get_static_file() {
    assert!(get_static_file("hello.html")
        .into_os_string()
        .into_string()
        .unwrap()
        .contains("static/hello.html"));
}

#[test]
fn test_get_public_file() {
    assert!(get_public_file("hello.html")
        .into_os_string()
        .into_string()
        .unwrap()
        .contains("public/hello.html"));
}

#[test]
fn test_parse_dirs() {
    let path = normalize_path("/a/b////c////////d/e//f.ext");

    assert_eq!(parse_dirs(path), "/a/b/c/d/e/");
}

#[test]
fn test_public_file() {
    assert!(public_path("hello.html").contains("public/hello.html"));
}
