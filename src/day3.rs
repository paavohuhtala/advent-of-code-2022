use std::collections::HashSet;

const INPUT: &str = include_str!("./day3.input");

fn read_input() -> Vec<Vec<char>> {
    INPUT.lines().map(|line| line.chars().collect()).collect()
}


fn score_char(ch: char) -> i32 {
  if ch.is_uppercase() {
    ch as i32 - 'A' as i32 + 27
  } else {
    ch as i32 - 'a' as i32 + 1
  }
}

pub fn day3a() {
    let input = read_input();

    let mut sum  = 0;

    for line in input {
      let (left, right) = line.split_at(line.len() / 2);
      let left = HashSet::<char>::from_iter(left.iter().cloned());
      let right = HashSet::<char>::from_iter(right.iter().cloned());
      sum += score_char(*left.intersection(&right).next().unwrap());
    }

    println!("Day 3a: {}", sum);
}

pub fn day3b() {
  let input = read_input();

  let mut sum = 0;

  for chunks in input.chunks(3) {
    let [a, b, c]: &[Vec<char>; 3] = chunks.try_into().unwrap();
    let a = HashSet::<char>::from_iter(a.iter().cloned());
    let b = HashSet::<char>::from_iter(b.iter().cloned());
    let c = HashSet::<char>::from_iter(c.iter().cloned());

    let common_char = a.intersection(&b).find(|ch| c.contains(ch)).unwrap();
    sum += score_char(*common_char);
  }

  println!("Day 3b: {}", sum);
}
