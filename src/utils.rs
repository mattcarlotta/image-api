use crate::http::ResponseType;
use std::fs;
use std::path::{Path, PathBuf};

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

/// Converts a string into a path buffer.
pub fn get_root_dir<'a>() -> &'a str {
    Path::new(relative!("static")).to_str().unwrap()
}

/// Joins a pathbuf with a relative path to the `static` folder.
///
/// Arguments:
/// * path - String
///
pub fn get_file_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(relative!("static")).join(path)
}

/// Converts a path buffer into a string.
///
/// Arguments:
/// * path - PathBuf
///
pub fn get_string_path(path: impl AsRef<Path>) -> String {
    path.as_ref().to_str().unwrap().into()
}

/// Appends public path directory to a path
///
/// Arguments:
/// * path - &str
///
pub fn public_path(path: &'_ str) -> String {
    Path::new(relative!("public"))
        .join(path)
        .to_str()
        .unwrap()
        .into()
}
