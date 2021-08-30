use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
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
    pub fn as_str(&self) -> &str {
        match *self {
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::CONNECT => "CONNECT",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
            Method::INVALIDMETHOD => "Invalid Method",
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
