use crate::http::ContentType;
use crate::utils::{get_public_file, get_root_dir, get_static_file, get_string_path};
use chunked_transfer::Encoder;
use image::imageops::FilterType;
use image::GenericImageView;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RequestedImage<'p> {
    pub content_type: Option<ContentType>,
    pub path: PathBuf,
    pub filename: String,
    pub new_pathname: String,
    pub new_pathname_buf: PathBuf,
    pub ratio: u8,
    pub ext: &'p str,
}

impl<'p> RequestedImage<'p> {
    /// Initialize a new requested image that:
    /// * strips out any provided ratios within the stem -> filename_ratio -> filename
    /// * creates buffers from the stripped pathname and a potential new path (filename_ratio.ext)
    /// * retrieves content type from requested image
    ///
    /// Arguments:
    ///
    /// * path - PathBuf
    /// * ratio - u8
    /// * public - bool
    ///
    pub fn new(path: &'p Path, ratio: u8, public: bool) -> Self {
        // if present, strip any included "_<ratio>" from the filename
        let filename = get_string_path(path.to_path_buf())
            .chars()
            .filter(|c| *c != '/' && *c != '_' && !c.is_digit(10))
            .collect::<String>();

        // retrieve file path to "static" or "public" folder => <rootdir><filepath><filename>.<ext>
        let filepath = if !public {
            get_static_file(&filename)
        } else {
            get_public_file(&filename)
        };

        let ext = path.extension().and_then(OsStr::to_str);

        // conditionally assign pathname with ratio: <rootdir><filename>_<ratio>.<ext>
        let pathname = if ratio == 0 {
            get_string_path(&filepath)
        } else {
            // retrieve image file stem => <filename>
            let stem = &filepath
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap_or_else(|| panic!("Image is missing stem"));

            // retrieve image file stem => <ext>
            format!("{}/{}_{}.{}", get_root_dir(), stem, ratio, &ext.unwrap())
        };

        RequestedImage {
            content_type: ext.and_then(ContentType::from_extension),
            path: Path::new(&filepath).to_path_buf(),
            filename,
            new_pathname: pathname.to_string(),
            new_pathname_buf: Path::new(&pathname).to_path_buf(),
            ratio,
            ext: ext.unwrap(),
        }
    }

    /// Determines if a requested image path with ratio already exists
    pub fn exists(&self) -> bool {
        self.new_pathname_buf.is_file()
    }

    /// Saves a new image to disk with the provided resized ratio of the requested image
    pub fn save(&self) -> Result<(), String> {
        // open original image
        let original_image = image::open(&self.path).expect("Failed to open image.");

        // pull out width from read image
        let (width, height) = original_image.dimensions();

        // calculate new image width/height based on ratio
        let new_width = (width * self.ratio as u32 / 100) as u32;
        let new_height = (height * self.ratio as u32 / 100) as u32;

        // resize and save it as the requested ratio
        original_image
            .resize(new_width, new_height, FilterType::CatmullRom)
            .save(self.new_pathname.to_string())
            .expect("Failed to resize image.");

        Ok(())
    }

    /// Synchronously reads the requested image and returns its contents as an encoded `Vec<u8>`
    pub fn read(&self) -> Result<Vec<u8>, ()> {
        let mut existing_file = match File::open(&self.new_pathname) {
            Ok(file) => file,
            Err(_) => return Err(()),
        };

        // read the contents of the image
        let mut buf = Vec::new();
        match existing_file.read_to_end(&mut buf) {
            Ok(vec) => vec,
            Err(e) => {
                println!("Unable to read the contents of the image: {}", e);

                return Err(());
            }
        };

        // encode image
        let mut body = Vec::new();
        {
            let mut encoder = Encoder::with_chunks_size(&mut body, 8192);
            encoder.write_all(&buf).unwrap();
        }

        Ok(body)
    }
}
