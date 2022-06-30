struct User {
    name: String,
    age: u32,
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.0),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Text("blue".to_string()),
    ];
}
