use std::path::Path;
use std::fs::read_to_string;
use serde_json::{Value, Map};


pub fn get_byte_sequence_locations_list() -> serde_json::Map<String, Value> {
    let path = Path::new("locations_list.json");
    // To keep track of sequences loaded over time, we store them all in a file locally.
    // If the file doesn't exist yet, we create it.
    if path.exists() {
        let json_file_string = read_to_string(path).unwrap();
        let locations_list_json: Value = serde_json::from_str(&json_file_string).unwrap();
        let locations_list: Map<String, Value> = locations_list_json.as_object().unwrap().clone();
        locations_list
    } else {
        let locations_list_json = serde_json::json!({});
        let locations_list: Map<String, Value> = locations_list_json.as_object().unwrap().clone();
        locations_list
    }
}
