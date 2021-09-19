mod resource_manager;
mod website_compiler;
use serde_json::Value::*;

#[tokio::main]
async fn main() {
    let website_url_to_compile = "https://facebook.com";
    let website_url_to_reference = "https://google.com";
    let sequence_length = 2;

    // Returns a map
    // Can add a k, v pair through locations_list.insert("test this".to_string(), Value::String("testing".to_string()));
    let mut locations_list = resource_manager::get_byte_sequence_locations_list();

    // Returns HTML as string
    let mut website_to_compile = resource_manager::get_website(website_url_to_compile).await.unwrap();
    let mut website_to_reference = resource_manager::get_website(website_url_to_reference).await.unwrap();

    website_compiler::compute_matching_sequences(&mut website_to_compile, &mut website_to_reference, &mut locations_list, sequence_length);
    let decentralized_compiled_website = website_compiler::compile_decentralized_source(&mut website_to_compile, &mut locations_list);
    println!("{}", decentralized_compiled_website);
    resource_manager::store_website_file(website_url_to_compile, &website_to_compile);
    resource_manager::store_website_file(website_url_to_reference, &website_to_reference);
    resource_manager::store_locations_list(locations_list);
}
