use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

use utils::get_input_path;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pub you_hit_points: i64,
    pub you_mana: i64,
    pub you_damage: i64,
    pub you_heal: i64,
    pub enemy_hit_points: i64,
    pub enemy_damage: i64,
    pub shield: i64,
    pub poisen: i64,
    pub recharge: i64,
    pub round: i64,
    pub mana_spend: i64,
    pub your_turn: bool,
}
impl State {
    fn next(&mut self) -> Self {
        self.shield = self.shield.sub(1).max(0);
        self.poisen = self.poisen.sub(1).max(0);
        self.recharge = self.recharge.sub(1).max(0);
        self.round = self.round.add(1);
        self.your_turn = !self.your_turn;

        *self
    }
}

fn run(input_file: &str) {
    // Preamble
    const YOU_HIT_POINTS: i64 = 50;
    const YOU_MANA: i64 = 500;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|f| f.unwrap()).collect();
    let enemy_hit_points: i64 = lines[0].split(' ').collect::<Vec<&str>>()[2]
        .parse()
        .unwrap();
    let enemy_damage: i64 = lines[1].split(' ').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    // Solve
    let start = State {
        you_hit_points: YOU_HIT_POINTS,
        you_mana: YOU_MANA,
        you_damage: 0,
        you_heal: 0,
        enemy_hit_points,
        enemy_damage,
        shield: 0,
        poisen: 0,
        recharge: 0,
        round: 0,
        mana_spend: 0,
        your_turn: true,
    };

    let mut seen: HashSet<State> = HashSet::new();

    let result: i64 = dfs(start, &mut seen);
    println!("You need at least {result} Mana");
}

fn run2(input_file: &str) {
    // Preamble
    const YOU_HIT_POINTS: i64 = 50;
    const YOU_MANA: i64 = 500;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|f| f.unwrap()).collect();
    let enemy_hit_points: i64 = lines[0].split(' ').collect::<Vec<&str>>()[2]
        .parse()
        .unwrap();
    let enemy_damage: i64 = lines[1].split(' ').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    // Solve
    let start = State {
        you_hit_points: YOU_HIT_POINTS,
        you_mana: YOU_MANA,
        you_damage: 1,
        you_heal: 0,
        enemy_hit_points,
        enemy_damage,
        shield: 0,
        poisen: 0,
        recharge: 0,
        round: 0,
        mana_spend: 0,
        your_turn: true,
    };

    let mut seen: HashSet<State> = HashSet::new();

    let result: i64 = dfs(start, &mut seen);
    println!("You need at least {result} Mana");
}

fn dfs(mut state: State, seen: &mut HashSet<State>) -> i64 {
    if !seen.insert(state) {
        return i64::MAX;
    }

    if state.your_turn {
        state.you_hit_points -= state.you_damage;
    }

    if state.you_hit_points <= 0 {
        return i64::MAX;
    }

    if state.poisen > 0 {
        state.enemy_hit_points -= 3;
    }

    if state.enemy_hit_points <= 0 {
        // println!("{state:?}");
        return state.mana_spend;
    }

    if state.recharge > 0 {
        state.you_mana += 101;
    }

    let next_turns = if state.your_turn {
        execute_your_turn(state)
    } else {
        execute_enemy_turn(state)
    };

    let mut result = i64::MAX;
    for turn in next_turns {
        // println!("{turn:?}");
        result = result.min(dfs(turn, seen));
    }

    result
}

fn execute_your_turn(state: State) -> Vec<State> {
    let mut rtn = Vec::new();

    // state.you_hit_points -= state.you_damage;

    // Magic Missile
    if state.you_mana >= 53 {
        let mut next = state;
        next.you_mana -= 53;
        next.mana_spend += 53;
        next.enemy_hit_points -= 4;
        rtn.push(next.next());
    }
    // Drain
    if state.you_mana >= 73 {
        let mut next = state;
        next.you_mana -= 73;
        next.mana_spend += 73;
        next.enemy_hit_points -= 2;
        next.you_hit_points += 2;
        rtn.push(next.next());
    }
    // Shield
    if state.shield <= 1 && state.you_mana >= 113 {
        let mut next = state;
        next.you_mana -= 113;
        next.mana_spend += 113;
        next.shield = 7;
        rtn.push(next.next());
    }
    // Poison
    if state.poisen <= 1 && state.you_mana >= 173 {
        let mut next = state;
        next.you_mana -= 173;
        next.mana_spend += 173;
        next.poisen = 7;
        rtn.push(next.next());
    }
    // Recharge
    if state.recharge <= 1 && state.you_mana >= 229 {
        let mut next = state;
        next.you_mana -= 229;
        next.mana_spend += 229;
        next.recharge = 6;
        rtn.push(next.next());
    }
    rtn
}

fn execute_enemy_turn(mut state: State) -> Vec<State> {
    state.you_hit_points = if state.shield > 0 {
        state.you_hit_points - (state.enemy_damage - 7) + state.you_heal
    } else {
        state.you_hit_points - state.enemy_damage + state.you_heal
    };
    vec![state.next()]
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
    use crate::run;
    use crate::run2;
    use utils::get_test_input_path;

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
