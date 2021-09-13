use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    ServerError,
    NotImplemented,
}

impl StatusCode {
    /// Common HTTP Responses as string slices
    pub fn parse(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::ServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
        }
    }

    /// Converts self into a string slice code
    pub fn code(&self) -> &str {
        match self {
            Self::Ok => "200",
            Self::BadRequest => "400",
            Self::NotFound => "404",
            Self::ServerError => "500",
            Self::NotImplemented => "501",
        }
    }

    /// Converts an int to `StatusCode`
    ///
    /// Arguments:
    /// i: u16
    ///
    pub fn convert(i: u16) -> StatusCode {
        match i {
            200 => StatusCode::Ok,
            400 => StatusCode::BadRequest,
            404 => StatusCode::NotFound,
            500 => StatusCode::ServerError,
            _ => StatusCode::NotImplemented,
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.code())
    }
}


#[cfg(test)]
mod test {
    use super::StatusCode;

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
} 
