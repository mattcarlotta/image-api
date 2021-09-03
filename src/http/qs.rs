use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    table: HashMap<&'buf str, &'buf str>,
}

impl<'buf> QueryString<'buf> {
    /// Initializes a query string hash table
    pub fn new() -> Self {
        QueryString {
            table: HashMap::new(),
        }
    }

    /// Parses queries from a string slice
    ///
    /// Arguments:
    /// * s : &str
    ///
    /// This will only retain the last instance of a key
    /// where any duplicated keys are overriden
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

    /// Attempts to retriev a vale from the table via key
    ///
    /// Arguments:
    /// * key: &str
    ///
    pub fn get(&self, key: &str) -> Option<&'buf str> {
        self.table.get(key).and_then(move |v| Some(*v))
    }
}
