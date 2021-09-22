use crate::http::{Request, Response, ResponseType};
use crate::reqimage::RequestedImage;
use crate::utils::server_error_file;
use std::path::Path;

pub fn favicon(_req: Request, res: Response) {
    let path = Path::new("favicon.ico");
    let img = RequestedImage::new(path, 0, None, true);

    let body = match img.read() {
        Ok(contents) => contents,
        Err(_) => return res.set_status(500).send(server_error_file()),
    };

    res.set_content("ico").send(ResponseType::Chunked(body))
}
