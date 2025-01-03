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

fn are_values_safe(v1: i32, v2: i32) -> bool {
    let r = (v1 - v2).abs();
    r >= 1 && r < 4
}

fn is_report_safe(report: &Vec<i32>, depth: i32) -> bool {
    // Base case, if an element was already removed then it's not valid
    if depth > 1 {
        return false;
    }

    let mut is_safe = true;
    let mut report_order: Order = Order::Undefined;
    for i in 0..report.len() - 1 {
        let current_value = report[i];
        let next_value = report[i + 1];

        match report_order {
            Order::Ascending => {
                if current_value > next_value {
                    is_safe = false;
                }
            }

            Order::Descending => {
                if current_value < next_value {
                    is_safe = false;
                }
            }

            // Undefined can mean that the current is equal or that we haven't set the order yet
            Order::Undefined => {
                if current_value < next_value {
                    report_order = Order::Ascending
                }

                if current_value > next_value {
                    report_order = Order::Descending;
                }

                if current_value == next_value {
                    is_safe = false;
                }
            }
        }

        if !are_values_safe(current_value, next_value) {
            is_safe = false;
        }

        // This case includes if there's a duplicate or if the order is broken
        if !is_safe {
            let mut report_clone1 = report.clone();
            let mut report_clone2 = report.clone();

            report_clone1.remove(i);
            report_clone2.remove(i + 1);

            let cond1 = is_report_safe(&report_clone1, depth + 1);
            let cond2 = is_report_safe(&report_clone2, depth + 1);

            if cond1 || cond2 {
                return true;
            }

            if !cond1 && !cond2 {
                return false;
            }
        }
    }

    true
}

fn main() {
    let content = read_file("input");
    let content_vector = string_to_vector(&content);
    let reports = parse_vector(&content_vector);

    are_values_safe(72, 73);
    let mut safe_reports = 0;
    for report in reports {
        let is_safe = is_report_safe(&report, 0);

        if is_safe {
            safe_reports += 1;
        } else {
            println!("{:?}", report);
        }
    }

    println!("Total number of safe reports {}", safe_reports);
}
