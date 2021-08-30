use crate::http::{Method, StatusCode};
use chrono::prelude::{DateTime, Utc};

pub fn logger(date: DateTime<Utc>, meth: Method, path: &str, status: StatusCode, res: i64) {
    println!(
        "[{}] - {} {} HTTP/1.1 {} {}ms",
        date.format("%B %d %Y, %I:%M:%S %P"),
        meth,
        path,
        status,
        res
    );
}
