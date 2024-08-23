use std::fs;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut data = fs::read_to_string(file_name).unwrap();

    if data.is_empty() {
        data = String::from("{}");
    }

    let json: Value = match serde_json::from_str(&data) {
        Ok(result) => result,
        Err(e) => panic!("Failed to read file: {}", e),
    };
    json.as_object().unwrap().clone()
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable tp write to file");
}
