use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

// #[derive(Ord, PartialOrd, PartialEq, Eq)]
// struct Container {
//     id: usize,
//     value: i32,
// }

fn run(input_file: &str) {
    // Preamble
    let mut containers: Vec<i32> = Vec::new();

    #[cfg(test)]
    const LITERS: i32 = 25;
    #[cfg(not(test))]
    const LITERS: i32 = 150;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let value = line.parse().unwrap();
        containers.push(value);
    }

    containers.sort_unstable_by_key(|k| -k);

    // Solve
    let mut result = 0;

    for left in 0..containers.len() {
        for right in (left + 1..containers.len()).rev() {
            let mut liters_left: i32 = LITERS;
            liters_left -= containers[left];
            for current_right in (left + 1..right + 1).rev() {
                liters_left -= containers[current_right];
                if liters_left == 0 {
                    result += 1;
                }
                if liters_left < 0 {
                    break;
                }
            }
        }
    }

    // Result
    println!("Result is {}", result);
}

fn run2(_input_file: &str) {}

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
