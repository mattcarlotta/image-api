use super::ResponseBody;
use std::fs;

/// Returns a boilerplate 400 Bad Request HTML document
pub fn bad_req_file() -> ResponseBody {
    ResponseBody::Str(fs::read_to_string("400.html").unwrap())
}
