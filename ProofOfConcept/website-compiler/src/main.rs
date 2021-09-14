use serde_json::{Value};

mod website_fetcher;
mod website_compiler;

#[tokio::main]
async fn main() {
    // Returns a map
    // Can add a k, v pair through locations_list.insert("test this".to_string(), Value::String("testing".to_string()));
    let mut locations_list = website_compiler::get_byte_sequence_locations_list();

    // Returns HTML as string
    let website_one = website_fetcher::get_website("https://facebook.com").await.unwrap();
    let _website_two = website_fetcher::get_website("https://google.com").await.unwrap();
}
