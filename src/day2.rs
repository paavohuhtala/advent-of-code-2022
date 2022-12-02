
const INPUT: &str = include_str!("./day2.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
  Rock,
  Paper,
  Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoundResult {
  Win,
  Draw,
  Loss,
}

impl Shape {
  fn from_str(s: &str) -> Shape {
    match s {
      "A" | "X" => Shape::Rock,
      "B" | "Y" => Shape::Paper,
      "C" | "Z" => Shape::Scissors,
      _ => panic!("Unknown shape: {}", s),
    }
  }
  
  fn play_round(self, other: Shape) -> RoundResult {
    match (self, other) {
      (Shape::Rock, Shape::Paper) => RoundResult::Loss,
      (Shape::Rock, Shape::Scissors) => RoundResult::Win,
      (Shape::Paper, Shape::Rock) => RoundResult::Win,
      (Shape::Paper, Shape::Scissors) => RoundResult::Loss,
      (Shape::Scissors, Shape::Rock) => RoundResult::Loss,
      (Shape::Scissors, Shape::Paper) => RoundResult::Win,
      _ => RoundResult::Draw,
    }
  }
}

impl RoundResult {
  fn from_str(s: &str) -> RoundResult {
    match s {
      "X" => RoundResult::Loss,
      "Y" => RoundResult::Draw,
      "Z" => RoundResult::Win,
      _ => panic!("Unknown round result: {}", s),
    }
  }
}

fn parse_input_1() -> Vec<(Shape, Shape)> {
  INPUT.lines().map(|line| {
    let (opponent, mine) = line.split_once(" ").unwrap();
    (Shape::from_str(opponent), Shape::from_str(mine))
  }).collect()
}

fn parse_input_2() -> Vec<(Shape, RoundResult)> {
  INPUT.lines().map(|line| {
    let (opponent, mine) = line.split_once(" ").unwrap();
    (Shape::from_str(opponent), RoundResult::from_str(mine))
  }).collect()
}

pub fn day2a() {
  let input  = parse_input_1();

  let total_score = input.into_iter().map(|(opponent, mine)| {
    let shape_score = match mine {
      Shape::Rock => 1,
      Shape::Paper => 2,
      Shape::Scissors => 3,
    };

    let round_score = match mine.play_round(opponent) {
      RoundResult::Win => 6,
      RoundResult::Draw => 3,
      RoundResult::Loss => 0,
    };

    shape_score + round_score
  }).sum::<i32>();

  println!("Day 2a: {}", total_score);
}

pub fn day2b() {
  let input  = parse_input_2();

  let total_score = input.into_iter().map(|(opponent, expected_result)| {
    let mine = match (opponent, expected_result) {
      (Shape::Rock, RoundResult::Win) => Shape::Paper,
      (Shape::Rock, RoundResult::Loss) => Shape::Scissors,
      (Shape::Paper, RoundResult::Win) => Shape::Scissors,
      (Shape::Paper, RoundResult::Loss) => Shape::Rock,
      (Shape::Scissors, RoundResult::Win) => Shape::Rock,
      (Shape::Scissors, RoundResult::Loss) => Shape::Paper,
      (a, RoundResult::Draw) => a,
    };

    let shape_score = match mine {
      Shape::Rock => 1,
      Shape::Paper => 2,
      Shape::Scissors => 3,
    };

    let round_score = match expected_result {
      RoundResult::Win => 6,
      RoundResult::Draw => 3,
      RoundResult::Loss => 0,
    };

    shape_score + round_score
  }).sum::<i32>();

  println!("Day 2b: {}", total_score);
}