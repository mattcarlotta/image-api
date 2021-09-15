use crate::http::QueryString;

#[test]
fn parse_and_get_queries() {
    let mut query = QueryString::new();
    assert_eq!(query.table.len(), 0);

    let queries =
        "ratio=1&width=100&ratio=500&height=100&pixels=20&resize=40&file=12345&ext=png&ratio=100";
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
