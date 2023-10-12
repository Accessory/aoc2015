use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

trait IsVowel {
    fn is_vowel(&self) -> bool;
}

impl IsVowel for char {
    fn is_vowel(&self) -> bool {
        "aeiou".contains(*self)
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    let forbidden_strings = ["ab", "cd", "pq", "xy"];

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut vowels_count: usize = 0;
        let mut has_letter_twice = false;
        let mut last_letter = '0';
        let mut has_forbidden_string = false;
        for c in line.chars() {
            if c.is_vowel() {
                vowels_count += 1;
            }

            if c == last_letter {
                has_letter_twice = true;
            }

            let to_check = format!("{last_letter}{c}");
            if forbidden_strings.contains(&to_check.as_str()) {
                has_forbidden_string = true;
            }

            last_letter = c;
        }
        if vowels_count >= 3 && has_letter_twice && !has_forbidden_string {
            // println!("{line} is nice" );
            result += 1;
        } else {
            // println!("{line} is naughty");
        }
    }

    // Solve
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut has_double_double = false;
        let mut has_double_char_wg = false;
        let mut skip_double_check = false;

        for (i, c) in line.chars().enumerate() {
            if i == 0 {
                continue;
            }

            if !has_double_double && !skip_double_check {
                let prev_char: char = line.chars().nth(i - 1).unwrap_or('0');
                let double = format!("{prev_char}{c}");
                has_double_double = line[i + 1..].contains(&double);
            } else {
                skip_double_check = false;
            }

            if i > 2 && !has_double_char_wg {
                let prev_char = line.chars().nth(i - 2).unwrap_or('0');
                if prev_char == c {
                    has_double_char_wg = true;
                }
            }
        }
        if has_double_double && has_double_char_wg {
            println!("{line} is nice");
            result += 1;
        } else {
            println!("{line} is naughty. Because: Char in between: {has_double_char_wg} Double double {has_double_double}");
        }
    }

    // Solve
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
