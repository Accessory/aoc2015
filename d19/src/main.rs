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
    length: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rtn = other.deeps.cmp(&self.deeps);
        if rtn == Ordering::Equal {
            return self.length.cmp(&other.length);
        }
        rtn
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn run2(input_file: &str) {
    // Preamble
    const START_MOLECULE: &str = "e";
    const FILTER_LENGTH:usize = 4;
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

    // Prepare
    let length = molecule.len();
    let start = State {
        molecule,
        deeps: 0,
        length,
    };

    let mut queue: Vec<State> = Vec::new();

    queue.push(start);

    let mut seen: HashSet<String> = HashSet::new();
    // let mut min_length = usize::MAX;

    // Solve
    let result: Option<State>;
    'outer: loop {
        let mut next = Vec::new();
        while let Some(state) = queue.pop() {
            for replace_line in &replace_lines {
                let mut pos = 0;
                while let Some(f) = state.molecule[pos..].find(&replace_line.to) {
                    let mut molecule_copy = state.molecule.clone();
                    molecule_copy.replace_range(
                        pos + f..pos + f + replace_line.to.len(),
                        &replace_line.from,
                    );
                    pos += f + 1;

                    let length = molecule_copy.len();
                    let new_state = State {
                        molecule: molecule_copy,
                        deeps: state.deeps + 1,
                        length,
                    };

                    if length == 1 && new_state.molecule == START_MOLECULE {
                        result = Some(new_state);
                        break 'outer;
                    }
                    if !seen.insert(new_state.molecule.clone()) {
                        continue;
                    }

                    // if min_length > length {
                    //     min_length = length;
                    //     println!("Current min length {min_length}");
                    // }

                    next.push(new_state);
                }
            }
        }
        next.sort_unstable();
        if next.len() > FILTER_LENGTH {
            next.drain(FILTER_LENGTH..);
        }
        queue.clear();
        queue.extend(next.into_iter());
    }

    // Result
    match result {
        Some(r) =>   println!("The minimum steps are {:?}", r.deeps),
        None => println!("Nothing was found!"),
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
