use chrono::prelude::{DateTime, Utc};

pub fn logger(d: DateTime<Utc>, m: &str, p: &str, v: &str, s: &str, r: i64) {
    println!(
        "[{}] - {} {} HTTP/{} {} {}ms",
        d.format("%B %d %Y, %I:%M:%S %P"),
        m,
        p,
        v,
        s,
        r
    );
}
