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

    /// Determine if parsed method is valid
    pub fn is_valid(&self) -> bool {
        *self != Method::Invalidmethod
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


#[cfg(test)]
mod test {
    use super::Method;
    use std::str::FromStr;

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
} 
