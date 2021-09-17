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

    let mut existing_file = match File::open(format!("/proc/{}/status", id())) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open process file: {}", e),
    };

    let mut buf = Vec::new();
    match existing_file.read_to_end(&mut buf) {
        Ok(vec) => vec,
        Err(e) => panic!("Unable to read the contents of the process file: {}", e),
    };

    let stats = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => panic!("Unable to strinify the contents of the process file: {}", e),
    };

    let body = stats
        .lines()
        .filter(|l| l.contains("VmSize") || l.contains("VmPeak"))
        .collect::<String>()
        .split_whitespace()
        .filter(|l| l.parse::<i64>().is_ok())
        .collect::<Vec<&str>>()
        .join(",");

    res.set_content("txt").send(ResponseType::Text(body))
}
