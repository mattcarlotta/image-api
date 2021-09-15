use crate::http::Method;
use std::str::FromStr;

#[test]
fn method_is_valid() {
    assert!(Method::Get.is_valid());
    assert!(!Method::Invalidmethod.is_valid());
}

#[test]
fn method_convert_as_str() {
    assert_eq!(Method::Connect.as_str(), "CONNECT");
    assert_eq!(Method::Delete.as_str(), "DELETE");
    assert_eq!(Method::Get.as_str(), "GET");
    assert_eq!(Method::Head.as_str(), "HEAD");
    assert_eq!(Method::Options.as_str(), "OPTIONS");
    assert_eq!(Method::Patch.as_str(), "PATCH");
    assert_eq!(Method::Post.as_str(), "POST");
    assert_eq!(Method::Put.as_str(), "PUT");
    assert_eq!(Method::Trace.as_str(), "TRACE");
    assert_eq!(Method::Invalidmethod.as_str(), "Invalid Method");
}

#[test]
fn method_convert_from_str() {
    assert_eq!(Method::from_str("CONNECT"), Ok(Method::Connect));
    assert_eq!(Method::from_str("DELETE"), Ok(Method::Delete));
    assert_eq!(Method::from_str("GET"), Ok(Method::Get));
    assert_eq!(Method::from_str("HEAD"), Ok(Method::Head));
    assert_eq!(Method::from_str("OPTIONS"), Ok(Method::Options));
    assert_eq!(Method::from_str("PATCH"), Ok(Method::Patch));
    assert_eq!(Method::from_str("POST"), Ok(Method::Post));
    assert_eq!(Method::from_str("PUT"), Ok(Method::Put));
    assert_eq!(Method::from_str("TRACE"), Ok(Method::Trace));
    assert_eq!(Method::from_str("INVALIDMETHOD"), Ok(Method::Invalidmethod));
}
