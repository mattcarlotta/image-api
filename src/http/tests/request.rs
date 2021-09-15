use crate::http::{Method, Request};
use chrono::prelude::Utc;

#[test]
fn parse_good_request() {
    let buf = b"GET /hello.html HTTP/1.1\r\n\r\n";
    let date = Utc::now();
    let req = Request::new(buf, date);

    assert_eq!(req.method, Method::Get);
    assert_eq!(req.path, "/hello.html");
    assert_eq!(req.timestamp, date);
}

#[test]
fn parse_bad_request() {
    let buf = b"POST / HTTP/1.1\r\n";
    let date = Utc::now();
    let req = Request::new(buf, date);

    assert_eq!(req.method, Method::Invalidmethod);
    assert_eq!(req.path, "/");
    assert_eq!(req.timestamp, date);
}
