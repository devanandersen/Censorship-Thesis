mod resource_manager;
mod website_compiler;
mod benchmark_functions;
use std::time::Instant;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let candidate_website_url = "https://facebook.com";
    let helper_website_url = "https://google.com";
    let sequence_length = 5;

    let mut locations_list = resource_manager::get_byte_sequence_locations_list();

    let mut candidate_website = resource_manager::get_website(candidate_website_url).await.unwrap();
    let mut helper_website = resource_manager::get_website(helper_website_url).await.unwrap();

    if args.contains(&String::from("bench")) {
        println!("Running Benchmark Timing...\n---------------------------------------");
        let benchmark_time_start = Instant::now();
        benchmark_functions::base_case_compile_timing(candidate_website.chars().count(), helper_website.chars().count(), sequence_length);
        let benchmark_time_end = benchmark_time_start.elapsed();
        println!("Benchmark worst case execution time:\n\t- Seconds: {}\n\t- Milliseconds: {}\n", benchmark_time_end.as_secs(), benchmark_time_end.as_millis());
    }

    println!("Running Sequence Algorithm...\n---------------------------------------");
    let compile_time_start = Instant::now();
    website_compiler::compute_matching_sequences(&mut candidate_website, &mut helper_website, &mut locations_list, sequence_length);
    let compile_time_end = compile_time_start.elapsed();
    println!("Compilation execution time:\n\t- Seconds: {}\n\t- Milliseconds {}\n", compile_time_end.as_secs(), compile_time_end.as_millis());

    println!("Constructing Website from Mapping...\n---------------------------------------");
    let compiled_candidate_website = website_compiler::compile_decentralized_source(&mut helper_website, &mut locations_list);
    resource_manager::store_website_file(candidate_website_url, &candidate_website);
    resource_manager::store_website_file(helper_website_url, &helper_website);
    resource_manager::store_website_file("https://recompiled_website.com", &compiled_candidate_website);
    //resource_manager::_store_locations_list(locations_list);
    benchmark_functions::final_stats_and_proportions(candidate_website, helper_website, sequence_length);
    println!("\nCompilation complete - compiled website stored at ./website_store/recompiled_website.com.html");
}
