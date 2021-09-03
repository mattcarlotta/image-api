use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    table: HashMap<&'buf str, &'buf str>,
}

impl<'buf> QueryString<'buf> {
    pub fn new() -> Self {
        QueryString {
            table: HashMap::new(),
        }
    }

    pub fn parse(&mut self, s: &'buf str) {
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            self.table.insert(key, val);
        }
    }

    pub fn get(&self, key: &str) -> Option<&'buf str> {
        self.table.get(key).and_then(move |v| Some(*v))
    }
}
