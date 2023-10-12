use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Clone, Copy, Debug)]
struct Container {
    id: u32,
    value: i32,
}

fn dfs(
    current_container: Container,
    mut current_liters: i32,
    mut current_seen: u32,
    containers: &Vec<Container>,
    seen: &mut HashSet<u32>,
    capazity: i32,
) -> usize {
    current_seen |= current_container.id;

    if !seen.insert(current_seen) {
        return 0;
    }

    current_liters += current_container.value;

    if current_liters > capazity {
        return 0;
    }

    if current_liters == capazity {
        return 1;
    }

    let mut rtn = 0;

    for container in containers {
        if current_seen & container.id == 0 {
            rtn += dfs(
                *container,
                current_liters,
                current_seen,
                containers,
                seen,
                capazity,
            );
        }
    }

    rtn
}

#[allow(clippy::too_many_arguments)]
fn dfs2(
    current_container: Container,
    mut current_liters: i32,
    mut current_seen: u32,
    containers: &Vec<Container>,
    seen: &mut HashSet<u32>,
    capazity: i32,
    mut deep: u32,
    minimum_deep: &mut u32,
) -> usize {
    current_seen |= current_container.id;
    deep += 1;

    if !seen.insert(current_seen) {
        return 0;
    }

    current_liters += current_container.value;

    if current_liters > capazity {
        return 0;
    }

    if current_liters == capazity {
        *minimum_deep = (*minimum_deep).min(deep);
        return 1;
    }

    let mut rtn = 0;

    for container in containers {
        if current_seen & container.id == 0 {
            rtn += dfs2(
                *container,
                current_liters,
                current_seen,
                containers,
                seen,
                capazity,
                deep,
                minimum_deep,
            );
        }
    }

    rtn
}

#[allow(clippy::too_many_arguments)]
fn dfs3(
    current_container: Container,
    mut current_liters: i32,
    mut current_seen: u32,
    containers: &Vec<Container>,
    seen: &mut HashSet<u32>,
    capazity: i32,
    mut deep: u32,
    maximum_deep: u32,
) -> usize {
    current_seen |= current_container.id;
    deep += 1;

    if deep > maximum_deep {
        return 0;
    }

    if !seen.insert(current_seen) {
        return 0;
    }

    current_liters += current_container.value;

    if current_liters > capazity {
        return 0;
    }

    if current_liters == capazity {
        return 1;
    }

    let mut rtn = 0;

    for container in containers {
        if current_seen & container.id == 0 {
            rtn += dfs3(
                *container,
                current_liters,
                current_seen,
                containers,
                seen,
                capazity,
                deep,
                maximum_deep,
            );
        }
    }

    rtn
}

fn run(input_file: &str) {
    // Preamble
    let mut containers: Vec<Container> = Vec::new();

    #[cfg(test)]
    const LITERS: i32 = 25;
    #[cfg(not(test))]
    const LITERS: i32 = 150;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (id, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let value = line.parse().unwrap();
        let bit_id = 1 << (id + 1);
        containers.push(Container { id: bit_id, value });
    }

    // Solve
    let mut seen: HashSet<u32> = HashSet::new();
    let mut result = 0;
    for container in &containers {
        result += dfs(*container, 0, 0, &containers, &mut seen, LITERS);
    }
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut containers: Vec<Container> = Vec::new();

    #[cfg(test)]
    const LITERS: i32 = 25;
    #[cfg(not(test))]
    const LITERS: i32 = 150;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (id, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let value = line.parse().unwrap();
        let bit_id = 1 << (id + 1);
        containers.push(Container { id: bit_id, value });
    }

    // Solve

    let mut minimut_deep = u32::MAX;
    {
        let mut seen: HashSet<u32> = HashSet::new();
        let deep = 0;
        for container in &containers {
            dfs2(
                *container,
                0,
                0,
                &containers,
                &mut seen,
                LITERS,
                deep,
                &mut minimut_deep,
            );
        }
    }
    let mut seen: HashSet<u32> = HashSet::new();
    let mut result = 0;
    let deep = 0;
    for container in &containers {
        result += dfs3(
            *container,
            0,
            0,
            &containers,
            &mut seen,
            LITERS,
            deep,
            minimut_deep,
        );
    }
    // Result
    println!("Result is {result} and Minimum is {minimut_deep}");
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
