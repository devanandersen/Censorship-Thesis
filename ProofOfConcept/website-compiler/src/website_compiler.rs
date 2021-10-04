use serde_json::Value;
use regex::Regex;

pub fn compute_matching_sequences(website_to_compile: &mut String, reference_website: &mut String, locations_list: &mut serde_json::Map<String, Value>, sequence_length: usize) {
    let comment_regex = Regex::new(r"<!--(.*?)-->").unwrap();
    *website_to_compile = comment_regex.replace_all(website_to_compile, "").to_string();
    *reference_website = comment_regex.replace_all(reference_website, "").to_string();

    let mut compile_index_pos;
    let mut reference_index_pos;
    let mut insertion_string = String::from("<!--");
    let mut last_added_index_pos = 0;
    let mut curr_sequence_length = sequence_length;
    let compile_website_length = website_to_compile.chars().count();

    while last_added_index_pos < compile_website_length && curr_sequence_length >= 1 {
        if curr_sequence_length > (compile_website_length - last_added_index_pos) {
            curr_sequence_length -= 1;
            continue;
        }

        let website_to_compile_chars = website_to_compile.chars();
        let reference_website_chars = reference_website.chars();
        let website_to_compile_sequences = gather_char_sequences(website_to_compile_chars, curr_sequence_length);
        let reference_website_sequences = gather_char_sequences(reference_website_chars, curr_sequence_length);

        compile_index_pos = 0;
        for (_index_one, sequence_one) in website_to_compile_sequences.iter().enumerate() {
            let sequence_one_length = sequence_one.chars().count();
            reference_index_pos = 0;
            for (_index_two, sequence_two) in reference_website_sequences.iter().enumerate() {
                let sequence_two_length = sequence_two.chars().count();
                if sequence_one == sequence_two && compile_index_pos <= compile_website_length {
                    if !locations_list.contains_key(&compile_index_pos.to_string()) {
                        last_added_index_pos += curr_sequence_length;
                        insertion_string.push_str(&format!("{}-{}:{},", reference_index_pos, (reference_index_pos+curr_sequence_length), compile_index_pos));
                        for index_accounted_for in compile_index_pos..compile_index_pos+sequence_one_length {
                            locations_list.insert(index_accounted_for.to_string(), Value::String(sequence_one.to_string()));
                        }
                        reference_index_pos = reference_index_pos + sequence_two_length;
                        continue;
                    }
                }
                reference_index_pos = reference_index_pos + sequence_two_length;
            }
            compile_index_pos = compile_index_pos + sequence_one_length;
        }
        curr_sequence_length -= 1
    }

    insertion_string.pop();
    insertion_string.push_str("-->");
    reference_website.push_str(&insertion_string);
}

pub fn compile_decentralized_source(website_to_reference: &mut String, _locations_list: &mut serde_json::Map<String, Value>) -> String {
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
        let length = ending_char-beginning_char;
        let placement_location: usize = sequence_mappings.last().unwrap().parse::<usize>().unwrap();

        let website_reference_str = website_to_reference.as_str();
        if new_compiled_website_string.len() < placement_location {
            for _ in new_compiled_website_string.len()..placement_location {
                new_compiled_website_string.push(' ');
            }
        }

        if new_compiled_website_string.len() == placement_location {
            new_compiled_website_string.push_str(&website_reference_str[beginning_char..ending_char]);
        } else {
            new_compiled_website_string.replace_range(placement_location..placement_location+length, &website_reference_str[beginning_char..ending_char]);
        }
    }

    new_compiled_website_string.push_str("\n<!-- Compiled using https://github.com/devanandersen/Censorship-Thesis -->");
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
