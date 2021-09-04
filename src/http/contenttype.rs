use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Png,
    Gif,
    Bmp,
    Jpeg,
    Jpg,
    Webp,
    Avif,
    Svg,
    Ico,
    Html,
    Invalid,
}

impl ContentType {
    /// Converts self into a string slice
    pub fn as_str(&self) -> &str {
        match *self {
            ContentType::Png => "image/png",
            ContentType::Bmp => "image/bmp",
            ContentType::Jpg => "image/jpg",
            ContentType::Jpeg => "image/jpeg",
            ContentType::Webp => "image/webp",
            ContentType::Avif => "image/avif",
            ContentType::Svg => "image/svg",
            ContentType::Ico => "image/ico",
            ContentType::Html => "text/html; charset=utf-8",
            _ => "",
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
            "jpg" => Some(ContentType::Jpg),
            "jpeg" => Some(ContentType::Jpeg),
            "webp" => Some(ContentType::Webp),
            "avif" => Some(ContentType::Avif),
            "svg" => Some(ContentType::Svg),
            "ico" => Some(ContentType::Ico),
            "html" => Some(ContentType::Html),
            _ => None,
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.as_str())
    }
}
