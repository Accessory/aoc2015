use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

struct ReindeerAttributes {
    name: String,
    speed: usize,
    action_time: usize,
    rest_time: usize,
    points: usize,
}

fn run(input_file: &str) {
    // Preamble

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    #[cfg(test)]
    const DURATION: usize = 1000;

    #[cfg(not(test))]
    const DURATION: usize = 2503;

    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();
        let name = split[0].to_string();
        let speed: usize = split[3].parse().unwrap();
        let action_time: usize = split[6].parse().unwrap();
        let rest_time: usize = split[13].parse().unwrap();

        let iteration_time = action_time + rest_time;

        let iterations = DURATION / iteration_time;

        let last_iteration_time = DURATION - iteration_time * iterations;

        let last_speed_time = action_time.min(last_iteration_time);

        let distance = last_speed_time * speed + iterations * speed * action_time;

        println!("Name: {name} Speed: {speed} Action Time: {action_time} Rest Time: {rest_time} Distance: {distance}");
        result = result.max(distance);
    }

    // Solve
    // Result

    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut reindeers: Vec<ReindeerAttributes> = Vec::new();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    #[cfg(test)]
    const DURATION: usize = 1000;

    #[cfg(not(test))]
    const DURATION: usize = 2503;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();
        let name = split[0].to_string();
        let speed: usize = split[3].parse().unwrap();
        let action_time: usize = split[6].parse().unwrap();
        let rest_time: usize = split[13].parse().unwrap();

        let reindeer = ReindeerAttributes {
            name,
            speed,
            action_time,
            rest_time,
            points: 0,
        };
        reindeers.push(reindeer);
    }

    // Solve
    for duration in 1..DURATION + 1 {
        let mut winning_distance = 0;
        let mut winning_reindeers: Vec<usize> = Vec::new();

        for (i, reindeer) in reindeers.iter().enumerate() {
            let iteration_time = reindeer.action_time + reindeer.rest_time;
            let iterations = duration / iteration_time;
            let last_iteration_time = duration - iteration_time * iterations;
            let last_speed_time = reindeer.action_time.min(last_iteration_time);
            let distance = last_speed_time * reindeer.speed
                + iterations * reindeer.speed * reindeer.action_time;

            if winning_distance < distance {
                winning_distance = distance;
                winning_reindeers.clear();
                winning_reindeers.push(i);
                continue;
            }
            if winning_distance == distance {
                winning_reindeers.push(i);
            }
        }

        for i in winning_reindeers {
            reindeers[i].points += 1;
        }
    }

    // Result
    let mut result = 0;
    for reindeer in &reindeers {
        println!("Reindeer: {} - Points: {}", reindeer.name, reindeer.points);
        result = result.max(reindeer.points);
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
