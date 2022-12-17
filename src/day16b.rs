use std::{
    fmt::{Display, Formatter},
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use arrayvec::ArrayVec;
use dashmap::DashMap;
use fnv::{FnvBuildHasher, FnvHashMap};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use petgraph::{
    dot::Dot,
    prelude::UnGraph,
    visit::{EdgeRef, IntoNodeReferences},
};
use rayon::prelude::*;
use regex::Regex;

const INPUT: &str = include_str!("./day16.input");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ValveId(u8);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Valve {
    name: &'static str,
    id: ValveId,
    flow_rate: u8,
    tunnels: Vec<(ValveId, u8)>,
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.flow_rate)
    }
}

type Valves = FnvHashMap<ValveId, Valve>;

fn parse_input() -> (Valves, ValveId, usize) {
    static REGEX: OnceCell<Regex> = OnceCell::new();

    let regex = REGEX.get_or_init(|| {
        regex::Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(, [A-Z]{2})*)$",
        )
        .unwrap()
    });

    let mut useful_valve_id = 1;
    let mut useless_valve_id = 32;

    let mut graph = UnGraph::<Valve, u8>::default();

    let mut name_to_node_index_map = FnvHashMap::default();
    let mut name_to_valve_id_map = FnvHashMap::default();

    let parsed_lines = INPUT
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str();
            let flow_rate: u8 = captures.get(2).unwrap().as_str().parse().unwrap();
            let tunnels = captures.get(3).unwrap().as_str().split(", ").collect_vec();

            let id = if flow_rate == 0 {
                let id = ValveId(useless_valve_id);
                useless_valve_id += 1;
                id
            } else {
                let id = ValveId(useful_valve_id);
                useful_valve_id += 1;
                id
            };

            let node_index = graph.add_node(Valve {
                name,
                id,
                flow_rate,
                tunnels: Vec::new(),
            });

            name_to_node_index_map.insert(name, node_index);
            name_to_valve_id_map.insert(name, id);

            (name, node_index, tunnels)
        })
        .collect_vec();

    // Resolve tunnels

    for (_, node_index, tunnels) in parsed_lines {
        for tunnel in tunnels {
            let tunnel_node_index = *name_to_node_index_map.get(tunnel).unwrap();

            if !graph.find_edge(node_index, tunnel_node_index).is_some() {
                graph.add_edge(node_index, tunnel_node_index, 1);
            }
        }
    }

    // Remove useless valves (flow rate = 0)
    // When a node is removed, we need to connect all its neighbors to each other with a cost of N + 1

    loop {
        let useless_node = graph
            .node_references()
            .find(|(_, valve)| valve.name != "AA" && valve.flow_rate == 0);

        let (node_index, _) = match useless_node {
            None => break,
            Some((node_index, valve)) => (node_index, valve),
        };

        let edge_references = graph.edges(node_index).collect_vec();

        let mut neighbor_edge_weights = FnvHashMap::default();

        for edge_reference in edge_references {
            let weight = edge_reference.weight();
            let neighbor = edge_reference.target();
            neighbor_edge_weights.insert(neighbor, *weight);
        }

        let neighbors = graph.neighbors(node_index).collect_vec();

        for neighbor_1 in neighbors.iter() {
            for neighbor_2 in neighbors.iter() {
                if neighbor_1 == neighbor_2 {
                    continue;
                }

                let edge = graph.find_edge(*neighbor_1, *neighbor_2);

                if edge.is_none() {
                    let weight =
                        neighbor_edge_weights[neighbor_1] + neighbor_edge_weights[neighbor_2];
                    graph.add_edge(*neighbor_1, *neighbor_2, weight);
                }
            }
        }

        graph.remove_node(node_index);
    }

    println!("{}", Dot::with_config(&graph, &[]));

    let aa_id = name_to_valve_id_map["AA"];

    // Convert graph back to a map

    let mut valves = FnvHashMap::default();

    for (node_index, valve) in graph.node_references() {
        let mut valve = valve.clone();

        for edge_reference in graph.edges(node_index) {
            let neighbor = edge_reference.target();
            let neighbor_valve = graph.node_weight(neighbor).unwrap();
            let weight = edge_reference.weight();

            valve.tunnels.push((neighbor_valve.id, *weight));
        }

        valves.insert(valve.id, valve);
    }

    (valves, aa_id, (useful_valve_id - 1) as usize)
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct BitVec32 {
    bits: u32,
}

impl BitVec32 {
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

impl std::fmt::Debug for BitVec32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0b}", self.bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitvec_set_one() {
        let mut bit_vec = BitVec32::new();
        assert_eq!(bit_vec.get(0), false);
        bit_vec.set(0, true);
        assert_eq!(bit_vec.get(0), true);
    }

    #[test]
    fn bitvec_set_many() {
        let mut bit_vec = BitVec32::new();
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
    Move(ValveId, u8),
    Open,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    time: u8,
    open_valves: BitVec32,
    released_pressure: u16,

    current_valve: ValveId,
    helper_current_valve: ValveId,
}

impl State {
    fn create_initial(current_valve: ValveId, helper_current_valve: Option<ValveId>) -> Self {
        let open_valves = BitVec32::new();

        Self {
            time: 1,
            released_pressure: 0,
            helper_current_valve: helper_current_valve.unwrap_or(ValveId(0)),
            current_valve,
            open_valves,
        }
    }

    fn perform_actions(
        &self,
        my_action: Action,
        helper_action: Option<Action>,
        valves: &Valves,
    ) -> Self {
        let mut new_state = self.clone();

        new_state.released_pressure += self.flow_rate(valves);

        match my_action {
            Action::Move(valve_id, _) => {
                new_state.current_valve = valve_id;
            }
            Action::Open => {
                new_state
                    .open_valves
                    .set(self.current_valve.0 as usize, true);
            }
        }

        match helper_action {
            None => {}
            Some(Action::Move(valve_id, _)) => {
                new_state.helper_current_valve = valve_id;
            }
            Some(Action::Open) => {
                new_state
                    .open_valves
                    .set(self.helper_current_valve.0 as usize, true);
            }
        }

        new_state.time += 1;

        new_state
    }

    fn is_valve_open(&self, valve_id: ValveId) -> bool {
        if valve_id.0 >= 32 {
            return false;
        }

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

    fn futher_score_upper_bound(&self, valves: &Valves, max_time: u8) -> u16 {
        let mut max_flow_rate = self.flow_rate(valves);

        for (id, valve) in valves {
            if !self.is_valve_open(*id) {
                max_flow_rate += valve.flow_rate as u16;
            }
        }

        let remaining_time = max_time - self.time;
        self.released_pressure + max_flow_rate * remaining_time as u16
    }
}

fn solve_b(
    valves: &FnvHashMap<ValveId, Valve>,
    useful_valves_count: u8,
    state: State,
    score_cache: Arc<DashMap<State, u16, FnvBuildHasher>>,
    best_score: Arc<AtomicU16>,
) -> u16 {
    const TOTAL_ROUNDS: u8 = 27;

    if state.time == TOTAL_ROUNDS {
        return state.released_pressure;
    }

    if let Some(best_score) = score_cache.get(&state) {
        return *best_score;
    }

    if (state.open_valves.bits.count_ones() as u8) == useful_valves_count {
        // Simulate to end
        return state.released_pressure
            + (TOTAL_ROUNDS - state.time) as u16 * state.flow_rate(valves);
    }

    let upper_bound = state.futher_score_upper_bound(valves, TOTAL_ROUNDS);

    if upper_bound < best_score.load(Ordering::SeqCst) {
        return 0;
    }

    let my_actions = {
        let mut actions = ArrayVec::<Action, 6>::new();

        let current_valve_id = state.current_valve;
        let current_valve = valves.get(&current_valve_id).unwrap();

        if current_valve.flow_rate > 0 && !state.is_valve_open(current_valve_id) {
            actions.push(Action::Open);
        }

        for (tunnel, cost) in &current_valve.tunnels {
            actions.push(Action::Move(*tunnel, *cost));
        }

        actions
    };

    let helper_actions = {
        let mut actions = ArrayVec::<Action, 6>::new();

        let current_valve_id = state.helper_current_valve;
        let current_valve = valves.get(&current_valve_id).unwrap();

        if current_valve.flow_rate > 0 && !state.is_valve_open(current_valve_id) {
            actions.push(Action::Open);
        }

        for (tunnel, cost) in &current_valve.tunnels {
            actions.push(Action::Move(*tunnel, *cost));
        }

        actions
    };

    // Try all combinations of my actions and helper actions
    let max_pressure = my_actions
        .par_iter()
        .flat_map(|my_action| {
            helper_actions
                .par_iter()
                .map(move |helper_action| (my_action, helper_action))
                .filter(|(my_action, helper_action)| {
                    // Prune some useless actions
                    match (my_action, helper_action) {
                        (Action::Move(my_tunnel, _), Action::Move(helper_tunnel, _)) => {
                            my_tunnel != helper_tunnel
                        }
                        (Action::Open, Action::Open)
                            if state.current_valve == state.helper_current_valve =>
                        {
                            false
                        }
                        _ => true,
                    }
                })
        })
        .map(|(my_action, helper_action)| {
            let new_state = state.perform_actions(*my_action, Some(*helper_action), valves);

            let new_state_score = solve_b(
                valves,
                useful_valves_count,
                new_state,
                score_cache.clone(),
                best_score.clone(),
            );
            new_state_score
        })
        .max()
        .unwrap_or(0);

    score_cache.insert(state, max_pressure);

    if max_pressure > best_score.load(Ordering::SeqCst) {
        best_score.store(max_pressure, Ordering::SeqCst);
    }

    max_pressure
}

pub fn day16b() {
    let start_time = Instant::now();

    let (input, initial_valve, useful_valves_count) = parse_input();

    let transition_cache: DashMap<State, u16, FnvBuildHasher> = DashMap::default();
    let transition_cache = Arc::new(transition_cache);
    let initial_state = State::create_initial(initial_valve, Some(initial_valve));
    let best_score = Arc::new(AtomicU16::new(0));

    let (tx, rx) = std::sync::mpsc::channel();

    let thread_best_score = best_score.clone();

    let thread = thread::spawn(move || loop {
        if rx.try_recv().is_ok() {
            break;
        }

        println!("Best score: {}", thread_best_score.load(Ordering::SeqCst));
        thread::sleep(Duration::from_millis(1000));
    });

    let result = solve_b(
        &input,
        useful_valves_count as u8,
        initial_state.clone(),
        transition_cache,
        best_score.clone(),
    );

    tx.send(()).unwrap();
    thread.join().unwrap();

    println!("Day 16b {}", result);
    println!("Time: {:?}", start_time.elapsed());
}
