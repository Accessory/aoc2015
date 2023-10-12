use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

enum EscapeState {
    Inactive,
    Start,
    Hex1,
    Hex2,
}

fn run(input_file: &str) {
    // Preamble

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut char_count: usize = 0;
    let mut memory_count: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        char_count += line.len();

        let mut escape_state = EscapeState::Inactive;
        let mut inner_memory_count = 0;
        let end = line.len() - 1;
        for c in line[1..end].chars() {
            match escape_state {
                EscapeState::Inactive => {
                    if c == '\\' {
                        escape_state = EscapeState::Start;
                    } else {
                        inner_memory_count += 1;
                    }
                }
                EscapeState::Start => {
                    if c == 'x' {
                        escape_state = EscapeState::Hex1;
                    } else {
                        inner_memory_count += 1;
                        escape_state = EscapeState::Inactive;
                    }
                }
                EscapeState::Hex1 => {
                    escape_state = EscapeState::Hex2;
                }
                EscapeState::Hex2 => {
                    escape_state = EscapeState::Inactive;
                    inner_memory_count += 1;
                }
            }
        }
        memory_count += inner_memory_count;
    }

    // Solve
    let result = char_count - memory_count;
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut char_count: usize = 0;
    let mut memory_count: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        char_count += line.len();

        let mut inner_memory_count = 6;
        let end = line.len() - 1;
        for c in line[1..end].chars() {
            if c == '\\' {
                inner_memory_count += 1;
            }
            if c == '"' {
                inner_memory_count += 1;
            }
            inner_memory_count += 1;
        }

        memory_count += inner_memory_count;
    }

    // Solve
    let result = memory_count - char_count;
    // Result
    println!("Result is {}", result);
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
