use crate::http::{Request, Response, ResponseType};
use crate::utils::public_path;
use std::fs;

pub fn styles(_req: Request, res: Response) {
    let body = fs::read_to_string(public_path("styles.css")).unwrap();

    res.set_content("css").send(ResponseType::Css(body))
}
