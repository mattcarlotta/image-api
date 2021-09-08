use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
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

    /// Converts self into a string slice code as string
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
