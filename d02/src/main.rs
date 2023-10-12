use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split('x');
        let l: u32 = split.next().unwrap().parse::<u32>().unwrap();
        let w: u32 = split.next().unwrap().parse::<u32>().unwrap();
        let h: u32 = split.next().unwrap().parse::<u32>().unwrap();

        let lw = l * w;
        let wh = w * h;
        let hl = h * l;

        let min = lw.min(wh).min(hl);

        let partial_result = min + 2 * lw + 2 * wh + 2 * hl;
        result += partial_result;
    }

    // Solve
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split('x');
        let l: u32 = split.next().unwrap().parse::<u32>().unwrap();
        let w: u32 = split.next().unwrap().parse::<u32>().unwrap();
        let h: u32 = split.next().unwrap().parse::<u32>().unwrap();

        let mut sides = vec![l, w, h];
        sides.sort_unstable();
        let s1 = sides[0];
        let s2 = sides[1];

        let partial_result = s1 + s1 + s2 + s2 + (l * w * h);
        result += partial_result;
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
