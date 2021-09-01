use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    ServerError,
    // NotImplemented = 501,
}

impl StatusCode {
    /// Common HTTP Responses as string slices
    /// `OK`, `Bad Request`, `Not Found` and `Internal Server Error`
    pub fn parse(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::ServerError => "Internal Server Error",
            // Self::NotImplemented => "Not Implemented",
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::Ok => "200",
            Self::BadRequest => "400",
            Self::NotFound => "404",
            Self::ServerError => "500",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.code())
    }
}
