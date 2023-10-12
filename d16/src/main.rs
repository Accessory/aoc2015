use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

struct Sue {
    children: i32,
    cats: i32,
    samoyeds: i32,
    pomeranians: i32,
    akitas: i32,
    vizslas: i32,
    goldfish: i32,
    trees: i32,
    cars: i32,
    perfumes: i32,
}

impl Sue {
    fn new() -> Self {
        Self {
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1,
        }
    }

    fn insert(&mut self, item: &str, value: i32) {
        match item {
            "children" => {
                self.children = value;
            }
            "cats" => {
                self.cats = value;
            }
            "samoyeds" => {
                self.samoyeds = value;
            }
            "pomeranians" => {
                self.pomeranians = value;
            }
            "akitas" => {
                self.akitas = value;
            }
            "vizslas" => {
                self.vizslas = value;
            }
            "goldfish" => {
                self.goldfish = value;
            }
            "trees" => {
                self.trees = value;
            }
            "cars" => {
                self.cars = value;
            }
            "perfumes" => {
                self.perfumes = value;
            }
            _ => panic!("Should not be here"),
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let sue = Sue {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    let mut sues: Vec<Sue> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut next_sue = Sue::new();

        let split: Vec<&str> = line.split(' ').collect();
        let v1 = &split[2][0..split[2].len() - 1];
        let v1_value: i32 = split[3][0..split[3].len() - 1].parse().unwrap();
        next_sue.insert(v1, v1_value);
        let v2 = &split[4][0..split[4].len() - 1];
        let v2_value: i32 = split[5][0..split[5].len() - 1].parse().unwrap();
        next_sue.insert(v2, v2_value);
        let v3 = &split[6][0..split[6].len() - 1];
        let v3_value: i32 = split[7].parse().unwrap();
        next_sue.insert(v3, v3_value);

        sues.push(next_sue);
    }

    // Solve
    let mut scores = Vec::new();
    for to_check in &sues {
        let mut score = 0;
        if sue.children == to_check.children {
            score += 1;
        }
        if sue.cats == to_check.cats {
            score += 1;
        }
        if sue.samoyeds == to_check.samoyeds {
            score += 1;
        }
        if sue.pomeranians == to_check.pomeranians {
            score += 1;
        }
        if sue.akitas == to_check.akitas {
            score += 1;
        }
        if sue.vizslas == to_check.vizslas {
            score += 1;
        }
        if sue.goldfish == to_check.goldfish {
            score += 1;
        }
        if sue.trees == to_check.trees {
            score += 1;
        }
        if sue.cars == to_check.cars {
            score += 1;
        }
        if sue.perfumes == to_check.perfumes {
            score += 1;
        }
        scores.push(score);
    }

    let mut hightest_score = 0;
    let mut highest_score_id = 0;

    for (i, value) in scores.iter().enumerate() {
        if *value > hightest_score {
            highest_score_id = i;
            hightest_score = *value;
        }
    }

    // Result
    println!("The right Sue is {}", highest_score_id + 1);
}

fn run2(input_file: &str) {
        // Preamble
        let sue = Sue {
            children: 3,
            cats: 7,
            samoyeds: 2,
            pomeranians: 3,
            akitas: 0,
            vizslas: 0,
            goldfish: 5,
            trees: 3,
            cars: 2,
            perfumes: 1,
        };
    
        let mut sues: Vec<Sue> = Vec::new();
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
    
            let mut next_sue = Sue::new();
    
            let split: Vec<&str> = line.split(' ').collect();
            let v1 = &split[2][0..split[2].len() - 1];
            let v1_value: i32 = split[3][0..split[3].len() - 1].parse().unwrap();
            next_sue.insert(v1, v1_value);
            let v2 = &split[4][0..split[4].len() - 1];
            let v2_value: i32 = split[5][0..split[5].len() - 1].parse().unwrap();
            next_sue.insert(v2, v2_value);
            let v3 = &split[6][0..split[6].len() - 1];
            let v3_value: i32 = split[7].parse().unwrap();
            next_sue.insert(v3, v3_value);
    
            sues.push(next_sue);
        }
    
        // Solve
        let mut scores = Vec::new();
        for to_check in &sues {
            let mut score = 0;
            if to_check.children != -1 && sue.children == to_check.children {
                score += 1;
            }
            if to_check.samoyeds != -1 && sue.samoyeds == to_check.samoyeds {
                score += 1;
            }
            if to_check.akitas != -1 && sue.akitas == to_check.akitas {
                score += 1;
            }
            if to_check.vizslas != -1 && sue.vizslas == to_check.vizslas {
                score += 1;
            }
            if to_check.cars != -1 && sue.cars == to_check.cars {
                score += 1;
            }
            if to_check.perfumes != -1 && sue.perfumes == to_check.perfumes {
                score += 1;
            }
            if to_check.trees != -1 && sue.trees < to_check.trees {
                score += 1;
            }
            if to_check.cats != -1 && sue.cats < to_check.cats {
                score += 1;
            }
            if to_check.goldfish != -1 && sue.goldfish > to_check.goldfish {
                score += 1;
            }
            if to_check.pomeranians != -1 && sue.pomeranians > to_check.pomeranians {
                score += 1;
            }
            scores.push(score);
        }
    
        let mut hightest_score = 0;
        let mut highest_score_id = 0;
    
        for (i, value) in scores.iter().enumerate() {
            if *value > hightest_score {
                highest_score_id = i;
                hightest_score = *value;
            }
        }
    
        // Result
        println!("The right Sue is {}", highest_score_id + 1);
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
