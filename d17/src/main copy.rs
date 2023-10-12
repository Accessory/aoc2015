use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

struct Container {
    id: usize,
    value: i32,
}

fn calc(containers: &Vec<Container>, mut amount: i32, seen: &mut HashSet<Vec<usize>>) -> bool {
    let mut new_seen = Vec::new();
    for i in 0..containers.len() {
        new_seen.push(containers[i].id);
        amount -= containers[i].value;
        if amount == 0 {
            new_seen.sort_unstable();
            return seen.insert(new_seen);
        }
        if amount < 0 {
            return false;
        }
    }

    true
}

fn run(input_file: &str) {
    // Preamble
    let mut containers: Vec<Container> = Vec::new();

    const LITERS: i32 = 25;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (id, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let value = line.parse().unwrap();
        containers.push(Container { id, value });
    }

    let mut seen: HashSet<Vec<usize>> = HashSet::new();

    // Solve
    let mut result = 0;
    if calc(&containers, LITERS,&mut seen) {
        result += 1;
    };

    let mut p: Vec<usize> = Vec::new();

    for i in 0..containers.len() {
        p.push(i);
    }
    p.push(usize::MAX);

    let mut i = 1;

    while i < containers.len() {
        p[i] -= 1;
        let j = i % 2 * p[i];
        containers.swap(j, i);
        if calc(&containers, LITERS, &mut seen) {
            result += 1;
        };
        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }

    // Result
    println!("Result is {}", result);
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
