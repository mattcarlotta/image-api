use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ContentType {
    PNG,
    GIF,
    BMP,
    JPEG,
    JPG,
    WEBP,
    AVIF,
    SVG,
    ICO,
    HTML,
    INVALID,
}

impl ContentType {
    /// Converts self into a string slice
    pub fn as_str(&self) -> &str {
        match *self {
            ContentType::PNG => "image/png",
            ContentType::BMP => "image/bmp",
            ContentType::JPG => "image/jpg",
            ContentType::JPEG => "image/jpeg",
            ContentType::WEBP => "image/webp",
            ContentType::AVIF => "image/avif",
            ContentType::SVG => "image/svg",
            ContentType::ICO => "image/ico",
            ContentType::HTML => "text/html; charset=utf-8",
            _ => "",
        }
    }

    /// Conditionally returns a matching ContentType from extension
    pub fn from_extension(s: &str) -> Option<ContentType> {
        match s {
            "png" => Some(ContentType::PNG),
            "gif" => Some(ContentType::GIF),
            "bmp" => Some(ContentType::BMP),
            "jpg" => Some(ContentType::JPG),
            "jpeg" => Some(ContentType::JPEG),
            "webp" => Some(ContentType::WEBP),
            "avif" => Some(ContentType::AVIF),
            "svg" => Some(ContentType::SVG),
            "ico" => Some(ContentType::ICO),
            "html" => Some(ContentType::HTML),
            _ => None,
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.as_str())
    }
}
