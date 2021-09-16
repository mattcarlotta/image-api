use crate::http::{Request, Response, ResponseType};
use crate::utils::public_path;
use std::fs;

pub fn hello(_req: Request, res: Response) {
    let body = fs::read_to_string(public_path("hello.html")).unwrap();

    res.send(ResponseType::Html(body))
}
