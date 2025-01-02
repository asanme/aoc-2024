#![allow(warnings)]
use std::io::Read;

fn read_file(file_name: &str) -> std::io::Result<String> {
    let result = std::fs::read_to_string(file_name)?;
    Ok(result)
}

fn string_to_vector(file_content: &String) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    for line in file_content.lines() {
        vector.push(String::from(line));
    }

    vector
}

fn divide_list(
    content_vector: &Vec<String>,
    left_list: &mut Vec<String>,
    right_list: &mut Vec<String>,
) {
    for line in content_vector {
        let words: Vec<&str> = line.split("   ").collect();
        left_list.push(String::from(words[0]));
        right_list.push(String::from(words[1]));
    }

    left_list.sort();
    right_list.sort();
}

fn calculate_total_distance(left_vector: &Vec<String>, right_vector: &Vec<String>) -> i32 {
    let mut distance = 0;

    for i in 0..left_vector.len() {
        let left_word: i32 = left_vector[i]
            .parse()
            .expect("An error ocurred while parsing left word");

        let right_word: i32 = right_vector[i]
            .parse()
            .expect("An error ocurred while parsing right word");

        if left_word > right_word {
            distance += left_word - right_word;
        } else {
            distance += right_word - left_word;
        }
    }

    distance
}

fn get_similarity_score(left_list: &Vec<String>, right_list: &Vec<String>) -> i32 {
    let mut similarity_score = 0;

    for left_element in left_list {
        let mut coincidences = 0;

        for right_element in right_list {
            let are_equal = left_element == right_element;

            // Doesn't exist
            if (right_element > left_element) {
                break;
            }

            // We found all coincidences already
            if (coincidences != 0 && !are_equal) {
                break;
            }

            if (are_equal) {
                coincidences += 1;
            }
        }

        let current_value: i32 = left_element.parse().expect("Error parsing element");
        similarity_score += current_value * coincidences;
    }

    similarity_score
}

fn main() {
    let file_name = "input";
    let content = read_file(&file_name).expect("Error loading the file");
    let content_vector = string_to_vector(&content);

    let mut left_vector: Vec<String> = Vec::new();
    let mut right_vector: Vec<String> = Vec::new();

    divide_list(&content_vector, &mut left_vector, &mut right_vector);

    let distance = calculate_total_distance(&mut left_vector, &mut right_vector);
    println!("Total distance: {}", distance);

    let similarity_score = get_similarity_score(&mut left_vector, &mut right_vector);
    println!("Similarity score: {}", similarity_score);
}
