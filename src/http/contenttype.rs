use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter, Result as FmtResult};
use std::str::{FromStr, Utf8Error};

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

    pub fn set(s: &str) -> ContentType {
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

impl FromStr for ContentType {
    type Err = InvalidExtension;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".png" => Ok(Self::PNG),
            ".gif" => Ok(Self::GIF),
            ".bmp" => Ok(Self::BMP),
            ".jpg" => Ok(Self::JPG),
            ".jpeg" => Ok(Self::JPEG),
            ".webp" => Ok(Self::WEBP),
            ".avif" => Ok(Self::AVIF),
            ".svg" => Ok(Self::SVG),
            ".ico" => Ok(Self::ICO),
            ".html" => Ok(Self::HTML),
            _ => Err(InvalidExtension),
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.as_str())
    }
}

pub struct InvalidExtension;

impl From<Utf8Error> for InvalidExtension {
    fn from(_: Utf8Error) -> Self {
        InvalidExtension
    }
}

impl Display for InvalidExtension {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", "Invalid Extension")
    }
}

impl Debug for InvalidExtension {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", "Invalid Extension")
    }
}

impl Error for InvalidExtension {}
