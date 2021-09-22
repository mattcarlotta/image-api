use crate::http::ContentType;
use crate::utils::{get_public_file, get_root_dir, get_static_file, get_string_path, parse_dirs};
use image::imageops::FilterType;
use image::GenericImageView;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, Write};
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
    /// * ext - Otion<&str>
    /// * public - bool
    ///
    pub fn new(path: &'p Path, ratio: u8, new_ext: Option<&'p str>, public: bool) -> Self {
        // if present strip any included "_<ratio>" from the filename
        let filename = get_string_path(path.to_path_buf())
            .chars()
            .filter(|c| *c != '_' && !c.is_digit(10))
            .collect::<String>();

        // retrieve file path to "static" or "public" folder: <rootdir><filepath><filename>.<ext>
        let filepath = if !public {
            get_static_file(&filename)
        } else {
            get_public_file(&filename)
        };

        let ext = match new_ext {
            Some(ext) => Some(ext),
            None => path.extension().and_then(OsStr::to_str),
        };

        // conditionally assign pathname with ratio: <rootdir><filename>_<ratio>.<ext>
        let pathname = if ratio == 0 && new_ext.is_none() {
            get_string_path(&filepath)
        } else {
            // parse any requested directories
            let directories = parse_dirs(path);

            let mut img_ratio = String::from("");
            if ratio != 0 {
                let fmt_r = format!("_{}", ratio);
                img_ratio.push_str(&fmt_r);
            }

            // retrieve image file stem => <filename>
            let stem = &filepath
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap_or_else(|| panic!("Image is missing stem"));

            // format image file: <rootdir><filename>_<ratio>.<ext>
            format!(
                "{}/{}{}{}.{}",
                get_root_dir(),
                directories,
                stem,
                img_ratio,
                &ext.unwrap()
            )
        };

        RequestedImage {
            content_type: ext.and_then(ContentType::from_extension),
            path: filepath,
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
        let mut new_image = image::open(&self.path).expect("Failed to open image.");

        // pull out width from read image
        let (width, height) = new_image.dimensions();

        let ratio = if self.ratio == 0 {
            100_u32
        } else {
            self.ratio as u32
        };

        // calculate new image width/height based on ratio
        let new_width = (width * ratio / 100) as u32;
        let new_height = (height * ratio / 100) as u32;

        //if self.ext == "webp" {
        //let img = new_image.to_rgb8();
        //let webp_image = webp::Encoder::new(&img, webp::PixelLayout::Rgb, width, height);

        //let output = webp_image.encode_lossless();
        //let mut webp_file = BufWriter::new(File::create(&self.new_pathname).unwrap());

        //webp_file
        //.write_all(&output)
        //.expect("Failed to save webp file");

        //webp_file.flush().expect("Failed to flush webp image");

        //new_image = image::open(&self.new_pathname).expect("Failed to open saved webp image");
        //}

        new_image
            .resize(new_width, new_height, FilterType::CatmullRom)
            .save(self.new_pathname.as_str())
            .expect("Failed to resize image.");
        // resize and save it as the requested ratio

        Ok(())
    }

    /// Synchronously reads the requested image and returns its contents as an encoded `Vec<u8>`
    pub fn read(&self) -> Result<Vec<u8>, ()> {
        // open requested file
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

        // encode the image
        let mut contents = Vec::new();
        {
            let mut encoder = chunked_transfer::Encoder::with_chunks_size(&mut contents, 8192);
            encoder.write_all(&buf).expect("Failed to encode image");
        }

        Ok(contents)
    }
}
