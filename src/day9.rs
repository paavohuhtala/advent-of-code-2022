use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("./day9.input");

struct State {
    tail: (i64, i64),
    head: (i64, i64),
}

impl State {
    pub fn new() -> Self {
        State {
            tail: (0, 0),
            head: (0, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DiagonalDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Cardinal(CardinalDirection),
    Diagonal(DiagonalDirection),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Instruction(CardinalDirection, i64);

fn read_input() -> Vec<Instruction> {
    INPUT
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_ascii_whitespace().next_tuple().unwrap();

            let direction = match direction {
                "U" => CardinalDirection::Up,
                "D" => CardinalDirection::Down,
                "L" => CardinalDirection::Left,
                "R" => CardinalDirection::Right,
                _ => panic!("Invalid direction: {}", direction),
            };

            let distance: i64 = distance.parse().unwrap();
            Instruction(direction, distance)
        })
        .collect_vec()
}

fn move_coord((x, y): (i64, i64), direction: Direction) -> (i64, i64) {
    match direction {
        Direction::Cardinal(CardinalDirection::Up) => (x, y - 1),
        Direction::Cardinal(CardinalDirection::Down) => (x, y + 1),
        Direction::Cardinal(CardinalDirection::Left) => (x - 1, y),
        Direction::Cardinal(CardinalDirection::Right) => (x + 1, y),
        Direction::Diagonal(DiagonalDirection::UpLeft) => (x - 1, y - 1),
        Direction::Diagonal(DiagonalDirection::UpRight) => (x + 1, y - 1),
        Direction::Diagonal(DiagonalDirection::DownLeft) => (x - 1, y + 1),
        Direction::Diagonal(DiagonalDirection::DownRight) => (x + 1, y + 1),
    }
}

fn are_touching((head_x, head_y): (i64, i64), (tail_x, tail_y): (i64, i64)) -> bool {
    let x_diff = head_x - tail_x;
    let y_diff = head_y - tail_y;

    x_diff.abs() <= 1 && y_diff.abs() <= 1
}

fn get_direction_to_move_to(head: (i64, i64), tail: (i64, i64)) -> Option<Direction> {
    if are_touching(head, tail) {
        return None;
    }

    let (head_x, head_y) = head;
    let (tail_x, tail_y) = tail;

    let x_diff = tail_x - head_x;
    let y_diff = tail_y - head_y;

    if y_diff == 0 {
        if x_diff > 0 {
            Some(Direction::Cardinal(CardinalDirection::Left))
        } else {
            Some(Direction::Cardinal(CardinalDirection::Right))
        }
    } else if x_diff == 0 {
        if y_diff > 0 {
            Some(Direction::Cardinal(CardinalDirection::Up))
        } else {
            Some(Direction::Cardinal(CardinalDirection::Down))
        }
    } else if x_diff > 0 && y_diff > 0 {
        Some(Direction::Diagonal(DiagonalDirection::UpLeft))
    } else if x_diff > 0 && y_diff < 0 {
        Some(Direction::Diagonal(DiagonalDirection::DownLeft))
    } else if x_diff < 0 && y_diff > 0 {
        Some(Direction::Diagonal(DiagonalDirection::UpRight))
    } else {
        Some(Direction::Diagonal(DiagonalDirection::DownRight))
    }
}

pub fn day9a() {
    let input = read_input();
    let mut state = State::new();
    let mut tail_visited_positions = HashSet::from([state.tail]);

    for instruction in input {
        let (direction, distance) = (instruction.0, instruction.1);

        for _ in 0..distance {
            state.head = move_coord(state.head, Direction::Cardinal(direction));

            if let Some(direction) = get_direction_to_move_to(state.head, state.tail) {
                state.tail = move_coord(state.tail, direction);
                tail_visited_positions.insert(state.tail);
            }
        }
    }

    println!("Day 9a: {}", tail_visited_positions.len());
}

pub fn day9b() {
    let input = read_input();
    let mut state: Vec<(i64, i64)> = vec![(0, 0); 10];
    let mut tail_visited_positions = HashSet::from([state[0]]);

    for instruction in input {
        let (direction, distance) = (instruction.0, instruction.1);

        for _ in 0..distance {
            state[0] = move_coord(state[0], Direction::Cardinal(direction));

            for i in 1..state.len() {
                let prev = state[i - 1];
                let next = state[i];

                if let Some(direction) = get_direction_to_move_to(prev, next) {
                    state[i] = move_coord(next, direction);

                    if i == state.len() - 1 {
                        tail_visited_positions.insert(state[i]);
                    }
                }
            }
        }
    }

    println!("Day 9b: {}", tail_visited_positions.len());
}
