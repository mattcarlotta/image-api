use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    table: HashMap<&'buf str, &'buf str>,
}

impl<'buf> QueryString<'buf> {
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
    pub fn parse(&mut self, s: &'buf str) {
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
    pub fn get(&self, key: &str) -> Option<&'buf str> {
        self.table.get(key).copied()
    }
}


#[cfg(test)]
mod test {
    use super::QueryString;

    #[test]
    fn parse_and_get_queries() {
        let mut query = QueryString::new();
        assert_eq!(query.table.len(), 0);

        let queries = "ratio=1&width=100&ratio=500&height=100&pixels=20&resize=40&file=12345&ext=png&ratio=100";
        query.parse(queries);

        // limits queries to first 5
        assert_eq!(query.table.len(), 5);

        // only saves the most recent entry of a duplicated query
        assert_eq!(query.get("ratio"), Some("500"));

        // parses other queries
        assert_eq!(query.get("width"), Some("100"));
        assert_eq!(query.get("height"), Some("100"));
        assert_eq!(query.get("pixels"), Some("20"));
        assert_eq!(query.get("resize"), Some("40"));

        // does not parse additional entries after parsing/adding 5
        assert_eq!(query.get("ext"), None);
    }
} 
