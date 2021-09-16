use crate::http::ResponseType;
use std::fs;
use std::path::{Component, Path, PathBuf};

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
    ResponseType::Html(fs::read_to_string(public_path("400.html")).unwrap())
}

/// Returns a boilerplate 404 Not Found HTML document
pub fn file_not_found() -> ResponseType {
    ResponseType::Html(fs::read_to_string(public_path("404.html")).unwrap())
}

/// Returns a boilerplate 500 Server Error HTML document
pub fn server_error_file() -> ResponseType {
    ResponseType::Html(fs::read_to_string(public_path("500.html")).unwrap())
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

/// Parses requsted path directories as a string
///
/// Arguments:
/// path: Path
///
pub fn parse_dirs(path: impl AsRef<Path>) -> String {
    let dirs = get_string_path(&path);
    let dirs = dirs.split('/').collect::<Vec<&str>>();

    let mut directories = String::new();
    if dirs.len() > 1 {
        for dir in dirs[0..dirs.len() - 1].iter() {
            let d = [dir, "/"].concat();
            directories.push_str(d.as_str());
        }
    }

    directories
}

/// Normalizes a path file to a pathbuf
///
/// Arguments:
/// * path - &str
///
pub fn normalize_path(path: impl AsRef<Path>) -> PathBuf {
    let ends_with_slash = path.as_ref().to_str().map_or(false, |s| s.ends_with('/'));
    let mut normalized = PathBuf::new();
    for component in path.as_ref().components() {
        match &component {
            Component::ParentDir => {
                if !normalized.pop() {
                    normalized.push(component);
                }
            }
            _ => {
                normalized.push(component);
            }
        }
    }
    if ends_with_slash {
        normalized.push("");
    }
    normalized
}
