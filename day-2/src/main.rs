use std::cmp::PartialEq;

enum Order {
    Ascending,
    Descending,
    Undefined,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Order::Ascending, Order::Ascending) => true,
            (Order::Descending, Order::Descending) => true,
            (Order::Undefined, Order::Undefined) => true,
            _ => false,
        }
    }
}

fn read_file(file_name: &str) -> String {
    let error_message = format!(
        "An error occurred while trying to reading the file \"{}\".\n",
        file_name
    );

    let result = std::fs::read_to_string(file_name).unwrap_or(error_message);
    result
}

fn string_to_vector(content: &String) -> Vec<String> {
    let mut content_vector: Vec<String> = Vec::new();
    for line in content.lines() {
        content_vector.push(String::from(line));
    }

    content_vector
}

fn parse_vector(content_vector: &Vec<String>) -> Vec<Vec<i32>> {
    let mut parsed_content: Vec<Vec<i32>> = Vec::new();
    for line in content_vector {
        let mut current_line: Vec<i32> = Vec::new();
        let words: Vec<&str> = line.split(" ").collect();
        for word in words {
            let parsed_number: i32 = word.parse().unwrap();
            current_line.push(parsed_number);
        }

        parsed_content.push(current_line);
    }

    parsed_content
}

fn is_vector_ordered(v: &Vec<i32>) -> (bool, Order) {
    let mut order: Order = Order::Undefined;
    let mut is_ordered = true;

    let mut current_value = v[0];
    for i in 1..v.len() {
        let next_value = v[i];

        match order {
            Order::Ascending => {
                if current_value > next_value {
                    is_ordered = false;
                    break;
                }
            }

            Order::Descending => {
                if current_value < next_value {
                    is_ordered = false;
                    break;
                }
            }

            Order::Undefined => {
                if current_value < next_value {
                    order = Order::Ascending
                }
                if current_value > next_value {
                    order = Order::Descending;
                }
            }
        }

        current_value = v[i];
    }

    (is_ordered, order)
}

fn is_report_safe(line: &Vec<i32>) -> bool {
    let (is_valid, order) = is_vector_ordered(&line);

    if !is_valid {
        return false;
    }

    if order == Order::Undefined {
        return true;
    }

    let mut current_value = line[0];
    for i in 1..line.len() {
        let mut level_difference = 0;
        let next_value = line[i];

        if current_value < next_value {
            level_difference = next_value - current_value;
        }
        if current_value > next_value {
            level_difference = current_value - next_value;
        }

        if level_difference > 3 || level_difference < 1 {
            return false;
        }

        current_value = line[i];
    }

    true
}

fn main() {
    let content = read_file("input");
    let content_vector = string_to_vector(&content);
    let reports = parse_vector(&content_vector);

    let mut safe_reports = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        }
    }

    println!("Total number of safe reports {}", safe_reports);
}
