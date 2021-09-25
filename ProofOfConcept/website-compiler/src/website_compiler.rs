use serde_json::{Value, Map};
use std::iter::FromIterator;
use regex::Regex;

pub fn compute_matching_sequences(website_to_compile: &mut String, reference_website: &mut String, locations_list: &mut serde_json::Map<String, Value>, sequence_length: usize) {
    let comment_regex = Regex::new(r"<!--(.*?)-->").unwrap();
    *website_to_compile = comment_regex.replace_all(website_to_compile, "").to_string();
    *reference_website = comment_regex.replace_all(reference_website, "").to_string();

    let mut compile_index_pos = 0;
    let mut reference_index_pos = 0;
    let mut insertion_string = String::from("<!--");
    let mut last_added_index_pos = 0;
    let mut curr_sequence_length = sequence_length;

    while last_added_index_pos < website_to_compile.chars().count() {
        if curr_sequence_length > (website_to_compile.chars().count() - last_added_index_pos) {
            curr_sequence_length -= 1;
            continue;
        }

        let mut website_to_compile_chars = website_to_compile[last_added_index_pos..].chars();
        let mut reference_website_chars = reference_website.chars();
        let website_to_compile_sequences = gather_char_sequences(website_to_compile_chars, curr_sequence_length);
        let reference_website_sequences = gather_char_sequences(reference_website_chars, curr_sequence_length);

        reference_index_pos = 0;
        compile_index_pos = last_added_index_pos;
        for (index_one, char_one) in website_to_compile_sequences.iter().enumerate() {
            reference_index_pos = 0;
            for (index_two, char_two) in reference_website_sequences.iter().enumerate() {
                if char_one == char_two && last_added_index_pos < website_to_compile.chars().count() {
                    last_added_index_pos += curr_sequence_length;
                    insertion_string.push_str(&format!("{}-{}:{},", reference_index_pos, (reference_index_pos+curr_sequence_length), compile_index_pos));
                    locations_list.insert(char_one.to_string(), Value::String(compile_index_pos.to_string()));

                    reference_index_pos = reference_index_pos + char_two.chars().count();
                    continue;
                }
                reference_index_pos = reference_index_pos + char_two.chars().count();
            }
            compile_index_pos = compile_index_pos + char_one.chars().count();
        }
        curr_sequence_length -= 1;
    }

    insertion_string.pop();
    insertion_string.push_str("-->");
    reference_website.push_str(&insertion_string);
}

pub fn compile_decentralized_source(website_to_reference: &mut String, locations_list: &mut serde_json::Map<String, Value>) -> String {
    let comment_regex = Regex::new(r"<!--(.*?)-->").unwrap();
    let comment_list: Vec<Vec<_>> = comment_regex.captures_iter(website_to_reference)
        .map(|c| c.iter().map(|m| m.unwrap().as_str()).collect())
        .collect();
    let comment_for_compiling: Vec<&str> = comment_list.last().unwrap().last().unwrap().split(",").collect();
    let mut new_compiled_website_string = String::from("");
    for sequence_set in comment_for_compiling {
        let sequence_mappings: Vec<&str> = sequence_set.split(":").collect();
        let sequence_origin_location: Vec<usize> = sequence_mappings.first().unwrap().split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let beginning_char: usize = *sequence_origin_location.first().unwrap();
        let ending_char: usize = *sequence_origin_location.last().unwrap();
        let placement_location: usize = sequence_mappings.last().unwrap().parse::<usize>().unwrap();

        let website_reference_str = website_to_reference.as_str();
        new_compiled_website_string.insert_str(placement_location, &website_reference_str[beginning_char..ending_char]);
    }

    new_compiled_website_string.push_str("<!-- Compiled using https://github.com/devanandersen/Censorship-Thesis -->");
    new_compiled_website_string
}

fn gather_char_sequences(chars_to_split: core::str::Chars, split_length: usize) -> Vec<std::string::String> {
        let mut mutable_chars_array = chars_to_split;
        let output_sequences = (0..)
            .map(|_| mutable_chars_array.by_ref().take(split_length).collect::<String>())
            .take_while(|s| !s.is_empty())
            .collect::<Vec<_>>();

        return output_sequences
}
