use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[cfg(test)]
fn print_map(map: &Vec<Vec<i8>>) {
    for row in map {
        for column in row {
            let c = if *column == 1 { '#' } else { '.' };
            print!("{c}");
        }
        println!();
    }
}

struct UPoint {
    x: usize,
    y: usize,
}

fn get_points(y: usize, x: usize, max_y: usize, max_x: usize) -> Vec<UPoint> {
    let mut rtn = Vec::with_capacity(8);

    if y != 0 && x != 0 {
        rtn.push(UPoint { x: x - 1, y: y - 1 })
    }
    if y != 0 {
        rtn.push(UPoint { x, y: y - 1 })
    }
    if y != 0 && x != (max_x - 1) {
        rtn.push(UPoint { x: x + 1, y: y - 1 })
    }
    if x != 0 {
        rtn.push(UPoint { x: x - 1, y })
    }
    if x != (max_x - 1) {
        rtn.push(UPoint { x: x + 1, y })
    }
    if y != (max_y - 1) && x != 0 {
        rtn.push(UPoint { x: x - 1, y: y + 1 })
    }
    if y != (max_y - 1) {
        rtn.push(UPoint { x, y: y + 1 })
    }
    if y != (max_y - 1) && x != (max_x - 1) {
        rtn.push(UPoint { x: x + 1, y: y + 1 })
    }
    rtn
}

#[allow(clippy::nonminimal_bool)]
fn on_off_check_with_corner(y: usize, x: usize, map: &[Vec<i8>], max_y: usize, max_x: usize) -> i8 {
    if (x == 0 && (y == 0 || y == max_y - 1)) || (x == (max_x - 1) && (y == 0 || y == max_y - 1)) {
        return 1;
    }

    on_off_check(y, x, map, max_y, max_x)
}
fn on_off_check(y: usize, x: usize, map: &[Vec<i8>], max_y: usize, max_x: usize) -> i8 {
    let points_to_check = get_points(y, x, max_y, max_x);

    let mut neighbors_on = 0;
    for point in points_to_check {
        if map[point.y][point.x] == 1 {
            neighbors_on += 1;
        }
    }

    let is_on = map[y][x] == 1;
    if is_on {
        if neighbors_on == 2 || neighbors_on == 3 {
            1
        } else {
            0
        }
    } else if neighbors_on == 3 {
        1
    } else {
        0
    }
}

fn run(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const STEPS: usize = 5;
    #[cfg(not(test))]
    const STEPS: usize = 100;

    let mut map: Vec<Vec<i8>> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut row = Vec::new();
        for c in line.chars() {
            let e = if c == '#' { 1 } else { 0 };
            row.push(e);
        }
        map.push(row);
    }

    // Prepare
    let map_y = map.len();
    let map_x = map[0].len();

    #[allow(unused_variables)]
    for step in 0..STEPS {
        #[cfg(test)]
        {
            println!("Step: {step}");
            print_map(&map);
        }
        let mut new_map = Vec::with_capacity(map_y);
        for y in 0..map_y {
            let mut new_row = Vec::with_capacity(map_x);
            for x in 0..map_x {
                new_row.push(on_off_check(y, x, &map, map_y, map_x));
            }
            new_map.push(new_row);
        }
        map = new_map;
    }

    // Solve
    // Result
    let mut result: usize = 0;
    for row in map {
        for item in row {
            result += item as usize;
        }
    }

    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const STEPS: usize = 5;
    #[cfg(not(test))]
    const STEPS: usize = 100;

    let mut map: Vec<Vec<i8>> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut row = Vec::new();
        for c in line.chars() {
            let e = if c == '#' { 1 } else { 0 };
            row.push(e);
        }
        map.push(row);
    }

    // Prepare
    let map_y = map.len();
    let map_x = map[0].len();

    #[allow(unused_variables)]
    for step in 0..STEPS {
        #[cfg(test)]
        {
            println!("Step: {step}");
            print_map(&map);
        }
        let mut new_map = Vec::with_capacity(map_y);
        for y in 0..map_y {
            let mut new_row = Vec::with_capacity(map_x);
            for x in 0..map_x {
                new_row.push(on_off_check_with_corner(y, x, &map, map_y, map_x));
            }
            new_map.push(new_row);
        }
        map = new_map;
    }

    // Solve
    // Result
    let mut result: usize = 0;
    for row in map {
        for item in row {
            result += item as usize;
        }
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
