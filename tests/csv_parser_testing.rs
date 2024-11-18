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

#[test]
fn test_quoted_field_with_escape() {
    let csv_content = r#"
    "name","age","city"
    "John Doe","30","New ""York"""
    "Jane Smith","25","Los ""Angeles"""
    "#;

    let result = parse_csv(csv_content);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed[1][2], "New \"York\""); 
    assert_eq!(parsed[2][2], "Los \"Angeles\"");
}

#[test]
fn test_multi_line_field() {
    let csv_content = r#"
    "name","age","address"
    "John Doe","30","123 Main St
    Apt 4B"
    "Jane Smith","25","456 Elm St
    Suite 12"
    "#;

    let result = parse_csv(csv_content);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed[1][2], "123 Main St\nApt 4B"); 
    assert_eq!(parsed[2][2], "456 Elm St\nSuite 12");
}

#[test]
fn test_combination_of_rules() {
    let csv_content = r#"
    "name","age","bio"
    "John Doe","30","Likes \"coding\" and
    enjoys \"hiking\""
    "Jane Smith","25","\"Traveler\" and
    \"photographer\""
    "#;

    let result = parse_csv(csv_content);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(
        parsed[1][2],
        "Likes \"coding\" and\nenjoys \"hiking\""
    ); 
    assert_eq!(
        parsed[2][2],
        "\"Traveler\" and\n\"photographer\""
    );
}
