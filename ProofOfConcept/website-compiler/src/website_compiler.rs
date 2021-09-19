use serde_json::{Value, Map};

pub fn compute_matching_sequences(website_to_compile: &mut String, reference_website: &mut String, locations_list: &mut serde_json::Map<String, Value>, sequence_length: usize) {
    // TODO: Parse out HTML comments
    *website_to_compile = website_to_compile.replace("\n", "");
    *reference_website = reference_website.replace("\n", "");

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

    let mut reference_index_pos = 0;
    let mut new_compiled_website_source = String::from("");
    for (index_one, char_one) in website_to_compile_sequences.iter().enumerate() {
        for (index_two, char_two) in reference_website_sequences.iter().enumerate() {
            if char_one == char_two {
                locations_list.insert(char_one.to_string(), Value::String(reference_index_pos.to_string()));
                let insertion_string = format!("<{}:{}>", sequence_length, reference_index_pos);
                new_compiled_website_source.push_str(&insertion_string);
                reference_index_pos = index_two + char_two.chars().count();
                continue;
            }
            reference_index_pos = index_two + char_two.chars().count();
        }
        new_compiled_website_source.push_str(char_one);
        reference_index_pos = 0;
    }
    // TODO: Parse out remaining sequence lengths, if they exist.
    *website_to_compile = new_compiled_website_source;
}

pub fn compile_decentralized_source(website_to_compile: &mut String, locations_list: &mut serde_json::Map<String, Value>) -> String {
    println!("maAaaaaaAAAAgic");
    String::from("return filler")
}
