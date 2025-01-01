use std::fs::File;
use std::io::Read;
use std::str::Split;

fn read_file(file_name: &str) -> std::io::Result<String> {
    let result = std::fs::read_to_string(file_name)?;
    Ok(result)
}

fn string_to_vector(file_content: &String) -> Vec<&str> {
    let mut myVec: Vec<&str> = Vec::new();
    for line in file_content.lines() {
        myVec.push(line);
    }

    return myVec;
}

fn calculate_total_distance(left_vector: &mut Vec<&str>, right_vector: &mut Vec<&str>) -> i32 {
    let mut distance = 0;

    for i in 0..left_vector.len() {
        let left_word: i32 = left_vector[i]
            .parse()
            .expect("An error ocurred while parsing left word");

        let right_word: i32 = right_vector[i]
            .parse()
            .expect("An error ocurred while parsing right word");

        if (left_word > right_word) {
            distance += left_word - right_word;
        } else {
            distance += right_word - left_word;
        }
    }

    return distance;
}

fn main() {
    let file_name = "input";
    let content = read_file(&file_name).expect("Error loading the file");
    let content_vector = string_to_vector(&content);

    let mut left_vector: Vec<&str> = Vec::new();
    let mut right_vector: Vec<&str> = Vec::new();

    for line in content_vector {
        let word: Vec<&str> = line.split("   ").collect();
        left_vector.push(&word[0]);
        right_vector.push(&word[1]);
    }

    left_vector.sort();
    right_vector.sort();

    let distance = calculate_total_distance(&mut left_vector, &mut right_vector);

    println!("Total distance: {}", distance);
}
