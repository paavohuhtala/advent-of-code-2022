use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("./day5.input");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input() -> (Vec<VecDeque<char>>, Vec<Move>) {
    let (stacks, instructions) = INPUT.split_once("\n\n").unwrap();

    let stack_lines = stacks
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (stack_numbers_line, stack_lines) = stack_lines.split_last().unwrap();

    let mut stacks = Vec::new();

    for (i, ch) in stack_numbers_line.iter().enumerate() {
        if !ch.is_digit(10) {
            continue;
        }

        let mut stack = VecDeque::new();

        for line in stack_lines {
            match line.get(i).copied() {
                Some(ch) if ch.is_ascii_alphabetic() => stack.push_back(ch),
                _ => {}
            }
        }

        stacks.push(stack);
    }

    let mut moves = Vec::new();

    let pattern_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in instructions.lines() {
        let captures = pattern_re.captures(line).unwrap();

        let count = captures.get(1).unwrap().as_str().parse().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        moves.push(Move { count, from, to });
    }

    (stacks, moves)
}

fn apply_instruction(stacks: &mut Vec<VecDeque<char>>, instruction: Move, move_whole_stack: bool) {
    let mut moved = stacks[instruction.from]
        .drain(..instruction.count)
        .collect_vec();

    if move_whole_stack {
        moved.reverse();
    }

    for ch in moved {
        stacks[instruction.to].push_front(ch);
    }
}

fn stacks_to_answer(stacks: Vec<VecDeque<char>>) -> String {
    stacks
        .into_iter()
        .map(|stack| stack.front().copied().unwrap())
        .collect()
}

pub fn day5a() {
    let (mut stacks, instructions) = parse_input();

    for instruction in instructions {
        apply_instruction(&mut stacks, instruction, false);
    }

    let top_chars = stacks_to_answer(stacks);

    println!("Day5a: {}", top_chars);
}

pub fn day5b() {
    let (mut stacks, instructions) = parse_input();

    for instruction in instructions {
        apply_instruction(&mut stacks, instruction, true);
    }

    let top_chars = stacks_to_answer(stacks);

    println!("Day5b: {}", top_chars);
}
