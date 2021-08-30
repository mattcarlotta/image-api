use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    ServerError = 500,
    NotImplemented = 501,
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
            Self::NotImplemented => "Not Implemented",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", *self as u16)
    }
}
