use std::{fmt::Formatter, sync::Arc, time::Instant};

use arrayvec::ArrayVec;
use dashmap::DashMap;
use fnv::{FnvBuildHasher, FnvHashMap};
use once_cell::sync::OnceCell;
use rayon::prelude::*;
use regex::Regex;

const INPUT: &str = include_str!("./day16.input");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ValveId(u8);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Valve {
    name: &'static str,
    id: ValveId,
    flow_rate: u16,
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
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    time: u8,
    released_pressure: u16,
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
        }

        new_state.time += 1;

        new_state
    }

    fn is_valve_open(&self, valve_id: ValveId) -> bool {
        self.open_valves.get(valve_id.0 as usize)
    }

    fn flow_rate(&self, valves: &Valves) -> u16 {
        let mut flow_rate = 0;

        for (id, valve) in valves {
            if self.is_valve_open(*id) {
                flow_rate += valve.flow_rate as u16;
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
    score_cache: Arc<DashMap<State, u16, FnvBuildHasher>>,
) -> u16 {
    static USEFUL_VALVES_MASK: OnceCell<BitVec64> = OnceCell::new();

    if state.time == 31 {
        return state.released_pressure;
    }

    let useful_valves_mask = USEFUL_VALVES_MASK.get_or_init(|| create_useful_valves_mask(valves));

    if (state.open_valves.bits ^ useful_valves_mask.bits) == 0 {
        // Simulate to end
        return state.released_pressure + (31 - state.time) as u16 * state.flow_rate(valves);
    }

    if let Some(best_score) = score_cache.get(&state) {
        return *best_score;
    }

    let actions = {
        let mut actions = ArrayVec::<Action, 6>::new();

        let current_valve = valves.get(&state.current_valve).unwrap();

        if current_valve.flow_rate > 0 && !state.is_valve_open(state.current_valve) {
            actions.push(Action::Open);
        }

        for tunnel in &current_valve.tunnels {
            actions.push(Action::Move(*tunnel));
        }

        actions
    };

    //let mut max_pressure = u16::MIN;

    let max_pressure = actions
        .par_iter()
        .copied()
        .map(|action| {
            let new_state = state.perform_action(action, valves);
            let new_state_score = solve(valves, new_state.clone(), score_cache.clone());
            new_state_score
        })
        .max()
        .unwrap();

    score_cache.insert(state, max_pressure);

    max_pressure
}

pub fn day16a() {
    let start_time = Instant::now();

    let (input, initial_valve) = parse_input();

    let transition_cache: DashMap<State, u16, FnvBuildHasher> = DashMap::default();
    let transition_cache = Arc::new(transition_cache);
    let initial_state = State::create_initial(initial_valve);
    let result = solve(&input, initial_state.clone(), transition_cache);

    println!("Day 16a: {}", result);
    println!("Time: {:?}", start_time.elapsed());
}

pub fn day16b() {}
