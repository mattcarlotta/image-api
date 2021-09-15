use std::path::Path;

use crate::http::{QueryString, Request, Response, ResponseType};
use crate::lrucache::Cache;
use crate::reqimage::RequestedImage;
use crate::utils::{bad_req_file, file_not_found, server_error_file};

pub fn image(cache: Cache, req: Request, res: Response) {
    let mut path = req.path.as_str();
    let mut query = QueryString::new();

    // if a query is found, parse it and remove it from the path
    if let Some(i) = path.find('?') {
        query.parse(&req.path[i + 1..]);
        path = &path[..i];
    }

    let path = Path::new(path.strip_prefix('/').unwrap());
    let ratio = query.get("ratio");

    // ensure that path is a directory
    if path.extension().is_none() || path.as_os_str().is_empty() {
        return res.set_status(404).send(file_not_found());
    }

    // converts supplied "ratio" to a valid u8 integer
    let ratio = ratio
        .map(str::parse::<u8>)
        .map(Result::ok)
        .flatten()
        .unwrap_or(0);

    // ensure the provided ratio is standardized
    if ![0, 10, 20, 35, 50, 75, 90].contains(&ratio) {
        return res.set_status(400).send(bad_req_file());
    }

    // initialize requested image
    let img = RequestedImage::new(path, ratio, false);

    // ensure the requested image has a valid content type
    if img.content_type.is_none() {
        return res.set_status(400).send(bad_req_file());
    }

    let mut imgcache = cache.lock().unwrap();
    // determine if cache contains requested image
    if !imgcache.contains_key(&img.new_pathname) {
        // return if requested image doesn't exist
        if !img.path.is_file() {
            return res.set_status(404).send(file_not_found());
        }

        // create a new image from original if one doesn't exist already
        if !img.exists() && img.save().is_err() {
            return res.set_status(500).send(server_error_file());
        }

        // read the requested image from disk and encode it
        match img.read() {
            Ok(contents) => imgcache.insert(img.new_pathname.clone(), contents),
            Err(_) => return res.set_status(500).send(server_error_file()),
        };

        // log the requested file and ratio
        let mut img_query = String::new();
        if ratio > 0 {
            img_query = format!("?ratio={}", ratio)
        }

        println!(
            "[{}] - Saved {}{} image into LRU cache",
            req.timestamp.format("%B %d %Y, %I:%M:%S %P"),
            img.filename,
            img_query
        );
    }

    // retrieve saved encoded image from the cache
    let cached_image = match imgcache.get(&img.new_pathname) {
        Some(i) => i,
        None => return res.set_status(500).send(server_error_file()),
    };

    res.set_content(img.ext)
        .send(ResponseType::Chunked(cached_image.to_vec()))
}
