use crate::http::{Request, Response, ResponseType};
use std::fs;
use std::thread;
use std::time::Duration;

pub fn sleep(_req: Request, mut res: Response) -> () {
    thread::sleep(Duration::from_secs(5));

    let body = fs::read_to_string("hello.html").unwrap();

    return res.send(ResponseType::Text(body));
}
