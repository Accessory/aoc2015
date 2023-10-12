use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut floor = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for c in line.chars() {
            if c == '(' {
                floor += 1;
            } else {
                floor -= 1;
            }
        }
    }

    println!("TThe right floor is {}", floor);
}

fn run2(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut floor = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut result = 0;

        for (i, c) in line.chars().enumerate() {
            if c == '(' {
                floor += 1;
            } else {
                floor -= 1;
            }
            if floor == -1 {
                result = i + 1;
                break;
            }
        }

        println!("The right floor comes after {} chars", result);
    }
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
