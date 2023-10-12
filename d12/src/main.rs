use std::fs;

use regex::Regex;
use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: i32 = 0;
    // Parse
    let input = fs::read_to_string(input_file).unwrap().trim().to_string();

    let re = Regex::new(r#"-?\d+"#).unwrap();
    let captures = re.find_iter(&input);

    for capture in captures {
        let number = capture.as_str();
        // println!("{number}");
        result += number.parse::<i32>().unwrap();
    }

    println!("Result is {result}");
}

#[allow(clippy::if_same_then_else)]
fn calc_tree(value: &serde_json::Value) -> i64 {
    let mut result = 0;
    if value.is_array() {
        for inner_value in value.as_array().unwrap() {
            result += calc_tree(inner_value);
        }
    } else if value.is_object() {
        for (_, inner_value) in value.as_object().unwrap() {
            if inner_value.is_i64() {
                result += inner_value.as_i64().unwrap()
            } else if inner_value.is_string() {
                if inner_value.as_str().unwrap() == "red" {
                    return 0;
                }
            } else if inner_value.is_array() {
                result += calc_tree(inner_value);
            } else if inner_value.is_object() {
                result += calc_tree(inner_value);
            }
        }
    } else if value.is_number() {
        result = value.as_i64().unwrap()
    }

    result
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let input = fs::read_to_string(input_file).unwrap().trim().to_string();

    let json = serde_json::from_str(&input).unwrap();

    let result: i64 = calc_tree(&json);

    println!("Result is {result}");
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
