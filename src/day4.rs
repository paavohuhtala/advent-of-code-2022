use itertools::Itertools;

const INPUT: &str = include_str!("./day4.input");

fn parse_input() -> Vec<((u32, u32), (u32, u32))> {
    INPUT
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            fn parse_range(s: &str) -> (u32, u32) {
                let (left, right) = s.split_once("-").unwrap();
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                (left, right)
            }

            (parse_range(left), parse_range(right))
        })
        .collect_vec()
}

pub fn day4a() {
    let input = parse_input();

    let fully_contained_count = input
        .into_iter()
        .filter(|((a_l, a_r), (b_l, b_r))| (a_l <= b_l && a_r >= b_r) || (b_l <= a_l && b_r >= a_r))
        .count();

    println!("Day4a: {}", fully_contained_count);
}

pub fn day4b() {
    let input = parse_input();

    let partially_contained_count = input
        .into_iter()
        .filter(|((a_l, a_r), (b_l, b_r))| (a_l <= b_r && b_l <= a_r))
        .count();

    println!("Day4a: {}", partially_contained_count);
}
