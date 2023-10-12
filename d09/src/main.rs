use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

// struct TravelOption {
//     location: String,
//     connections: Vec<String>,
// }

#[derive(Clone, Debug)]
struct Connection {
    to: String,
    distance: usize,
}

fn dfs(
    location: String,
    distance: usize,
    mut seen: HashSet<String>,
    travel_map: &HashMap<String, Vec<Connection>>,
) -> usize {
    seen.insert(location.clone());

    let location_connections = travel_map.get(&location).unwrap();

    if seen.len() == (location_connections.len() + 1) {
        return distance;
    }

    let mut return_distance = usize::MAX;
    for lc in location_connections {
        if !seen.contains(&lc.to) {
            let tmp = dfs(
                lc.to.clone(),
                distance + lc.distance,
                seen.clone(),
                travel_map,
            );
            return_distance = return_distance.min(tmp);
        }
    }

    return_distance
}

fn dfs_max(
    location: String,
    distance: usize,
    mut seen: HashSet<String>,
    travel_map: &HashMap<String, Vec<Connection>>,
) -> usize {
    seen.insert(location.clone());

    let location_connections = travel_map.get(&location).unwrap();

    if seen.len() == (location_connections.len() + 1) {
        return distance;
    }

    let mut return_distance = 0;
    for lc in location_connections {
        if !seen.contains(&lc.to) {
            let tmp = dfs_max(
                lc.to.clone(),
                distance + lc.distance,
                seen.clone(),
                travel_map,
            );
            return_distance = return_distance.max(tmp);
        }
    }

    return_distance
}

fn run(input_file: &str) {
    // Preamble
    let mut travel_map: HashMap<String, Vec<Connection>> = HashMap::new();
    let mut locations: HashSet<String> = HashSet::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();
        let from = split.first().unwrap().to_string();
        let to = split.get(2).unwrap().to_string();
        let distance: usize = split.get(4).unwrap().parse().unwrap();
        // println!("From: {from} to: {to} distance: {distance}");

        locations.insert(from.clone());
        locations.insert(to.clone());

        if !travel_map.contains_key(&from) {
            travel_map.insert(from.clone(), Vec::new());
        }

        let item = travel_map.get_mut(&from).unwrap();
        item.push(Connection {
            to: to.clone(),
            distance,
        });

        if !travel_map.contains_key(&to) {
            travel_map.insert(to.clone(), Vec::new());
        }

        let item = travel_map.get_mut(&to).unwrap();
        item.push(Connection {
            to: from,
            distance,
        });
    }

    // Solve
    let mut results: Vec<usize> = Vec::new();
    for start in locations {
        let result = dfs(start, 0, HashSet::new(), &travel_map);
        results.push(result);
    }

    // Result
    let mut result = usize::MAX;
    for item in results {
        result = result.min(item);
    }
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut travel_map: HashMap<String, Vec<Connection>> = HashMap::new();
    let mut locations: HashSet<String> = HashSet::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();
        let from = split.first().unwrap().to_string();
        let to = split.get(2).unwrap().to_string();
        let distance: usize = split.get(4).unwrap().parse().unwrap();
        // println!("From: {from} to: {to} distance: {distance}");

        locations.insert(from.clone());
        locations.insert(to.clone());

        if !travel_map.contains_key(&from) {
            travel_map.insert(from.clone(), Vec::new());
        }

        let item = travel_map.get_mut(&from).unwrap();
        item.push(Connection {
            to: to.clone(),
            distance,
        });

        if !travel_map.contains_key(&to) {
            travel_map.insert(to.clone(), Vec::new());
        }

        let item = travel_map.get_mut(&to).unwrap();
        item.push(Connection {
            to: from,
            distance,
        });
    }

    // Solve
    let mut results: Vec<usize> = Vec::new();
    for start in locations {
        let result = dfs_max(start, 0, HashSet::new(), &travel_map);
        results.push(result);
    }

    // Result
    let mut result = 0;
    for item in results {
        result = result.max(item);
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
