use serde_json::{Value, Map};
use regex::Regex;

pub fn compute_matching_sequences(website_to_compile: &mut String, reference_website: &mut String, locations_list: &mut serde_json::Map<String, Value>, sequence_length: usize) {
    let comment_regex = Regex::new(r"<!--(.*?)-->").unwrap();
    *website_to_compile = comment_regex.replace_all(website_to_compile, "").to_string();
    *reference_website = comment_regex.replace_all(reference_website, "").to_string();

    let mut website_to_compile_chars = website_to_compile.chars();
    let mut reference_website_chars = reference_website.chars();
    let website_to_compile_sequences = (0..)
        .map(|_| website_to_compile_chars.by_ref().take(sequence_length).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let reference_website_sequences = (0..)
        .map(|_| reference_website_chars.by_ref().take(sequence_length).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();

    println!("{:?}\n", website_to_compile_sequences);
    println!("{:?}", reference_website_sequences);

    let mut compile_index_pos = 0;
    let mut reference_index_pos = 0;
    let mut insertion_string = String::from("<!--");
    for (index_one, char_one) in website_to_compile_sequences.iter().enumerate() {
        for (index_two, char_two) in reference_website_sequences.iter().enumerate() {
            if char_one == char_two {
                insertion_string.push_str(&format!("{}-{}:{},", reference_index_pos, (reference_index_pos+sequence_length), compile_index_pos));
                locations_list.insert(char_one.to_string(), Value::String(compile_index_pos.to_string()));
                reference_index_pos = reference_index_pos + char_two.chars().count();
                continue;
            }
            reference_index_pos = reference_index_pos + char_two.chars().count();
        }
        compile_index_pos = compile_index_pos + char_one.chars().count();
        reference_index_pos = 0;
    }

    // TODO: Parse out remaining sequence length, if it exists.
    insertion_string.pop();
    insertion_string.push_str("-->");
    reference_website.push_str(&insertion_string);
}

pub fn compile_decentralized_source(website_to_compile: &mut String, locations_list: &mut serde_json::Map<String, Value>) -> String {
    println!("maAaaaaaAAAAgic");
    String::from("return filler")
}
