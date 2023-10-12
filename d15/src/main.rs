use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[allow(dead_code)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn calc(buckts: &[i32], ingredients: &[Ingredient]) -> u64 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    for (i, num) in buckts.iter().enumerate() {
        capacity += ingredients[i].capacity * *num as i64;
        durability += ingredients[i].durability * *num as i64;
        flavor += ingredients[i].flavor * *num as i64;
        texture += ingredients[i].texture * *num as i64;
    }
    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
        return 0;
    }

    (capacity * durability * flavor * texture) as u64
}

fn calc2(buckts: &[i32], ingredients: &[Ingredient]) -> u64 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;
    for (i, num) in buckts.iter().enumerate() {
        capacity += ingredients[i].capacity * *num as i64;
        durability += ingredients[i].durability * *num as i64;
        flavor += ingredients[i].flavor * *num as i64;
        texture += ingredients[i].texture * *num as i64;
        calories += ingredients[i].calories * *num as i64;
    }
    if calories != 500 || capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
        return 0;
    }

    (capacity * durability * flavor * texture) as u64
}



fn fill_buckets(buckets: &[i32], used_buckets: &mut [i32], needed_space: i32) -> bool {
    let mut remaining = needed_space;
    for i in 0..buckets.len() {
        let bucked_content = min(needed_space - buckets[i], remaining);
        remaining -= bucked_content;
        used_buckets[i] = bucked_content;
    }

    remaining > 0
}

fn add_one_to_bucket(buckets: &mut [i32], max: i32) {
    for bucket in buckets.iter_mut() {
        if *bucket < max {
            *bucket += 1;
            break;
        } else {
            *bucket = 0;
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    const TEASPOONS: i32 = 100;
    let mut ingredients: Vec<Ingredient> = Vec::new();
    let mut buckets: Vec<i32> = Vec::new();
    let mut used_buckets: Vec<i32> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();

        let name = split[0][0..split[0].len() - 1].to_string();
        let capacity: i64 = split[2][0..split[2].len() - 1].parse().unwrap();
        let durability: i64 = split[4][0..split[4].len() - 1].parse().unwrap();
        let flavor: i64 = split[6][0..split[6].len() - 1].parse().unwrap();
        let texture: i64 = split[8][0..split[8].len() - 1].parse().unwrap();
        let calories: i64 = split[10].parse().unwrap();

        ingredients.push(Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });

        buckets.push(0);
        used_buckets.push(0);
    }

    // Solve
    buckets[0] = 0;
    let mut result = 0;

    loop {
        if fill_buckets(&buckets, &mut used_buckets, TEASPOONS) {
            break;
        }
        result = result.max(calc(&used_buckets, &ingredients));
        add_one_to_bucket(&mut buckets, TEASPOONS);
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
        // Preamble
        const TEASPOONS: i32 = 100;
        let mut ingredients: Vec<Ingredient> = Vec::new();
        let mut buckets: Vec<i32> = Vec::new();
        let mut used_buckets: Vec<i32> = Vec::new();
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
            let split: Vec<&str> = line.split(' ').collect();
    
            let name = split[0][0..split[0].len() - 1].to_string();
            let capacity: i64 = split[2][0..split[2].len() - 1].parse().unwrap();
            let durability: i64 = split[4][0..split[4].len() - 1].parse().unwrap();
            let flavor: i64 = split[6][0..split[6].len() - 1].parse().unwrap();
            let texture: i64 = split[8][0..split[8].len() - 1].parse().unwrap();
            let calories: i64 = split[10].parse().unwrap();
    
            ingredients.push(Ingredient {
                name,
                capacity,
                durability,
                flavor,
                texture,
                calories,
            });
    
            buckets.push(0);
            used_buckets.push(0);
        }
    
        // Solve
        buckets[0] = 0;
        let mut result = 0;
    
        loop {
            if fill_buckets(&buckets, &mut used_buckets, TEASPOONS) {
                break;
            }
            result = result.max(calc2(&used_buckets, &ingredients));
            add_one_to_bucket(&mut buckets, TEASPOONS);
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
