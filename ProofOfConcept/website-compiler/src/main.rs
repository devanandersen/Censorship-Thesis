mod resource_manager;
mod website_compiler;

#[tokio::main]
async fn main() {
    let candidate_website_url = "https://carleton.ca";
    let helper_website_url = "https://carleton.ca";
    let sequence_length = 5;

    let mut locations_list = resource_manager::get_byte_sequence_locations_list();

    let mut candidate_website = resource_manager::get_website(candidate_website_url).await.unwrap();
    let mut helper_website = resource_manager::get_website(helper_website_url).await.unwrap();

    website_compiler::compute_matching_sequences(&mut candidate_website, &mut helper_website, &mut locations_list, sequence_length);
    let compiled_candidate_website = website_compiler::compile_decentralized_source(&mut helper_website, &mut locations_list);
    println!("{}", compiled_candidate_website);
    resource_manager::store_website_file(candidate_website_url, &candidate_website);
    resource_manager::store_website_file(helper_website_url, &helper_website);
    resource_manager::store_website_file("https://recompiled_website.com", &compiled_candidate_website);
    //resource_manager::_store_locations_list(locations_list);
}
