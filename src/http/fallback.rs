use super::ResponseBody;
use std::fs;

/// Returns a boilerplate 400 Bad Request HTML document
pub fn bad_req_file() -> ResponseBody {
    ResponseBody::Text(fs::read_to_string("400.html").unwrap())
}

/// Returns a boilerplate 404 Not Found HTML document
pub fn file_not_found() -> ResponseBody {
    ResponseBody::Text(fs::read_to_string("404.html").unwrap())
}

/// Returns a boilerplate 500 Server Error HTML document
pub fn server_error_file() -> ResponseBody {
    ResponseBody::Text(fs::read_to_string("500.html").unwrap())
}
