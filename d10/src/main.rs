use std::fs;


use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble

    // Parse
    let input = fs::read_to_string(input_file).unwrap();

    let mut current_input = input;

    for _ in 0..40 {
        let mut new = String::new();
        let mut to_extend = 'n';
        let mut to_extend_count = 0;
        for c in current_input.chars() {
            if to_extend == c {
                to_extend_count += 1;
            } else if to_extend == 'n' {
                to_extend = c;
                to_extend_count = 1;
                continue;
            } else {
                new.push_str(&to_extend_count.to_string());
                new.push(to_extend);
                to_extend_count = 1;
                to_extend = c;
            }
        }
        new.push_str(&to_extend_count.to_string());
        new.push(to_extend);
        current_input = new;
    }

    // Solve
    // Result
    println!("Result is {}", current_input.len());
}

fn run2(input_file: &str) {
        // Preamble

    // Parse
    let input = fs::read_to_string(input_file).unwrap();

    let mut current_input = input;

    for _ in 0..50 {
        let mut new = String::new();
        let mut to_extend = 'n';
        let mut to_extend_count = 0;
        for c in current_input.chars() {
            if to_extend == c {
                to_extend_count += 1;
            } else if to_extend == 'n' {
                to_extend = c;
                to_extend_count = 1;
                continue;
            } else {
                new.push_str(&to_extend_count.to_string());
                new.push(to_extend);
                to_extend_count = 1;
                to_extend = c;
            }
        }
        new.push_str(&to_extend_count.to_string());
        new.push(to_extend);
        current_input = new;
    }

    // Solve
    // Result
    println!("Result is {}", current_input.len());
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
