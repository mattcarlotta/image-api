use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
    Invalidmethod,
}

impl Method {
    /// Common HTTP request methods as string slices
    pub fn as_str(&self) -> &str {
        match self {
            Method::Connect => "CONNECT",
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
            Method::Patch => "PATCH",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Trace => "TRACE",
            _ => "Invalid Method",
        }
    }

    /// Determine if method is valid
    pub fn is_valid(&self) -> bool {
        !matches!(self, Method::Invalidmethod)
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CONNECT" => Self::Connect,
            "DELETE" => Self::Delete,
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "OPTIONS" => Self::Options,
            "PATCH" => Self::Patch,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "TRACE" => Self::Trace,
            _ => Self::Invalidmethod,
        })
    }
}

impl Display for Method {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.as_str())
    }
}
