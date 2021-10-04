use crate::http::ContentType;
use crate::utils::{get_public_file, get_root_dir, get_static_file, get_string_path, parse_dirs};
use chunked_transfer::Encoder as ImageEncoder;
use image::imageops::FilterType;
use image::GenericImageView;
use libwebp_sys::{WebPEncodeLosslessRGBA, WebPFree};
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
        let filename = get_string_path(path)
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

            let mut img_ratio = String::new();
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
            new_pathname_buf: PathBuf::from(&pathname),
            ratio,
            ext: ext.unwrap(),
        }
    }

    /// Determines if a requested image path with ratio already exists
    pub fn exists(&self) -> bool {
        self.new_pathname_buf.is_file()
    }

    /// Saves a new image to disk with the provided resized ratio of the requested image
    pub fn save(&self) -> Result<(), &str> {
        // open original image
        let orig_image = match image::open(&self.path) {
            Ok(f) => f,
            Err(_) => return Err("Failed to open image"),
        };

        // pull out width from read image
        let (width, height) = orig_image.dimensions();

        let ratio = if self.ratio == 0 {
            100_u32
        } else {
            self.ratio as u32
        };

        let new_width = (width * ratio / 100) as u32;
        let new_height = (height * ratio / 100) as u32;
        let orig_ext = self.path.extension().and_then(OsStr::to_str).unwrap();

        // This works around the issue where webp file types can't
        // be resized nor saved, so the original image is downsampled and converted
        // to the new file type
        if orig_ext != self.ext {
            let filestem = self
                .new_pathname
                .split('.')
                .filter(|l| l.contains('/'))
                .collect::<String>();

            // convert new image to orignal extension
            let file_path = format!("{}.{}", filestem, orig_ext);
            let downsampled_path = Path::new(&file_path);

            if !downsampled_path.is_file() {
                // downsample original image and save it
                if orig_image
                    .resize(new_width, new_height, FilterType::CatmullRom)
                    .save(&file_path)
                    .is_err()
                {
                    return Err("Failed to downsample original png");
                };
            }

            // open downsampled image
            let downsampled_image = match image::open(file_path) {
                Ok(f) => f,
                Err(_) => return Err("Failed to open downsampled image"),
            };

            let (ds_width, ds_height) = downsampled_image.dimensions();

            let raw_img = downsampled_image.into_rgba8();

            unsafe {
                let w = ds_width as i32;
                let h = ds_height as i32;
                let mut buf_ptr = std::ptr::null_mut();
                let stride = w * 4;
                let created_img = match File::create(self.new_pathname.as_str()) {
                    Ok(f) => f,
                    Err(_) => return Err("Failed to create new image file"),
                };
                let mut output = BufWriter::new(created_img);

                // losslessly encode downsampled image
                let len = WebPEncodeLosslessRGBA(raw_img.as_ptr(), w, h, stride, &mut buf_ptr);

                // create encoded image slice from memory
                let img_slice = std::slice::from_raw_parts(buf_ptr, len);

                // write encoded image slice to file
                if output.write_all(img_slice).is_err() {
                    return Err("Failed to save encoded image");
                };

                // flush buf writer
                if output.flush().is_err() {
                    return Err("Failed to flush encoded image");
                };

                WebPFree(buf_ptr as _);

                Ok(())
            }
        } else {
            // resize original image and save it as the requested ratio
            match orig_image
                .resize(new_width, new_height, FilterType::Triangle)
                .save(self.new_pathname.as_str())
            {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to save the resized image"),
            }
        }
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
            let mut encoder = ImageEncoder::with_chunks_size(&mut contents, 8192);
            encoder.write_all(&buf).expect("Failed to encode image");
        }

        Ok(contents)
    }
}
