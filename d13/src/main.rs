use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

// type HappienessType = Rc<RefCell<HashMap<String, i32>>>;
// type N2nType = Rc<RefCell<HashMap<String, HappienessType>>>;

fn calc_happiness(names: &Vec<String>, n2n: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut result = 0;
    for (i, name) in names.iter().enumerate() {
        let before = (i + names.len() - 1) % names.len();
        let after = (i + 1) % names.len();
        let name_before = names.get(before).unwrap();
        let name_after = names.get(after).unwrap();

        // println!("{names:?}");
        // println!("Before: {name_before} - Name: {name} - After: {name_after}");

        let happiness_with = n2n.get(name).unwrap();
        let happiness_before = happiness_with.get(name_before).unwrap();
        let happiness_after = happiness_with.get(name_after).unwrap();

        result += happiness_before + happiness_after;
    }
    result
}

fn run(input_file: &str) {
    // Preamble
    let mut n2n: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut names: Vec<String> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let splits: Vec<&str> = line.split(' ').collect();

        let from = splits[0].to_string();
        let is_lose = splits[2] == "lose";
        let value: i32 = if is_lose {
            -splits[3].parse::<i32>().unwrap()
        } else {
            splits[3].parse::<i32>().unwrap()
        };
        let to_str = splits[10];
        let to = to_str[0..to_str.len()-1].to_string();

        // println!("from: {from} - is_lose: {is_lose} - value: {value} - to: {to}");

        if !n2n.contains_key(&from) {
            n2n.insert(from.clone(), HashMap::new());
            names.push(from.clone())
        }

        let feelings = n2n.get_mut(&from).unwrap();
        feelings.insert(to, value);
    }

    // Solve
    let mut result = calc_happiness(&names, &n2n);
    let mut p: Vec<usize> = Vec::new();

    for i in 0..names.len() {
        p.push(i);
    }
    p.push(usize::MAX);

    let mut i = 1;

    while i < names.len() {
        p[i] -= 1;
        let j = i % 2 * p[i];
        names.swap(j, i);
        // println!("{names:?}");
        result = result.max(calc_happiness(&names, &n2n));
        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
        // Preamble
        let mut n2n: HashMap<String, HashMap<String, i32>> = HashMap::new();
        let mut names: Vec<String> = Vec::new();
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
    
            let splits: Vec<&str> = line.split(' ').collect();
    
            let from = splits[0].to_string();
            let is_lose = splits[2] == "lose";
            let value: i32 = if is_lose {
                -splits[3].parse::<i32>().unwrap()
            } else {
                splits[3].parse::<i32>().unwrap()
            };
            let to_str = splits[10];
            let to = to_str[0..to_str.len()-1].to_string();
    
            // println!("from: {from} - is_lose: {is_lose} - value: {value} - to: {to}");
    
            if !n2n.contains_key(&from) {
                n2n.insert(from.clone(), HashMap::new());
                names.push(from.clone())
            }
    
            let feelings = n2n.get_mut(&from).unwrap();
            feelings.insert(to, value);
        }

        // Prepare
        n2n.insert(String::from("You"), HashMap::new());

        for item in &mut n2n.values_mut() {
            item.insert(String::from("You"), 0);
        }

        
        let you_happiness = n2n.get_mut("You").unwrap();
        for name in &names {
            you_happiness.insert(name.clone(), 0);
        }

        names.push(String::from("You"));
    
        // Solve
        let mut result = calc_happiness(&names, &n2n);
        let mut p: Vec<usize> = Vec::new();
    
        for i in 0..names.len() {
            p.push(i);
        }
        p.push(usize::MAX);
    
        let mut i = 1;
    
        while i < names.len() {
            p[i] -= 1;
            let j = i % 2 * p[i];
            names.swap(j, i);
            // println!("{names:?}");
            result = result.max(calc_happiness(&names, &n2n));
            i = 1;
            while p[i] == 0 {
                p[i] = i;
                i += 1;
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
