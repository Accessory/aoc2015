use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use utils::get_input_path;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn up(&mut self) {
        self.x += 1;
    }
    fn down(&mut self) {
        self.x -= 1;
    }
    fn left(&mut self) {
        self.y += 1;
    }
    fn right(&mut self) {
        self.y -= 1;
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut result: HashSet<Point2D> = HashSet::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut current_position = Point2D { x: 0, y: 0 };
        result.clear();
        result.insert(current_position);

        for c in line.chars() {
            match c {
                '^' => current_position.up(),
                '>' => current_position.right(),
                '<' => current_position.left(),
                'v' => current_position.down(),
                _ => panic!("Something is wrong"),
            }
            result.insert(current_position);
        }
        // Solve
        // Result
        println!("Result is {}", result.len());
    }
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: HashSet<Point2D> = HashSet::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut current_position = Point2D { x: 0, y: 0 };
        let mut current_position_2 = Point2D { x: 0, y: 0 };
        let mut is_santa = true;
        result.clear();
        result.insert(current_position);

        for c in line.chars() {
            if is_santa {
                match c {
                    '^' => current_position.up(),
                    '>' => current_position.right(),
                    '<' => current_position.left(),
                    'v' => current_position.down(),
                    _ => panic!("Something is wrong"),
                }
                result.insert(current_position);
            } else {
                match c {
                    '^' => current_position_2.up(),
                    '>' => current_position_2.right(),
                    '<' => current_position_2.left(),
                    'v' => current_position_2.down(),
                    _ => panic!("Something is wrong"),
                }
                result.insert(current_position_2);
            }
            is_santa = !is_santa;
        }
        // Solve
        // Result
        println!("Result is {}", result.len());
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
