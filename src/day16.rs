use std::{fmt::Formatter, time::Instant};

use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;

const INPUT: &str = include_str!("./day16.input");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ValveId(u8);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Valve {
    name: &'static str,
    id: ValveId,
    flow_rate: u8,
    tunnels: Vec<ValveId>,
}

type Valves = FnvHashMap<ValveId, Valve>;

fn parse_input() -> (FnvHashMap<ValveId, Valve>, ValveId) {
    static REGEX: OnceCell<Regex> = OnceCell::new();

    let regex = REGEX.get_or_init(|| {
        regex::Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(, [A-Z]{2})*)$",
        )
        .unwrap()
    });

    let mut valve_id = 0;
    let mut name_to_id_map = FnvHashMap::default();
    let mut valves = FnvHashMap::default();

    for line in INPUT.lines() {
        let captures = regex.captures(line).unwrap();

        let name = captures.get(1).unwrap().as_str();
        let flow_rate = captures.get(2).unwrap().as_str().parse().unwrap();

        let id = *name_to_id_map.entry(name).or_insert_with(|| {
            let id = ValveId(valve_id);
            valve_id += 1;
            id
        });

        let mut tunnels = Vec::new();

        for tunnel in captures.get(3).unwrap().as_str().split(", ") {
            let id = *name_to_id_map.entry(tunnel).or_insert_with(|| {
                let id = ValveId(valve_id);
                valve_id += 1;
                id
            });

            tunnels.push(id);
        }

        let valve = Valve {
            name,
            id,
            flow_rate,
            tunnels,
        };

        valves.insert(id, valve);
    }

    let aa_id = *name_to_id_map.get("AA").unwrap();

    (valves, aa_id)
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct BitVec64 {
    bits: u64,
}

impl BitVec64 {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn set(&mut self, index: usize, value: bool) {
        if value {
            self.bits |= 1 << index;
        } else {
            self.bits &= !(1 << index);
        }
    }

    fn get(&self, index: usize) -> bool {
        self.bits & (1 << index) != 0
    }

    fn bits_iter(&self) -> impl Iterator<Item = usize> + '_ {
        (0..64).filter(move |index| self.get(*index))
    }
}

impl std::fmt::Debug for BitVec64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0b}", self.bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitvec_set_one() {
        let mut bit_vec = BitVec64::new();
        assert_eq!(bit_vec.get(0), false);
        bit_vec.set(0, true);
        assert_eq!(bit_vec.get(0), true);
    }

    #[test]
    fn bitvec_set_many() {
        let mut bit_vec = BitVec64::new();
        bit_vec.set(0, true);
        bit_vec.set(2, true);
        assert_eq!(bit_vec.get(0), true);
        assert_eq!(bit_vec.get(1), false);
        assert_eq!(bit_vec.get(2), true);
        assert_eq!(bit_vec.get(3), false);
    }
}

#[derive(Debug, Clone, Copy, Hash)]
enum Action {
    Move(ValveId),
    Open,
    Wait,
}

impl Action {
    fn pretty_print(self, valves: &Valves) -> String {
        match self {
            Action::Move(valve_id) => format!("Move to {}", valves[&valve_id].name),
            Action::Open => "Open valve".to_string(),
            Action::Wait => "Wait".to_string(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    time: u8,
    released_pressure: u64,
    current_valve: ValveId,
    open_valves: BitVec64,
}

impl State {
    fn create_initial(current_valve: ValveId) -> Self {
        let open_valves = BitVec64::new();

        Self {
            time: 1,
            released_pressure: 0,
            current_valve,
            open_valves,
        }
    }

    fn perform_action(&self, action: Action, valves: &Valves) -> Self {
        let mut new_state = self.clone();

        new_state.released_pressure += self.flow_rate(valves);

        match action {
            Action::Move(valve_id) => {
                new_state.current_valve = valve_id;
            }
            Action::Open => {
                new_state
                    .open_valves
                    .set(self.current_valve.0 as usize, true);
            }
            Action::Wait => {}
        }

        new_state.time += 1;

        new_state
    }

    fn is_valve_open(&self, valve_id: ValveId) -> bool {
        self.open_valves.get(valve_id.0 as usize)
    }

    fn flow_rate(&self, valves: &Valves) -> u64 {
        let mut flow_rate = 0;

        for (id, valve) in valves {
            if self.is_valve_open(*id) {
                flow_rate += valve.flow_rate as u64;
            }
        }

        flow_rate
    }
}

fn create_useful_valves_mask(valves: &Valves) -> BitVec64 {
    let mut mask = BitVec64::new();

    for (id, valve) in valves {
        if valve.flow_rate > 0 {
            mask.set(id.0 as usize, true);
        }
    }

    mask
}

fn solve(
    valves: &FnvHashMap<ValveId, Valve>,
    state: State,
    score_cache: &mut FnvHashMap<State, (Option<(State, Action)>, i64)>,
) -> i64 {
    static USEFUL_VALVES_MASK: OnceCell<BitVec64> = OnceCell::new();

    // let indent = " ".repeat((state.time - 1) as usize);
    //let flow_rate = state.flow_rate(valves) as i64;

    if state.time == 31 {
        return state.released_pressure as i64;
    }

    let useful_valves_mask = USEFUL_VALVES_MASK.get_or_init(|| create_useful_valves_mask(valves));

    if (state.open_valves.bits ^ useful_valves_mask.bits) == 0 {
        // Simulate to end
        return state.released_pressure as i64
            + (31 - state.time) as i64 * state.flow_rate(valves) as i64;
    }

    if let Some((_, best_score)) = score_cache.get(&state) {
        return *best_score;
    }

    let actions = {
        let mut actions = Vec::new();

        let current_valve = valves.get(&state.current_valve).unwrap();

        if current_valve.flow_rate > 0 && !state.is_valve_open(state.current_valve) {
            actions.push(Action::Open);
        }

        for tunnel in &current_valve.tunnels {
            actions.push(Action::Move(*tunnel));
        }

        if actions.is_empty() {
            actions.push(Action::Wait);
        }

        actions
    };

    let mut max_pressure = i64::MIN;
    let mut best_next_state = None;
    let mut best_action = None;

    /*if state.time == 4 {
        println!("DEBUG: {:?}", state);
        println!("Current valve: {}", valves[&state.current_valve].name);
        for action in actions.iter() {
            println!("{:?}", action.pretty_print(valves));
        }
        println!();
    }*/

    // let mut seen_scores = FnvHashSet::default();

    for action in actions {
        let new_state = state.perform_action(action, valves);

        let new_state_score = solve(valves, new_state.clone(), score_cache);

        /*if !seen_scores.contains(&new_state_score) {
            println!(
                "{}N: {:?} -> {:?} = {}",
                indent, state, action, new_state_score
            );
        } else {
            println!(
                "{}O: {:?} -> {:?} = {}",
                indent, state, action, new_state_score
            );
        }*/

        // seen_scores.insert(new_state_score);

        if new_state_score > max_pressure {
            max_pressure = new_state_score;
            best_next_state = Some(new_state);
            best_action = Some(action);
        }
    }

    // println!();

    score_cache.insert(
        state,
        (
            Some((best_next_state.unwrap(), best_action.unwrap())),
            max_pressure,
        ),
    );

    max_pressure
}

pub fn day16a() {
    let start_time = Instant::now();

    let (input, initial_valve) = parse_input();
    // println!("{:#?}", input);

    let mut transition_cache = FnvHashMap::default();
    let initial_state = State::create_initial(initial_valve);
    let result = solve(&input, initial_state.clone(), &mut transition_cache);

    let mut state = initial_state;
    let mut states = Vec::new();

    while let Some((Some((next_state, action)), _)) = transition_cache.get(&state) {
        states.push((state, action));
        state = next_state.clone();
    }

    let mut released_pressure = 0;

    for (state, action) in states.into_iter() {
        println!("== Minute {} ==", state.time);

        let open_valve_names = state
            .open_valves
            .bits_iter()
            .map(|bit| {
                let valve_id = ValveId(bit as u8);
                let valve = input.get(&valve_id).unwrap();
                valve.name
            })
            .join(", ");

        let flow_rate = state.flow_rate(&input);

        println!(
            "Open valves: {}, flow rate: {}, total: {}",
            open_valve_names, flow_rate, released_pressure
        );

        match action {
            Action::Move(to) => {
                let valve = input.get(&state.current_valve).unwrap();
                let to_valve = input.get(&to).unwrap();
                println!("Move from {} to {}", valve.name, to_valve.name);
            }
            Action::Open => {
                let valve = input.get(&state.current_valve).unwrap();
                println!("Open {}", valve.name);
            }
            Action::Wait => {
                println!("Wait");
            }
        }

        released_pressure += flow_rate as i64;

        println!();
    }

    println!("Day 16a: {}", result);
    println!("Time: {:?}", start_time.elapsed());
}

pub fn day16b() {}
