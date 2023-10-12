use std::fs;

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let secret = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    loop {
        let to_test = format!("{secret}{result}");

        let md5_result = md5::compute(to_test.as_bytes());
        let md5_compare_result =
            md5_result[0] == 0 && md5_result[1] == 0 && (md5_result[2] & 0xF0) == 0;

        if md5_compare_result {
            println!("Found Hash {:?}", md5_result);
            break;
        }

        result += 1;
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
        // Preamble
        let mut result: usize = 0;

        // Parse
        let secret = fs::read_to_string(input_file).unwrap().trim().to_string();
    
        // Solve
        loop {
            let to_test = format!("{secret}{result}");
    
            let md5_result = md5::compute(to_test.as_bytes());
            let md5_compare_result =
                md5_result[0] == 0 && md5_result[1] == 0 && md5_result[2] == 0;
    
            if md5_compare_result {
                println!("Found Hash {:?}", md5_result);
                break;
            }
    
            result += 1;
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
