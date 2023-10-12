use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use utils::get_input_path;

enum ToDo {
    On,
    Toggle,
    Off,
}

fn run(input_file: &str) {
    // Preamble
    const ROW_SIZE: u32 = 1000;
    // let mut lights: [bool; 1_000_000] = [false; 1_000_000];
    let mut lights: Vec<bool> = Vec::new();
    lights.resize(1000 * 1000, false);

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let re = Regex::new(r#"\D+(\d+),(\d+)\D+(\d+),(\d+)"#).unwrap();
        let caputres = re.captures(&line).unwrap();
        let sr: u32 = caputres[1].parse().unwrap();
        let sc: u32 = caputres[2].parse().unwrap();
        let er: u32 = caputres[3].parse::<u32>().unwrap() + 1;
        let ec: u32 = caputres[4].parse::<u32>().unwrap() + 1;

        let mut to_do = ToDo::On;
        if line.starts_with("turn off") {
            to_do = ToDo::Off;
        } else if line.starts_with("toggle") {
            to_do = ToDo::Toggle;
        }

        for y in sr..er {
            for x in sc..ec {
                let pos: usize = (y * ROW_SIZE + x) as usize;
                match to_do {
                    ToDo::On => lights[pos] = true,
                    ToDo::Toggle => lights[pos] = !lights[pos],
                    ToDo::Off => lights[pos] = false,
                }
            }
        }
    }

    // Solve
    // Result
    let mut result: usize = 0;
    for ele in lights {
        if ele {
            result += 1;
        }
    }

    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    const ROW_SIZE: u32 = 1000;
    // let mut lights: [bool; 1_000_000] = [false; 1_000_000];
    let mut lights: Vec<u32> = Vec::new();
    lights.resize(1000 * 1000, 0);

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let re = Regex::new(r#"\D+(\d+),(\d+)\D+(\d+),(\d+)"#).unwrap();
        let caputres = re.captures(&line).unwrap();
        let sr: u32 = caputres[1].parse().unwrap();
        let sc: u32 = caputres[2].parse().unwrap();
        let er: u32 = caputres[3].parse::<u32>().unwrap() + 1;
        let ec: u32 = caputres[4].parse::<u32>().unwrap() + 1;

        let mut to_do = ToDo::On;
        if line.starts_with("turn off") {
            to_do = ToDo::Off;
        } else if line.starts_with("toggle") {
            to_do = ToDo::Toggle;
        }

        for y in sr..er {
            for x in sc..ec {
                let pos: usize = (y * ROW_SIZE + x) as usize;
                match to_do {
                    ToDo::On => lights[pos] += 1,
                    ToDo::Toggle => lights[pos] += 2,
                    ToDo::Off => {
                        if lights[pos] != 0 {
                            lights[pos] -= 1;
                        }
                    }
                }
            }
        }
    }

    // Solve
    // Result
    let mut result: u32 = 0;
    for ele in lights {
        result += ele;
    }

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
