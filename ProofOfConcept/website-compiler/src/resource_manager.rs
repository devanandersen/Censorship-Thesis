use std::error::Error;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::create_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use serde_json::{Value, Map};

pub async fn get_website(url: &str) -> Result<String, Box<dyn Error>> {
    let website_path = format!("./website_store/{}.html", &url[8..]);
    let path = Path::new(&website_path);
    // To simulate websites adding tags, we store HTML locally and update it as the program runs.
    // This is done in individual files stored under the website name
    if path.exists() {
        Ok(read_to_string(path).unwrap())
    } else {
        let res = reqwest::get(url)
            .await?
            .text()
            .await?;

        Ok(res.as_str().to_string())
    }
}

pub fn store_website_file(url: &str, website_string: &str) {
    if !Path::new("website_store/").exists() {
        create_dir("./website_store/").expect("Unable to create directory");
    }

    let website_path = format!("website_store/{}.html", &url[8..]);
    if Path::new(&website_path).exists() {
        std::fs::remove_file(&website_path).map_err(|err| println!("{:?}", err)).ok();
    }

    let mut file = File::create(website_path).expect("Create failed");
    file.write_all(website_string.as_bytes()).expect("Unable to write file");
}

pub fn get_byte_sequence_locations_list() -> serde_json::Map<String, Value> {
    let path = Path::new("locations_list.json");

    if path.exists() {
        let json_file_string = read_to_string(path).unwrap();
        println!("{}", json_file_string);
        let locations_list_json: Value = serde_json::from_str(&json_file_string).unwrap();
        let locations_list: Map<String, Value> = locations_list_json.as_object().unwrap().clone();
        locations_list
    } else {
        let locations_list_json = serde_json::json!({});
        let locations_list: Map<String, Value> = locations_list_json.as_object().unwrap().clone();
        locations_list
    }
}

pub fn _store_locations_list(locations_list: serde_json::Map<String, Value>) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("locations_list.json")
        .unwrap();

    serde_json::to_writer(&file, &locations_list).map_err(|err| println!("{:?}", err)).ok();
}

