use std::fs;

use utils::get_input_path;

// fn calc_presents(house: usize) -> usize {
//     let mut rtn = 0;
//     for i in 1..=house {
//         if house % i == 0 {
//             rtn += i;
//         }
//     }
//     rtn
// }

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let input = fs::read_to_string(input_file).unwrap().trim().to_string();
    let input_value: usize = input.parse::<usize>().unwrap() / 10;

    // Solve
    let mut houses = vec![1; 1_000_000];

    for i in 2..houses.len() {
        for j in (i..houses.len()).step_by(i) {
            houses[j] += i;
        }
    }

    for (i, p) in houses.iter().enumerate() {
        if *p >= input_value {
            result = i;
            break;
        }
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let input = fs::read_to_string(input_file).unwrap().trim().to_string();
    let input_value: usize = input.parse::<usize>().unwrap();

    // Solve
    let mut houses = vec![1; 1_000_000];

    for i in 2..houses.len() {
        let mut break_count = 50;
        for j in (i..houses.len()).step_by(i) {
            houses[j] += i * 11;
            break_count -= 1;
            if break_count == 0 {
                break;
            }
        }
    }

    for (i, p) in houses.iter().enumerate() {
        if *p >= input_value {
            result = i;
            break;
        }
    }

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
