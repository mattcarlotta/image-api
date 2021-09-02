use crate::http::{QueryString, Request, Response, ResponseType};
use crate::reqimage::RequestedImage;
use crate::utils::{bad_req_file, file_not_found, server_error_file};
use std::path::PathBuf;

pub fn image(req: Request, mut res: Response) -> () {
    let mut path = req.path;
    let mut query = QueryString::new();

    // if a query is found, parse it and remove it from the path
    if let Some(i) = path.find('?') {
        query.parse(&req.path[i + 1..]);
        path = &path[..i];
    }

    let path: PathBuf = [path].iter().collect();
    let ratio = query.get("ratio");

    // ensure that path is a directory
    if path.extension().is_none() || path.as_os_str().is_empty() {
        res.set_status(404);
        return res.send(file_not_found());
    }

    // converts supplied "ratio" to a valid u8 integer
    let ratio = ratio
        .map(str::parse::<u8>)
        .map(Result::ok)
        .flatten()
        .unwrap_or(0);

    // ensure the provided ratio is standardized
    if ![0, 20, 35, 50, 75, 90].contains(&ratio) {
        res.set_status(400);

        return res.send(bad_req_file());
    }

    // initialize requested image
    let img = RequestedImage::new(&path, ratio);

    // ensure the requested image has a valid content type
    if img.content_type.is_none() {
        res.set_status(400);

        return res.send(bad_req_file());
    }

    //    let mut cache = state.lock().await;
    // determine if cache contains requested image
    // if !cache.contains_key(&img.new_pathname) {
    // return if requested image doesn't exist
    if !img.path.is_file() {
        res.set_status(404);

        return res.send(file_not_found());
    }

    // create a new image from original if one doesn't exist already
    if !img.exists() {
        match img.save() {
            Ok(()) => (),
            Err(_) => {
                res.set_status(500);

                return res.send(server_error_file());
            }
        };
    }

    // read the original or new image and store its contents into cache
    let body = match img.read() {
        Ok(contents) => contents,
        Err(_) => {
            res.set_status(500);

            return res.send(server_error_file());
        }
    };

    // match img.read() {
    //    Ok(contents) => cache.insert(img.new_pathname.clone(), contents),
    //    Err(reason) => return Err(send_400_response(reason.to_string())),
    //};

    // println!("Saved requested image into cache.");
    // }

    // retrieve saved image from the cache
    //let cached_image = cache
    //.get(&img.new_pathname)
    //.expect("Unable to retrieve image entry from cache.");

    // print!("Served requested image from cache.");
    //
    //

    res.set_content(img.ext);

    return res.send(ResponseType::Chunked(body));
}
