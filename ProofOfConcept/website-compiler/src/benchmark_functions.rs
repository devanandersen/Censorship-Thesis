use regex::Regex;

// A benchmark worst case runtime for the provided websites and sequence length.
pub fn base_case_compile_timing(candidate_website_length: usize, helper_website_length: usize, _sequence_length: usize) { 
    for _i in 1..candidate_website_length {
        for _j in 1..helper_website_length {
            {}
        }
    }
}

pub fn final_stats_and_proportions(candidate_website: String, helper_website: String, sequence_length: usize) {
    let candidate_length = candidate_website.chars().count();
    let mapped_helper_length = helper_website.chars().count();
    let comment_regex = Regex::new(r"<!--(.*?)-->").unwrap();
    let comment_list: Vec<Vec<_>> = comment_regex.captures_iter(&helper_website)
        .map(|c| c.iter().map(|m| m.unwrap().as_str()).collect())
        .collect();
    let comment_for_compiling: Vec<&str> = comment_list.last().unwrap().last().unwrap().split(",").collect();
    let total_sequences_mapped = comment_for_compiling.len();
    let unmapped_helper_website = comment_regex.replace_all(&helper_website, "").trim().to_string();
    let unmapped_helper_length = unmapped_helper_website.chars().count();

    let mut proportion_vec = vec![0; sequence_length];
    for sequence_set in comment_for_compiling {
        let sequence_mappings: Vec<&str> = sequence_set.split(":").collect();
        let sequence_origin_location: Vec<usize> = sequence_mappings.first().unwrap().split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let beginning_char: usize = *sequence_origin_location.first().unwrap();
        let ending_char: usize = *sequence_origin_location.last().unwrap();
        let mut length = ending_char-beginning_char;
        if length > sequence_length { length = sequence_length; }
        proportion_vec[length-1] += 1;
    }
    println!("- Final Website Compiled from {} mappings:", total_sequences_mapped);
    let mut total_mappings = 0;
    for count in proportion_vec.iter() {
        total_mappings += count;
    }
    for (i, count) in proportion_vec.iter().rev().enumerate() {
        let decimal_percent = (*count as f64 / total_mappings as f64) * 100 as f64;
        println!("\t- {:0>5.2}% of the sequences are of length {}", decimal_percent, (proportion_vec.len()-i));
    }

    let increase_in_space = (mapped_helper_length as f64 / unmapped_helper_length as f64) * 100 as f64;
    let average_sequence_size = candidate_length as f64 / total_mappings as f64;
    println!("- Website Lengths:\n\t- Candidate Website Length: {}\n\t- Helper Website Length without Mappings: {}\n\t- Helper Website Length with Mappings: {}\n\t- Increase in Helper Length: {:.2}%\n\t- Average Sequence Size Found: {:.2}", candidate_length, unmapped_helper_length, mapped_helper_length, increase_in_space, average_sequence_size);
}

