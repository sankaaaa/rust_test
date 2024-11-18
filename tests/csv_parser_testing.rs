use csv_parser::parse_csv;

#[test]
fn test_csv_parsing() {
    let csv_content = r#"
    "name","age","city"
    "John Doe",30,"New York"
    "Jane Smith",25,"Los Angeles"
    "Sam Brown",22,"Chicago"
    "#;

    let result = parse_csv(csv_content);
    assert!(result.is_ok());
}
#[test]
fn test_single_record() {
    let single_record = r#"
    "name","age","city"
    "John Doe",30,"New York"
    "#;

    let result = parse_csv(single_record);
    assert!(result.is_ok());
}

#[test]
fn test_empty_field() {
    let csv_content = r#"
    "name",,"city"
    "John Doe",, "New York"
    "#;

    let result = parse_csv(csv_content);
    assert!(result.is_ok());
}

#[test]
fn test_quoted_field() {
    let quoted_field = r#"
    "name","age","city"
    "John Doe","30","New York"
    "#;

    let result = parse_csv(quoted_field);
    assert!(result.is_ok());
}

#[test]
fn test_unquoted_field() {
    let unquoted_field = r#"
    name,age,city
    John Doe,30,New York
    "#;

    let result = parse_csv(unquoted_field);
    assert!(result.is_ok());
}
