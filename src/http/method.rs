use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Method {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
    INVALIDMETHOD,
}

impl Method {
    /// Common HTTP Request Methods
    /// `GET`, `OPTIONS`, `POST`, `PUT` ...etc
    pub fn as_str(&self) -> &str {
        match *self {
            Method::CONNECT => "CONNECT",
            Method::DELETE => "DELETE",
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::PATCH => "PATCH",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::TRACE => "TRACE",
            _ => "Invalid Method",
        }
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "CONNECT" => Self::CONNECT,
            "DELETE" => Self::DELETE,
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "OPTIONS" => Self::OPTIONS,
            "PATCH" => Self::PATCH,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "TRACE" => Self::TRACE,
            _ => Self::INVALIDMETHOD,
        })
    }
}

impl Display for Method {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.as_str())
    }
}
