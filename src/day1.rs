use itertools::Itertools;


const INPUT: &str = include_str!("./day1.input");

pub fn day1a() {
  let max_calories = INPUT.split("\n\n").map(|group| group.lines().map(|line| line.parse::<u32>().unwrap()).sum::<u32>()).max().unwrap();
  println!("Day 1a: {}", max_calories);
}

pub fn day1b() {
  let top3_calories = INPUT.split("\n\n").map(|group| group.lines().map(|line| line.parse::<i32>().unwrap()).sum::<i32>()).sorted_by_key(|i| -i).take(3).sum::<i32>();
  println!("Day 1b: {}", top3_calories);
}