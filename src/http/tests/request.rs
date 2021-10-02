use crate::http::{Method, Request};
use chrono::prelude::Utc;
use std::sync::Arc;

#[test]
fn parse_good_request() {
    let buf = b"GET /hello.html HTTP/1.1\r\nHost:localhost:5000\r\n\r\n";
    let date = Utc::now();
    let allowedhosts: [String; 2] = ["localhost:5000".to_string(), "localost:3000".to_string()];
    let hosts = Arc::from(allowedhosts.clone());

    let req = Request::new(buf, hosts, date);

    assert_eq!(req.method, Method::Get);
    assert_eq!(req.path, "/hello.html");
    assert_eq!(req.timestamp, date);
}

#[test]
fn parse_bad_request() {
    let buf = b"POST / HTTP/1.1\r\nHost: localhost:5000\r\n";
    let date = Utc::now();
    let allowedhosts: [String; 2] = ["localhost:5000".to_string(), "localost:3000".to_string()];
    let hosts = Arc::from(allowedhosts.clone());

    let req = Request::new(buf, hosts, date);

    assert_eq!(req.method, Method::Invalidmethod);
    assert_eq!(req.path, "/");
    assert_eq!(req.timestamp, date);
}
