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

    /// Converts string slice into a `ContentType`
    pub fn convert(s: &str) -> ContentType {
        match s {
            ".png" => ContentType::PNG,
            ".gif" => ContentType::GIF,
            ".bmp" => ContentType::BMP,
            ".jpg" => ContentType::JPG,
            ".jpeg" => ContentType::JPEG,
            ".webp" => ContentType::WEBP,
            ".avif" => ContentType::AVIF,
            ".svg" => ContentType::SVG,
            ".ico" => ContentType::ICO,
            ".html" => ContentType::HTML,
            _ => ContentType::INVALID,
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.as_str())
    }
}
