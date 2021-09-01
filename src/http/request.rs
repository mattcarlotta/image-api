use super::{bad_req_file, ContentType, Method, Response, RouteHandler, StatusCode};
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request<'a> {
    pub status_code: StatusCode,
    pub method: Method,
    pub body: String,
    pub path: &'a str,
}

impl<'a> Request<'a> {
    /// Attempts to parse a buffer stream
    ///
    /// Arguments:
    /// * buffer: &[u8: 2]
    ///
    /// Returns a `Response` that contains a `status_code`, `method`, `path`, `content_type` and `body`
    pub fn parse(buffer: &'a [u8]) -> Response {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        // attempt to parse the path and method from the incoming request header
        let (path, method) = match req.parse(buffer) {
            Ok(r) => {
                let path = req.path.unwrap_or("");
                let method = req.method.unwrap_or("");
                let method = Method::from_str(method).unwrap();

                // if the request/path/method are invalid return a generic bad request document
                if r.is_partial() || path.is_empty() || method == Method::INVALIDMETHOD {
                    return Response::new(
                        StatusCode::BadRequest,
                        method,
                        path,
                        ContentType::HTML,
                        bad_req_file(),
                    );
                };

                (path, method)
            }
            Err(_) => {
                return Response::new(
                    StatusCode::BadRequest,
                    Method::INVALIDMETHOD,
                    "",
                    ContentType::HTML,
                    bad_req_file(),
                )
            }
        };

        let (status_code, content_type, body) = RouteHandler::delegater(&method, path);

        Response::new(status_code, method, path, content_type, body)
    }
}
