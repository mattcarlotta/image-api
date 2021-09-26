use crate::http::{Request, Response, ResponseType};
use crate::utils::{file_not_found, public_path};
//use std::env;
use std::fs;

pub fn testing(_req: Request, res: Response) {
    //let in_testing = env::var("testing").unwrap_or_else(|_| "".to_string());
    //if in_testing.is_empty() {
    //return res.set_status(404).send(file_not_found());
    //}

    let body = fs::read_to_string(public_path("testing.html")).unwrap();

    res.send(ResponseType::Html(body))
}
