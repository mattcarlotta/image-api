use crate::http::ResponseType;
use std::fs;
use std::path::{Path, PathBuf};

/// Converts a path into a string.
///
/// Arguments:
/// * path - Path
///
pub fn get_string_path(path: impl AsRef<Path>) -> String {
    path.as_ref().to_str().unwrap().into()
}

/// Returns a boilerplate 400 Bad Request HTML document
pub fn bad_req_file() -> ResponseType {
    ResponseType::Text(fs::read_to_string(public_path("400.html")).unwrap())
}

/// Returns a boilerplate 404 Not Found HTML document
pub fn file_not_found() -> ResponseType {
    ResponseType::Text(fs::read_to_string(public_path("404.html")).unwrap())
}

/// Returns a boilerplate 500 Server Error HTML document
pub fn server_error_file() -> ResponseType {
    ResponseType::Text(fs::read_to_string(public_path("500.html")).unwrap())
}

/// Returns the root directory of "static" as a string
pub fn get_root_dir() -> String {
    get_string_path(relative!("static"))
}

/// Joins a path with a relative path to the `static` folder.
///
/// Arguments:
/// * path - String
///
pub fn get_static_file(path: impl AsRef<Path>) -> PathBuf {
    relative!("static").join(path)
}

/// Joins a path with a relative path to the `public` folder.
///
/// Arguments:
/// * path - String
///
pub fn get_public_file(path: impl AsRef<Path>) -> PathBuf {
    relative!("public").join(path)
}

/// Converts a "public" path file to a string
///
/// Arguments:
/// * path - &str
///
pub fn public_path(path: &'_ str) -> String {
    get_string_path(get_public_file(path))
}


#[cfg(test)]
mod test {
    use super::*;

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
        assert!(get_static_file("hello.html").into_os_string().into_string().unwrap().contains("static/hello.html"));
    }

    #[test]
    fn test_get_public_file() {
        assert!(get_public_file("hello.html").into_os_string().into_string().unwrap().contains("public/hello.html"));
    }

    #[test]
    fn test_public_file() {
        assert!(public_path("hello.html").contains("public/hello.html"));
    }
} 
