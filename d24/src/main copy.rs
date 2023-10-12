use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;

use utils::get_input_path;

#[derive(Debug, Clone, Copy)]
struct State {
    sum: usize,
    visited: usize,
    deep: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct PState {
    sum: usize,
    visited: usize,
    hits: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct PresentGroup {
    pub count: usize,
    pub quantum_entanglement: usize,
    pub items: usize,
    pub packages: Vec<usize>,
}

fn get_packages(group: usize, packages: &[usize]) -> Vec<usize> {
    let mut rtn: Vec<usize> = Vec::new();
    for i in 0..packages.len() {
        if group & 1 << i != 0 {
            rtn.push(packages[i]);
        }
    }
    rtn
}
fn calc_quantum_entanglement(group: usize, packages: &[usize]) -> usize {
    let mut rtn = 1;
    for i in 0..packages.len() {
        if group & 1 << i != 0 {
            rtn *= packages[i];
        }
    }
    rtn
}

fn run(input_file: &str) {
    // Preamble
    let mut packages: Vec<usize> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let weight = line.parse().unwrap();
        packages.push(weight);
    }
    let combined_weight = packages.iter().sum::<usize>();
    let wpg = combined_weight / 3;
    // Prepare
    let mut groups: Vec<usize> = Vec::new();

    let mut queue = Vec::with_capacity(packages.len());
    for i in 0..packages.len() {
        queue.push(State {
            sum: packages[i],
            visited: 1 << i,
            deep: 0,
        });
    }
    let mut seen: HashSet<usize> = HashSet::new();
    let mut min_deep = usize::MAX;

    while let Some(package) = queue.pop() {
        if !seen.insert(package.visited) {
            continue;
        }

        if min_deep < package.deep {
            continue;
        }

        if package.sum == wpg {
            if min_deep > package.deep {
                groups.clear();
                min_deep = package.deep;
            }

            groups.push(package.visited);
            continue;
        }
        if package.sum > wpg {
            continue;
        }

        for i in 0..packages.len() {
            if package.visited & 1 << i == 0 {
                let new_sum = package.sum + packages[i];
                let new_visited = 1 << i | package.visited;
                let new_deep = package.deep + 1;
                let new_state = State {
                    sum: new_sum,
                    visited: new_visited,
                    deep: new_deep,
                };
                queue.push(new_state);
            }
        }
    }
    let mut package_groups = Vec::new();

    // Solve
    for group in groups {
        let count = group.count_ones();
        let quantum_entanglement = calc_quantum_entanglement(group, &packages);
        let packages = get_packages(group, &packages);
        let pg = PresentGroup {
            count: count as usize,
            items: group,
            quantum_entanglement,
            packages,
        };
        package_groups.push(pg);
    }

    package_groups.sort_unstable();

    // Find Fitting Groups
    let mut result = None;

    'outer: for (i, package_group) in package_groups.iter().enumerate() {
        println!(
            "Try to find fitting groups for {:?}",
            package_group.packages
        );
        println!("This is Group {} out of {}", i + 1, package_groups.len());

        let mut p_queue = Vec::new();
        p_queue.push(PState {
            sum: 20,
            visited: package_group.items,
            hits: 1,
        });

        let mut p_seen = HashSet::new();

        while let Some(p) = p_queue.pop() {
            if !p_seen.insert(p) {
                continue;
            }

            if p.sum > wpg * (p.hits + 1) {
                continue;
            }

            let mut new_hits = p.hits;
            if p.sum == wpg * 2 {
                new_hits = new_hits + 1;
            }

            if p.sum == wpg * 3 {
                println!("Found groups");
                result = Some(i);
                break 'outer;
            }

            for i in 0..packages.len() {
                if p.visited & 1 << i == 0 {
                    let new = PState {
                        sum: p.sum + packages[i],
                        visited: p.visited | 1 << i,
                        hits: new_hits,
                    };
                    p_queue.push(new);
                }
            }
        }
    }

    match result {
        Some(result) => println!(
            "Result is {}. The winning group is: {:?}",
            package_groups[result].quantum_entanglement, package_groups[result].packages
        ),
        None => println!("Did not find a valid Combination"),
    }
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
