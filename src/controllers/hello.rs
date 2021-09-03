use crate::{
    http::{Request, Response, ResponseType},
    utils::public_path,
};
use std::fs;

pub fn hello(_req: Request, mut res: Response) -> () {
    let body = fs::read_to_string(public_path("hello.html")).unwrap();

    return res.send(ResponseType::Text(body));
}
