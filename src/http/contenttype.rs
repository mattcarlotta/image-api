use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Png,
    Gif,
    Bmp,
    Jpeg,
    Webp,
    Avif,
    Svg,
    Ico,
    Html,
    Text,
    Css,
    Invalid,
}

impl ContentType {
    /// Converts self into a string slice
    pub fn as_str(&self) -> &str {
        match *self {
            ContentType::Png => "image/png",
            ContentType::Gif => "image/gif",
            ContentType::Bmp => "image/bmp",
            ContentType::Jpeg => "image/jpeg",
            ContentType::Webp => "image/webp",
            ContentType::Avif => "image/avif",
            ContentType::Svg => "image/svg",
            ContentType::Ico => "image/ico",
            ContentType::Text => "text/plain; charset=utf-8",
            ContentType::Html => "text/html; charset=utf-8",
            ContentType::Css => "text/css",
            ContentType::Invalid => "",
        }
    }

    /// Converts an image extension to a condtional string slice
    pub fn to_ext(s: &str) -> Option<&str> {
        match s {
            "webp" => Some("webp"),
            _ => None,
        }
    }

    /// Conditionally returns a matching ContentType from extension
    ///
    /// Arguments:
    /// s : &str
    ///
    pub fn from_extension(s: &str) -> Option<ContentType> {
        match s {
            "png" => Some(ContentType::Png),
            "gif" => Some(ContentType::Gif),
            "bmp" => Some(ContentType::Bmp),
            "jpg" | "jpeg" => Some(ContentType::Jpeg),
            "webp" => Some(ContentType::Webp),
            "avif" => Some(ContentType::Avif),
            "svg" => Some(ContentType::Svg),
            "ico" => Some(ContentType::Ico),
            "txt" => Some(ContentType::Text),
            "html" => Some(ContentType::Html),
            "css" => Some(ContentType::Css),
            _ => None,
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.as_str())
    }
}
