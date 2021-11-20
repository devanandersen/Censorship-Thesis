use serde_json::Value;
use regex::Regex;

pub fn compute_matching_sequences(candidate_website: &mut String, helper_websites: &mut Vec<String>, locations_list: &mut serde_json::Map<String, Value>, sequence_length: usize) {
    let comment_regex = Regex::new(r"<!--(.*?)-->").unwrap();
    *candidate_website = comment_regex.replace_all(candidate_website, "").trim().to_string();
    let mut insertion_strings = Vec::new();
    for helper_website in helper_websites.iter_mut() {
        let new_helper_website = comment_regex.replace_all(helper_website, "").trim().to_string();
        *helper_website = new_helper_website;
        helper_website.push_str("\n");

        insertion_strings.push(String::from(""));
    }

    let mut curr_sequence_length = sequence_length;
    let compile_website_length = candidate_website.chars().count();

    while curr_sequence_length > 0 {
        let candidate_website_chars = candidate_website.chars();
        let candidate_website_tuple = gather_char_sequences(candidate_website_chars, curr_sequence_length);
        let candidate_website_sequences = candidate_website_tuple.0;
        let candidate_website_indexes = candidate_website_tuple.1;

        for (index, helper_website) in helper_websites.iter().enumerate() {
            let helper_website_chars = helper_website.chars();
            let helper_website_tuple = gather_char_sequences(helper_website_chars, curr_sequence_length);
            let helper_website_sequences = helper_website_tuple.0;
            let helper_website_indexes = helper_website_tuple.1;

            'outer: for (_index_one, sequence_one) in candidate_website_sequences.iter().enumerate() {
                let mut sequence_included = true;
                for index_accounted_for in candidate_website_indexes[_index_one][0]..candidate_website_indexes[_index_one][1] {
                    if !locations_list.contains_key(&index_accounted_for.to_string()) {
                        sequence_included = false;
                    }
                }
                if sequence_included { continue 'outer; }
                for (_index_two, sequence_two) in helper_website_sequences.iter().enumerate() {
                    if sequence_one == sequence_two
                        && candidate_website_indexes[_index_one][0] <= compile_website_length {
                            insertion_strings[index].push_str(&format!("{}-{}:{},", helper_website_indexes[_index_two][0], (helper_website_indexes[_index_two][1]), candidate_website_indexes[_index_one][0]));
                            for index_accounted_for in candidate_website_indexes[_index_one][0]..candidate_website_indexes[_index_one][1] {
                                locations_list.insert(index_accounted_for.to_string(), Value::String(sequence_one.to_string()));
                            }
                            continue 'outer;
                    }
                }
            }
        }
        curr_sequence_length -= 1;
    }


    for (index, helper_website) in helper_websites.iter_mut().enumerate() {
        insertion_strings[index].pop();
        insertion_strings[index].insert_str(0, "<!--");
        insertion_strings[index].push_str("-->");
        helper_website.push_str(&insertion_strings[index]);
    }
}

pub fn compile_decentralized_source(helper_websites: &mut Vec<String>, _locations_list: &mut serde_json::Map<String, Value>) -> String {
    let comment_regex = Regex::new(r"<!--(.*?)-->").unwrap();
    let mut new_compiled_website_string = String::from("");

    for helper_website in helper_websites.iter_mut() {
        let comment_list: Vec<Vec<_>> = comment_regex.captures_iter(helper_website)
            .map(|c| c.iter().map(|m| m.unwrap().as_str()).collect())
            .collect();
        let comment_for_compiling: Vec<&str> = comment_list.last().unwrap().last().unwrap().split(",").collect();

        if comment_for_compiling != [""] {
            for sequence_set in comment_for_compiling {
                let sequence_mappings: Vec<&str> = sequence_set.split(":").collect();
                let sequence_origin_location: Vec<usize> = sequence_mappings.first().unwrap().split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let beginning_char: usize = *sequence_origin_location.first().unwrap();
                let ending_char: usize = *sequence_origin_location.last().unwrap();
                let length = ending_char-beginning_char;
                let placement_location: usize = sequence_mappings.last().unwrap().parse::<usize>().unwrap();

                let helper_website_str = helper_website.as_str();
                if new_compiled_website_string.len() <= placement_location+length {
                    for _ in new_compiled_website_string.len()..placement_location+length {
                        new_compiled_website_string.push(' ');
                    }
                }


                let sequence_to_print = &helper_website_str[beginning_char..ending_char];
                if new_compiled_website_string.len() == placement_location {
                    new_compiled_website_string.push_str(&sequence_to_print);
                } else {
                    new_compiled_website_string.replace_range(placement_location..placement_location+length, &sequence_to_print);
                }
            }
        }
    }

    new_compiled_website_string.push_str("\n<!-- Compiled using https://github.com/devanandersen/Censorship-Thesis -->");
    new_compiled_website_string
}

fn gather_char_sequences(chars_to_split: core::str::Chars, split_length: usize) -> (Vec<std::string::String>, Vec<Vec<usize>>) {
    let website_string = chars_to_split.as_str();
    let mut size_vec = Vec::new();
    let mut output_sequences = vec![];
    let mut iter = chars_to_split;
    let mut pos = 0;

    while pos < website_string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(split_length) {
            len += ch.len_utf8();
        }
        output_sequences.push(website_string[pos..pos + len].to_string());
        size_vec.push(vec![pos, pos+len]);
        pos += len;
    }

    return (output_sequences, size_vec)
}
