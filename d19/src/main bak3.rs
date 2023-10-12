use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

struct ReplaceLine {
    from: String,
    to: String,
}

fn run(input_file: &str) {
    // Preamble
    let mut molecule = String::new();
    let mut replace_lines = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut on_last_line = false;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if on_last_line {
            molecule = line;
            break;
        }

        if line.is_empty() {
            on_last_line = true;
            continue;
        }

        let split: Vec<&str> = line.split(' ').collect();
        let from = split.first().unwrap().to_string();
        let to = split.get(2).unwrap().to_string();
        let rl = ReplaceLine { from, to };

        replace_lines.push(rl);
    }

    // Solve
    let mut result_set: HashSet<String> = HashSet::new();

    for replace_line in replace_lines {
        let mut pos = 0;
        while let Some(f) = molecule[pos..].find(&replace_line.from) {
            let mut molecule_copy = molecule.clone();
            molecule_copy
                .replace_range(pos + f..pos + f + replace_line.from.len(), &replace_line.to);
            result_set.insert(molecule_copy);
            pos += f + 1;
        }
    }

    // Result
    let result = result_set.len();

    println!("Result is {}", result);
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    molecule: String,
    deeps: usize,
    distance: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rtn = self.distance.cmp(&other.distance);
        if rtn == Ordering::Equal {
            return self.deeps.cmp(&other.deeps);
        }
        rtn
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

fn run2(input_file: &str) {
    // Preamble
    const START_MOLECULE: &str = "e";
    let mut molecule = String::new();
    let mut replace_lines: Vec<ReplaceLine> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut on_last_line = false;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if on_last_line {
            molecule = line;
            break;
        }

        if line.is_empty() {
            on_last_line = true;
            continue;
        }

        let split: Vec<&str> = line.split(' ').collect();
        let from = split.first().unwrap().to_string();
        let to = split.get(2).unwrap().to_string();
        let rl = ReplaceLine { from, to };

        replace_lines.push(rl);
    }

    // Prepare
    let start = State {
        molecule: molecule,
        deeps: 0,
        distance: i32::MAX,
    };

    // let mut queue: BinaryHeap<State> = BinaryHeap::new();
    // let mut queue: Vec<State> = Vec::new();
    // queue.push(start);
    let mut seen: HashSet<String> = HashSet::new();
    let mut min_distince: i32 = i32::MAX;

    // Solve

    let result = dfs(
        start,
        &replace_lines,
        &mut seen,
        START_MOLECULE,
        &mut min_distince,
    );

    // Result

    println!("The minimum steps are {}", result.unwrap().deeps);
}

fn dfs(
    state: State,
    replace_lines: &Vec<ReplaceLine>,
    seen: &mut HashSet<String>,
    molecule: &str,
    min_distince: &mut i32,
) -> Option<State> {
    for replace_line in replace_lines {
        let mut pos = 0;
        let mut lives = 10;
        while let Some(f) = state.molecule[pos..].find(&replace_line.to) {
            let mut molecule_copy = state.molecule.clone();
            molecule_copy
                .replace_range(pos + f..pos + f + replace_line.to.len(), &replace_line.from);
            pos += f + 1;

            let distance = molecule_copy.len() as i32 - molecule.len() as i32;

            let new_state = State {
                molecule: molecule_copy,
                deeps: state.deeps + 1,
                distance: distance,
            };

            if lives == 0 {
                break;
            }

            if !seen.insert(new_state.molecule.clone()) {
                continue;
            }

            if molecule.len() == new_state.molecule.len()  && state.molecule == molecule {
                return Some(new_state);
            }

            if distance < 0 && state.deeps > 230 {
                continue;
            }

            if *min_distince > distance {
                *min_distince = distance;
                println!("Current min distnance {min_distince}");
            }
            lives -= 1;

            if let Some(rtn) = dfs(new_state, replace_lines, seen, molecule, min_distince) {
                return Some(rtn);
            }
        }
    }
    None
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
    use utils::get_test_input_2_path;
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
        let input_path = get_test_input_2_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
