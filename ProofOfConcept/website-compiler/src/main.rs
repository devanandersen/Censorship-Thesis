mod resource_manager;
mod website_compiler;

#[tokio::main]
async fn main() {
    let website_url_to_compile = "https://carleton.ca";
    let website_url_to_reference = "https://carleton.ca";
    let sequence_length = 5;

    let mut locations_list = resource_manager::get_byte_sequence_locations_list();

    let mut website_to_compile = resource_manager::get_website(website_url_to_compile).await.unwrap();
    let mut website_to_reference = resource_manager::get_website(website_url_to_reference).await.unwrap();

    website_compiler::compute_matching_sequences(&mut website_to_compile, &mut website_to_reference, &mut locations_list, sequence_length);
    let decentralized_compiled_website = website_compiler::compile_decentralized_source(&mut website_to_reference, &mut locations_list);
    println!("{}", decentralized_compiled_website);
    resource_manager::store_website_file(website_url_to_compile, &website_to_compile);
    resource_manager::store_website_file(website_url_to_reference, &website_to_reference);
    resource_manager::store_website_file("https://recompiled_website.com", &decentralized_compiled_website);
    //resource_manager::_store_locations_list(locations_list);
}
