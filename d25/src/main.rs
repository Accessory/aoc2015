use std::fs;

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble

    // Parse
    let file_string = fs::read_to_string(input_file).unwrap();
    let split: Vec<&str> = file_string.split(' ').collect();

    let row: usize = split[16][0..split[16].len() - 1].parse().unwrap();
    let column: usize = split[18][0..split[18].len() - 1].parse().unwrap();

    let mut start: u64 = 20151125;

    let mut c_row = 1;
    let mut c_column = 1;
    let mut pos = 1;

    loop {
        start *= 252533;
        start %= 33554393;

        c_row -= 1;
        c_column += 1;

        if c_row == 0 {
            pos +=1;
            c_row = pos;
            c_column = 1;
        }

        if c_row == row && c_column == column {
            break;
        }
    }


    // Solve
    // Result
    println!("Result is {}", start);
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
