use itertools::Itertools;

use crate::util::to_set;

const INPUT: &str = include_str!("./day6.input");

fn read_input() -> Vec<char> {
    INPUT.chars().collect()
}

fn find_marker_offset(input: &[char], length: usize) -> usize {
    let (pos, _) = input
        .windows(length)
        .find_position(|window| to_set(*window).len() == length)
        .unwrap();

    pos + length
}

pub fn day6a() {
    let input = read_input();
    let pos = find_marker_offset(&input, 4);
    println!("Day 6a: {}", pos);
}

pub fn day6b() {
    let input = read_input();
    let pos = find_marker_offset(&input, 14);
    println!("Day 6b: {}", pos);
}
