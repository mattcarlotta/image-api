use crate::http::{Request, Response, ResponseType};
use std::fs;

pub fn hello(_req: Request, mut res: Response) -> () {
    let body = fs::read_to_string("hello.html").unwrap();

    return res.send(ResponseType::Text(body));
}
