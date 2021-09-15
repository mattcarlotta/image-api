use crate::http::StatusCode;

#[test]
fn statuscode_parse() {
    assert_eq!(StatusCode::Ok.parse(), "OK");
    assert_eq!(StatusCode::BadRequest.parse(), "Bad Request");
    assert_eq!(StatusCode::NotFound.parse(), "Not Found");
    assert_eq!(StatusCode::ServerError.parse(), "Internal Server Error");
    assert_eq!(StatusCode::NotImplemented.parse(), "Not Implemented");
}

#[test]
fn statuscode_code_as_str() {
    assert_eq!(StatusCode::Ok.code(), "200");
    assert_eq!(StatusCode::BadRequest.code(), "400");
    assert_eq!(StatusCode::NotFound.code(), "404");
    assert_eq!(StatusCode::ServerError.code(), "500");
    assert_eq!(StatusCode::NotImplemented.code(), "501");
}

#[test]
fn statuscode_convert() {
    assert_eq!(StatusCode::convert(200), StatusCode::Ok);
    assert_eq!(StatusCode::convert(400), StatusCode::BadRequest);
    assert_eq!(StatusCode::convert(404), StatusCode::NotFound);
    assert_eq!(StatusCode::convert(500), StatusCode::ServerError);
    assert_eq!(StatusCode::convert(501), StatusCode::NotImplemented);
}
