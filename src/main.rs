use json_deserializer::parse_ui;
use serde_json::json;
mod json_deserializer;
mod ui_parser;
mod unit;
mod style;
mod color;
mod edge;
mod corner;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    if parse_ui().is_ok() {
        println!("DONE");    
    }

    // if typed_example().is_ok() {
    //     println!("first phone number: {}", john["phones"][0]);

    //     // Convert to a string of JSON and print it out
    //     println!("{}", john.to_string());    
    // }
}