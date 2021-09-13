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

#[cfg(test)]
mod test {
    use super::ContentType;

    #[test]
    fn convert_as_str() {
        assert_eq!(ContentType::Png.as_str(), "image/png");
        assert_eq!(ContentType::Bmp.as_str(), "image/bmp");
        assert_eq!(ContentType::Jpg.as_str(), "image/jpg");
        assert_eq!(ContentType::Jpeg.as_str(), "image/jpeg");
        assert_eq!(ContentType::Webp.as_str(), "image/webp");
        assert_eq!(ContentType::Avif.as_str(), "image/avif");
        assert_eq!(ContentType::Svg.as_str(), "image/svg");
        assert_eq!(ContentType::Ico.as_str(), "image/ico");
        assert_eq!(ContentType::Html.as_str(), "text/html; charset=utf-8");
        assert_eq!(ContentType::Invalid.as_str(), "");
    }

    #[test]
    fn parse_from_extension() {
        assert_eq!(ContentType::from_extension("png"), Some(ContentType::Png));
        assert_eq!(ContentType::from_extension("gif"), Some(ContentType::Gif));
        assert_eq!(ContentType::from_extension("bmp"), Some(ContentType::Bmp));
        assert_eq!(ContentType::from_extension("jpg"), Some(ContentType::Jpg));
        assert_eq!(ContentType::from_extension("jpeg"), Some(ContentType::Jpeg));
        assert_eq!(ContentType::from_extension("webp"), Some(ContentType::Webp));
        assert_eq!(ContentType::from_extension("avif"), Some(ContentType::Avif));
        assert_eq!(ContentType::from_extension("svg"), Some(ContentType::Svg));
        assert_eq!(ContentType::from_extension("ico"), Some(ContentType::Ico));
        assert_eq!(ContentType::from_extension("html"), Some(ContentType::Html));
        assert_eq!(ContentType::from_extension("abcdef"), None);
    }
} 
