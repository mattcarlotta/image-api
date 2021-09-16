use crate::http::{Request, Response, ResponseType};
use crate::utils::file_not_found;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::id;

pub fn memory(_req: Request, res: Response) {
    let in_testing = env::var("testing").unwrap_or_else(|_| "".to_string());
    if in_testing.is_empty() {
        return res.set_status(404).send(file_not_found());
    }

    let pid = format!("/proc/{}/status", id());

    let mut existing_file = match File::open(pid) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open process: {}", e),
    };

    let mut buf = Vec::new();
    match existing_file.read_to_end(&mut buf) {
        Ok(vec) => vec,
        Err(e) => panic!("Unable to read the contents of the image: {}", e),
    };

    let stats = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => panic!("Unable to read the contents of the image: {}", e),
    };

    let body = stats
        .lines()
        .filter(|l| l.contains("VmSize"))
        .collect::<String>()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    res.send(ResponseType::Text(body))
}
