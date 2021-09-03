use crate::http::{Request, Response, ResponseType};
use crate::utils::public_path;
use std::fs;
use std::thread;
use std::time::Duration;

pub fn sleep(_req: Request, res: Response) -> () {
    thread::sleep(Duration::from_secs(5));

    let body = fs::read_to_string(public_path("hello.html")).unwrap();

    return res.send(ResponseType::Text(body));
}
