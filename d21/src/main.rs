use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

use crate::items::{ARMOR, RINGS, WEAPONS};

mod items;

#[derive(Clone, Copy)]
struct Actor {
    hit_points: i64,
    damage: i64,
    armor: i64,
}

fn simulate_game(
    weapon: &items::Item,
    armor: &items::Item,
    ring1: &items::Item,
    ring2: &items::Item,
    mut enemy: Actor,
    mut p_hit_points: i64,
) -> bool {
    let p_damage = weapon.damage + ring1.damage + ring2.damage;
    let p_armor = armor.armor + ring1.armor + ring2.armor;

    loop {
        // Player attack
        if enemy.armor > p_damage {
            enemy.hit_points -= 1;
        } else {
            enemy.hit_points -= p_damage - enemy.armor;
        }
        if enemy.hit_points <= 0 {
            return true;
        }

        // Enemy attack
        if p_armor > enemy.damage {
            p_hit_points -= 1;
        } else {
            p_hit_points -= enemy.damage - p_armor;
        }
        if p_hit_points <= 0 {
            return false;
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    const YOU_HIT_POINTS: i64 = 100;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|f| f.unwrap()).collect();
    let enemy = Actor {
        hit_points: lines[0].split(' ').collect::<Vec<&str>>()[2]
            .parse()
            .unwrap(),
        damage: lines[1].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
        armor: lines[2].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
    };

    // Solve
    let mut final_weapon = None;
    let mut final_armor = None;
    let mut final_ring1 = None;
    let mut final_ring2 = None;
    let mut result = i64::MAX;
    for weapon in &WEAPONS {
        for armor in &ARMOR {
            for ring1 in &RINGS {
                for ring2 in &RINGS {
                    if ring1 != ring2
                        && simulate_game(weapon, armor, ring1, ring2, enemy, YOU_HIT_POINTS)
                    {
                        let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                        if cost < result {
                            result = cost;
                            final_weapon = Some(weapon);
                            final_armor = Some(armor);
                            final_ring1 = Some(ring1);
                            final_ring2 = Some(ring2);
                        }
                    }
                }
            }
        }
    }

    // Result
    println!(
        "Winning set is {}, {}, {}, {}",
        final_weapon.unwrap().name,
        final_armor.unwrap().name,
        final_ring1.unwrap().name,
        final_ring2.unwrap().name
    );
    let p_damage =
        final_weapon.unwrap().damage + final_ring1.unwrap().armor + final_ring2.unwrap().armor;
    let p_armor =
        final_armor.unwrap().armor + final_ring1.unwrap().armor + final_ring2.unwrap().armor;
    println!("Winning damage {p_damage}, armor {p_armor}");
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    const YOU_HIT_POINTS: i64 = 100;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|f| f.unwrap()).collect();
    let enemy = Actor {
        hit_points: lines[0].split(' ').collect::<Vec<&str>>()[2]
            .parse()
            .unwrap(),
        damage: lines[1].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
        armor: lines[2].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
    };

    // Solve
    let mut final_weapon = None;
    let mut final_armor = None;
    let mut final_ring1 = None;
    let mut final_ring2 = None;
    let mut result = 0;
    for weapon in &WEAPONS {
        for armor in &ARMOR {
            for ring1 in &RINGS {
                for ring2 in &RINGS {
                    if ring1 != ring2
                        && !simulate_game(weapon, armor, ring1, ring2, enemy, YOU_HIT_POINTS)
                    {
                        let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                        if cost > result {
                            result = cost;
                            final_weapon = Some(weapon);
                            final_armor = Some(armor);
                            final_ring1 = Some(ring1);
                            final_ring2 = Some(ring2);
                        }
                    }
                }
            }
        }
    }

    // Result
    println!(
        "Losing set is {}, {}, {}, {}",
        final_weapon.unwrap().name,
        final_armor.unwrap().name,
        final_ring1.unwrap().name,
        final_ring2.unwrap().name
    );
    let p_damage =
        final_weapon.unwrap().damage + final_ring1.unwrap().armor + final_ring2.unwrap().armor;
    let p_armor =
        final_armor.unwrap().armor + final_ring1.unwrap().armor + final_ring2.unwrap().armor;
    println!("Losing damage {p_damage}, armor {p_armor}");
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

    use crate::items::Item;
    use crate::items::ARMOR;
    use crate::items::RINGS;
    use crate::items::WEAPONS;
    use crate::run;
    use crate::run2;
    use crate::simulate_game;
    use crate::Actor;

    #[test]
    fn test_simulation() {
        let enemy = Actor {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        let weapon = Item {
            name: "Dagger",
            cost: 8,
            damage: 4,
            armor: 0,
        };
        let armor = Item {
            name: "Leather",
            cost: 8,
            damage: 0,
            armor: 4,
        };
        let ring1 = Item {
            name: "UseLessTheGreat",
            cost: 0,
            damage: 0,
            armor: 1,
        };
        let ring2 = Item {
            name: "UseLessTheBetter",
            cost: 0,
            damage: 1,
            armor: 0,
        };
        let result = simulate_game(&weapon, &armor, &ring1, &ring2, enemy, 8);
        println!("Result : {result}");
    }

    #[test]
    fn test_simulation_2() {
        let enemy = Actor {
            hit_points: 100,
            damage: 7,
            armor: 2,
        };

        let result = simulate_game(&WEAPONS[4], &ARMOR[0], &RINGS[0], &RINGS[3], enemy, 100);
        println!("Result : {result}");
    }

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
