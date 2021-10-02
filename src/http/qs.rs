use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'a> {
    pub table: HashMap<&'a str, &'a str>,
}

impl<'a> QueryString<'a> {
    /// Initializes a query string hash table
    pub fn new() -> Self {
        QueryString {
            table: HashMap::with_capacity(5),
        }
    }

    /// Parses queries from a string slice
    ///
    /// Arguments:
    /// * s : &str
    ///
    /// This will only retain the last instance of a key
    /// where any duplicated keys are overriden
    pub fn parse(&mut self, s: &'a str) {
        for sub_str in s.split('&') {
            if self.table.len() == 5 {
                return;
            }

            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            self.table.insert(key, val);
        }
    }

    /// Attempts to retrieve a value from the table via key
    ///
    /// Arguments:
    /// * key: &str
    ///
    pub fn get(&self, key: &str) -> Option<&'a str> {
        self.table.get(key).copied()
    }
}
