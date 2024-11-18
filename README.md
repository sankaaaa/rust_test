# CSV_PARSER_MOSHKOVSKYI
Realisation of basic csv parser
# Technical description
This parser is parsing a csv file.
He will parse file, show it in structured way and check file on correctnes.
# Usage
The result can be used for basic data analytics, here is the example:
We have the test csv file:

"name","age","city"
"John Doe",30,"New York"
"Jane Smith",25,"Los Angeles"
"Sam Brown",22,"Chicago"

Then using our parse_csv method, and our test file, we can have something like this:
fn parse_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => eprintln!("Error reading record: {}", err),
        }
    }
    Ok(())
}

fn main() {
    let file_path = "test.csv";
    if let Err(e) = parse_csv(file_path) {
        eprintln!("Error parsing CSV: {}", e);
    }
}

Where:
ReaderBuilder::new().has_headers(true).from_reader(file) - creates a CSV reader that treats
the first line as header
rdr.records() - reads rows from file
println!("{:?}", record) - prints each row as the tuple of fields

Output:

Record { 0: "Alice", 1: "30", 2: "New York" }
Record { 0: "Bob", 1: "25", 2: "San Francisco" }
Record { 0: "Charlie", 1: "35", 2: "Los Angeles" }
# Grammar
- **WHITESPACE**: Пропуски, табуляція, нові рядки, використовуються для ігнорування пробілів у файлі.
    ```rust
    WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
    ```

- **NEWLINE**: Розриви рядків у стилях Unix і Windows.
    ```rust
    NEWLINE = _{ "\n" | "\r\n" }
    ```

- **csv**: Головне правило для парсингу CSV, що дозволяє обробляти необов'язкові пробіли та нові рядки, а також розділені записи.
    ```rust
    csv = { (WHITESPACE | NEWLINE)* ~ record ~ (NEWLINE ~ record)* ~ (WHITESPACE | NEWLINE)* }
    ```

- **record**: Запис, що складається з одного або більше полів, розділених комами.
    ```rust
    record = { field ~ ("," ~ field)* }
    ```

- **field**: Поле CSV, яке може бути порожнім, в лапках або без лапок.
    ```rust
    field = { empty_field | quoted_field | unquoted_field }
    ```

- **empty_field**: Порожнє поле, що позначається лише комою.
    ```rust
    empty_field = _{ "," }
    ```

- **quoted_field**: Поле в лапках, що дозволяє включати коми або інші лапки.
    ```rust
    quoted_field = _{ "\"" ~ (!"\"" ~ ANY | "\"" ~ "\"")* ~ "\"" }
    ```

- **unquoted_field**: Поле без лапок, яке не містить коми або нових рядків.
    ```rust
    unquoted_field = _{ (!("," | NEWLINE | " ") ~ ANY)+ }- **WHITESPACE**: Represents spaces, tabs, newlines, and carriage returns. Used for ignoring whitespace within the CSV file.
    ```rust
    WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
    ```

- **NEWLINE**: Represents line breaks in Unix and Windows styles.
    ```rust
    NEWLINE = _{ "\n" | "\r\n" }
    ```

- **csv**: The main rule for parsing a CSV file. It allows for optional whitespace or newlines at the beginning, followed by one or more `record` entries separated by newlines, and optional whitespace or newlines after the last record. This rule defines that a CSV file can have leading and trailing line breaks.
    ```rust
    csv = { (WHITESPACE | NEWLINE)* ~ record ~ (NEWLINE ~ record)* ~ (WHITESPACE | NEWLINE)* }
    ```

- **record**: Defines a CSV record, which consists of one or more fields separated by commas. Each field is processed according to the `field` rule.
    ```rust
    record = { field ~ ("," ~ field)* }
    ```

- **field**: A field in the CSV file, which can be empty (`empty_field`), quoted (`quoted_field`), or unquoted (`unquoted_field`).
    ```rust
    field = { empty_field | quoted_field | unquoted_field }
    ```

- **empty_field**: Represents an empty field, indicated by a single comma without any value. This is used when a field is empty.
    ```rust
    empty_field = _{ "," }
    ```

- **quoted_field**: Represents a field enclosed in double quotes (`"`), allowing commas or other quotes to be included in the field. For example, `"value with quotes ""inside"""` will be parsed as a single field. Double quotes inside the field represent a single quote in the final value.
    ```rust
    quoted_field = _{ "\"" ~ (!"\"" ~ ANY | "\"" ~ "\"")* ~ "\"" }
    ```

- **unquoted_field**: Represents a field that is not enclosed in quotes and does not contain commas or newlines. It allows any character except delimiters (`,` , spaces, and `NEWLINE`).
    ```rust
    unquoted_field = _{ (!("," | NEWLINE | " ") ~ ANY)+ }
    ```
# Useful Links
Creates.io = https://crates.io/crates/csv_parser_moshkovskyi
GitHub = https://github.com/IvanKawun/csv_parser_moshkoskyi/tree/master
Docs.rs = https://docs.rs/csv_parser_moshkovskyi/latest/csv_parser_moshkovskyi/