use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Clone, Copy)]
struct State {
    sum: usize,
    visited: usize,
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
    for (i, packages) in packages.iter().enumerate() {
        if group & 1 << i != 0 {
            rtn.push(*packages);
        }
    }
    rtn
}
fn calc_quantum_entanglement(group: usize, packages: &[usize]) -> usize {
    let mut rtn = 1;
    for (i, packages) in packages.iter().enumerate() {
        if group & 1 << i != 0 {
            rtn *= *packages;
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
    for (i, packages) in packages.iter().enumerate() {
        queue.push(State {
            sum: *packages,
            visited: 1 << i,
        });
    }
    let mut seen: HashSet<usize> = HashSet::new();

    while let Some(package) = queue.pop() {
        if !seen.insert(package.visited) {
            continue;
        }

        if package.sum == wpg {
            groups.push(package.visited);
            continue;
        }
        if package.sum > wpg {
            continue;
        }

        for (i, packages) in packages.iter().enumerate() {
            if package.visited & 1 << i == 0 {
                let new_sum = package.sum + *packages;
                let new_visited = 1 << i | package.visited;
                let new_state = State {
                    sum: new_sum,
                    visited: new_visited,
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

    // Result
    package_groups.sort_unstable();

    // Find Fitting Group
    let mut result = usize::MAX;
    let mut result_g2 = usize::MAX;
    let mut result_g3 = usize::MAX;

    'outer: for i1 in 0..package_groups.len() {
        for i2 in i1 + 1..package_groups.len() {
            if i1 & i2 != 0 {
                continue;
            }
            for i3 in i2 + 1..package_groups.len() {
                if i1 & i3 != 0 || i2 & i3 != 0 {
                    continue;
                }
                // let visited =
                //     package_groups[i1].items | package_groups[i2].items | package_groups[i3].items;

                // println!("{visited}, {}", visited.count_ones());

                // if visited.count_ones() as usize == packages.len() {
                    result = i1;
                    result_g2 = i2;
                    result_g3 = i3;
                    break 'outer;
                // }
            }
        }
    }
    match package_groups.get(result) {
        Some(package_group) => println!(
            "Found the groups {:?} - {:?} - {:?}, the Quantum Entanglement is {}",
            package_group.packages,
            package_groups[result_g2].packages,
            package_groups[result_g3].packages,
            package_group.quantum_entanglement
        ),
        None => println!("Could not find any fitting group"),
    }
}

fn run2(input_file: &str) {
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
    let wpg = combined_weight / 4;
    // Prepare
    let mut groups: Vec<usize> = Vec::new();

    let mut queue = Vec::with_capacity(packages.len());
    for (i, packages) in packages.iter().enumerate() {
        queue.push(State {
            sum: *packages,
            visited: 1 << i,
        });
    }
    let mut seen: HashSet<usize> = HashSet::new();

    while let Some(package) = queue.pop() {
        if !seen.insert(package.visited) {
            continue;
        }
        if package.sum == wpg {
            groups.push(package.visited);
            continue;
        }
        if package.sum > wpg {
            continue;
        }

        for (i, packages) in packages.iter().enumerate() {
            if package.visited & 1 << i == 0 {
                let new_sum = package.sum + *packages;
                let new_visited = 1 << i | package.visited;
                let new_state = State {
                    sum: new_sum,
                    visited: new_visited,
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

    // Result
    package_groups.sort_unstable();

    // Find Fitting Group
    let mut result = usize::MAX;
    let mut result_g2 = usize::MAX;
    let mut result_g3 = usize::MAX;
    let mut result_g4 = usize::MAX;

    'outer: for i1 in 0..package_groups.len() {
        for i2 in i1 + 1..package_groups.len() {
            if i1 & i2 != 0 {
                continue;
            }
            for i3 in i2 + 1..package_groups.len() {
                if i1 & i3 != 0 || i2 & i3 != 0 {
                    continue;
                }
                for i4 in i3 + 1..package_groups.len() {
                    if i1 & i4 != 0 || i2 & i4 != 0 || i3 & i4 != 0 {
                        continue;
                    }
                    // let visited = package_groups[i1].items
                    //     | package_groups[i2].items
                    //     | package_groups[i3].items
                    //     | package_groups[i4].items;

                    // println!("{visited}, {}", visited.count_ones());

                    // if visited.count_ones() as usize == packages.len() {
                        result = i1;
                        result_g2 = i2;
                        result_g3 = i3;
                        result_g4 = i4;
                        break 'outer;
                    // }
                }
            }
        }
    }
    match package_groups.get(result) {
        Some(package_group) => println!(
            "Found the groups {:?} - {:?} - {:?} - {:?}, the Quantum Entanglement is {}",
            package_group.packages,
            package_groups[result_g2].packages,
            package_groups[result_g3].packages,
            package_groups[result_g4].packages,
            package_group.quantum_entanglement
        ),
        None => println!("Could not find any fitting group"),
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
