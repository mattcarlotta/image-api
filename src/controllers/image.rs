use crate::http::{file_not_found, server_error_file, Request, Response, ResponseType};
use chunked_transfer::Encoder;
use std::fs::File;
use std::io::prelude::*;

pub fn image(_req: Request, mut res: Response) -> () {
    // TODO - Make sure requested image size doesn't extend beyond actual image dimensions
    // open requested image
    let mut existing_file = match File::open("placeholder.png") {
        Ok(file) => file,
        Err(_) => {
            res.set_status(404);

            return res.send(file_not_found());
        }
    };

    // read the contents of the image
    let mut buf = Vec::new();
    match existing_file.read_to_end(&mut buf) {
        Ok(vec) => vec,
        Err(e) => {
            println!("Unable to read the contents of the image: {}", e);

            res.set_status(500);

            return res.send(server_error_file());
        }
    };

    let mut body = Vec::new();
    {
        let mut encoder = Encoder::with_chunks_size(&mut body, 8192);
        encoder.write_all(&buf).unwrap();
    }

    // TODO Set this as parsed extension
    res.set_content(".png");

    return res.send(ResponseType::Chunked(body));
}
