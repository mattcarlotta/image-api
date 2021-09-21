use reqwest::blocking::get;
use reqwest::header::{CONTENT_TYPE, TRANSFER_ENCODING};
use std::process::Command;
use std::{thread, time::Duration};

#[test]
#[ignore]
fn e2e_integrations() {
    let mut server = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .spawn()
        .expect("Failed to spin up server");

    thread::sleep(Duration::from_secs(5));

    let hello_res = get("http://localhost:5000").unwrap();
    assert!(hello_res.status().is_success());
    assert_eq!(
        hello_res.headers().get(CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let notfound_res = get("http://localhost:5000/123").unwrap();
    assert!(!notfound_res.status().is_success());
    assert_eq!(
        notfound_res.headers().get(CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let img_res = get("http://localhost:5000/placeholder.png").unwrap();
    assert!(img_res.status().is_success());
    assert_eq!(img_res.headers().get(CONTENT_TYPE).unwrap(), "image/png");
    assert_eq!(img_res.headers().get(TRANSFER_ENCODING).unwrap(), "chunked");

    server.kill().expect("Server wasn't running");
}
