use crate::http::{Request, Response, ResponseType};
use crate::utils::{public_path, server_error_file};
use chunked_transfer::Encoder;
use std::fs::File;
use std::io::prelude::*;

pub fn favicon(_req: Request, res: Response) {
    let mut existing_file = match File::open(public_path("favicon.ico")) {
        Ok(file) => file,
        Err(_) => return res.set_status(500).send(server_error_file()),
    };

    // read the contents of the image
    let mut buf = Vec::new();
    match existing_file.read_to_end(&mut buf) {
        Ok(vec) => vec,
        Err(e) => {
            println!("Unable to read the contents of the image: {}", e);

            return res.set_status(500).send(server_error_file());
        }
    };

    // encode image
    let mut body = Vec::new();
    {
        let mut encoder = Encoder::with_chunks_size(&mut body, 8192);
        encoder.write_all(&buf).unwrap();
    }

    res.set_content("ico").send(ResponseType::Chunked(body))
}
