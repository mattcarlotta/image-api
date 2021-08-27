use chrono::prelude::{DateTime, Utc};

pub fn logger(date: DateTime<Utc>, meth: &str, path: &str, vers: &str, stat: &str, res: i64) {
    println!(
        "[{}] - {} {} HTTP/{} {} {}ms",
        date.format("%B %d %Y, %I:%M:%S %P"),
        meth,
        path,
        vers,
        stat,
        res
    );
}
