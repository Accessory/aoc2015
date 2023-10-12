use std::fs;

use utils::get_input_path;

const FORBIDDEN_CHARS: [char; 3] = ['i', 'o', 'l'];

fn next_char(c: char) -> char {
    let nc = ((c as u8) + 1) as char;
    if FORBIDDEN_CHARS.contains(&nc) {
        ((nc as u8) + 1) as char
    } else {
        nc
    }
}

fn init_password(current_password: &str) -> String {
    let mut new_pw = current_password.to_string();
    for i in (0..current_password.len()).rev() {
        let current_char = current_password.chars().nth(i).unwrap();
        if FORBIDDEN_CHARS.contains(&current_char) {
            unsafe {
                let next_char = (current_char as u8 + 1) as char;
                new_pw.as_bytes_mut()[i] = next_char as u8;
            }
        }

        // new_pw.replace_range(i..i+1, next_char);
    }

    new_pw
}

fn next_password(current_password: &str) -> String {
    let mut new_pw = current_password.to_string();
    for i in (0..current_password.len()).rev() {
        let current_char = current_password.chars().nth(i).unwrap();
        let next_char = if current_char == 'z' {
            'a'
        } else {
            let tmp = (current_char as u8 + 1) as char;
            if FORBIDDEN_CHARS.contains(&tmp) {
                (tmp as u8 + 1) as char
            } else {
                tmp
            }
        };

        unsafe {
            new_pw.as_bytes_mut()[i] = next_char as u8;
        }
        // new_pw.replace_range(i..i+1, next_char);
        if next_char != 'a' {
            break;
        }
    }

    new_pw
}

fn check_password(password: &str) -> bool {
    let mut streak_count = 0;
    let mut has_streak = false;
    let mut next_streak_char = '0';
    let mut double1 = '0';
    let mut has_double1 = false;
    let mut double2 = '0';
    let mut has_double2 = false;

    for c in password.chars() {
        if FORBIDDEN_CHARS.contains(&c) {
            return false;
        }

        if !has_streak {
            if streak_count == 0 || c == next_streak_char || next_streak_char == '0' {
                streak_count += 1;
            } else {
                streak_count = 1;
            }
            next_streak_char = next_char(c);

            if streak_count == 3 {
                has_streak = true;
            }
        }

        if !has_double1 {
            if double1 == c {
                has_double1 = true;
            } else {
                double1 = c;
            }
        } else if !has_double2 && c != double1 {
            if double2 == c {
                has_double2 = true;
            } else {
                double2 = c;
            }
        }
    }
    has_streak && has_double1 && has_double2
}

fn run(input_file: &str) {
    // Parse
    let input = fs::read_to_string(input_file).unwrap().trim().to_string();

    let mut current_password = init_password(&input);
    loop {
        current_password = next_password(&current_password);
        if check_password(&current_password) {
            break;
        }
    }

    // Solve
    // Result
    println!("Result is {}", current_password);
}

fn run2(input_file: &str) {
    // Parse
    let input = fs::read_to_string(input_file).unwrap().trim().to_string();

    let mut current_password = init_password(&input);
    for _ in 0..2 {
        loop {
            current_password = next_password(&current_password);
            if check_password(&current_password) {
                break;
            }
        }
    }

    // Solve
    // Result
    println!("Result is {}", current_password);
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
