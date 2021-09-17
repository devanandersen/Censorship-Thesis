mod resource_manager;
mod website_compiler;
use serde_json::Value::*;

#[tokio::main]
async fn main() {
    let website_url_one = "https://facebook.com";
    let website_url_two = "https://google.com";

    // Returns a map
    // Can add a k, v pair through locations_list.insert("test this".to_string(), Value::String("testing".to_string()));
    let mut locations_list = resource_manager::get_byte_sequence_locations_list();
    locations_list.insert("test this".to_string(), String("Testing".to_string()));

    // Returns HTML as string
    let mut website_one = resource_manager::get_website(website_url_one).await.unwrap();
    let mut website_two = resource_manager::get_website(website_url_two).await.unwrap();

    website_compiler::compute_matching_sequences(&mut website_one, &mut website_two);
    resource_manager::store_website_file(website_url_one, &website_one);
    resource_manager::store_website_file(website_url_two, &website_two);
    resource_manager::store_locations_list(locations_list);
}
