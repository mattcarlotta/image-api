use crate::http::ContentType;
use crate::utils::{get_file_path, get_root_dir, get_string_path};
use chunked_transfer::Encoder;
use image::imageops::FilterType;
use image::GenericImageView;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RequestedImage {
    pub content_type: Option<ContentType>,
    pub path: PathBuf,
    pub new_pathname: String,
    pub new_pathname_buf: PathBuf,
    pub ratio: u8,
}

impl<'p, 'r> RequestedImage {
    /// Initialize a new requested image that:
    /// * strips out any provided ratios within the stem -> filename_ratio -> filename
    /// * creates buffers from the stripped pathname and a potential new path (filename_ratio.ext)
    /// * retrieves content type from requested image
    ///
    /// Arguments:
    ///
    /// * `path` - PathBuf
    /// * `ratio` - Option<u8>
    ///
    /// Usage: ```RequestedImage::new(&path, ratio);```
    pub fn new(path: &'p PathBuf, ratio: u8) -> Self {
        // if present, strip any included "_<ratio>" from the filename
        let filename: String = get_string_path(path.to_path_buf())
            .chars()
            .filter(|c| *c != '/')
            .filter(|c| !c.is_digit(10))
            .filter(|c| *c != '_')
            .collect();

        // retrieve file path to "static" folder => <rootdir><static><filename>.<ext>
        let filepath = get_file_path(filename);

        // or assign pathname with ratio: <rootdir><filename>_<ratio>.<ext>
        let pathname = match ratio == 0 {
            true => get_string_path(&filepath),
            false => {
                // retrieve image file stem => <filename>
                let stem = &filepath
                    .file_stem()
                    .and_then(OsStr::to_str)
                    .expect(&format!("Image is missing stem"));

                // retrieve image file stem => <ext>
                let ext = &filepath
                    .extension()
                    .and_then(OsStr::to_str)
                    .expect(&format!("Image is missing extension"));
                format!("{}/{}_{}.{}", get_root_dir(), stem, ratio, ext)
            }
        };

        RequestedImage {
            content_type: path
                .extension()
                .and_then(OsStr::to_str)
                .and_then(ContentType::from_extension),
            path: get_file_path(&filepath),
            new_pathname: pathname.to_string(),
            new_pathname_buf: [&pathname].iter().collect(),
            ratio,
        }
    }

    /// Determines if a requested image path with ratio already exists
    ///
    /// Arguments: (none)
    ///
    /// Usage: ```req_image.exists();```
    pub fn exists(&self) -> bool {
        self.new_pathname_buf.is_file()
    }

    /// Saves a new image to disk with the provided resized ratio of the requested image
    ///
    /// Arguments: (none)
    ///
    /// Usage: ```req_image.save();```
    pub fn save(&self) -> Result<(), String> {
        // open original image
        let original_image = image::open(&self.path).expect("Failed to open image.");

        // pull out width from read image
        let (width, ..) = original_image.dimensions();

        // calculate new image width based on ratio
        let new_image_width = (width * self.ratio as u32 / 100) as u32;

        // resize and save it as the requested ratio
        original_image
            .resize(new_image_width, new_image_width, FilterType::CatmullRom)
            .save(self.new_pathname.to_string())
            .expect("Failed to resize image.");

        Ok(())
    }

    /// Asynchronously reads the requested image and returns its contents as `Vec<u8>`
    ///
    /// Arguments: (none)
    ///
    /// Usage: ```req_image.read();```
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

        let mut body = Vec::new();
        {
            let mut encoder = Encoder::with_chunks_size(&mut body, 8192);
            encoder.write_all(&buf).unwrap();
        }

        Ok(body)
    }
}
