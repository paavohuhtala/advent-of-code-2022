use std::collections::{HashMap, HashSet};

use nalgebra::Vector2;
use rayon::prelude::*;

const INPUT: &str = include_str!("./day15.input");

type Pos = Vector2<i64>;

#[derive(Debug)]
struct InputLine {
    sensor: Pos,
    beacon: Pos,
}

#[derive(Debug)]
struct SensorWithDistance {
    input: InputLine,
    distance: i64,
}

fn parse_line(line: &str) -> InputLine {
    // Input line looks like this:
    // Sensor at x=833202, y=92320: closest beacon is at x=743030, y=-87472
    let regex = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();
    let captures = regex.captures(line).unwrap();

    let sensor = Pos::new(
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    );

    let beacon = Pos::new(
        captures.get(3).unwrap().as_str().parse().unwrap(),
        captures.get(4).unwrap().as_str().parse().unwrap(),
    );

    InputLine { sensor, beacon }
}

fn manhattan_distance(a: Pos, b: Pos) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn is_covered_by_sensor(pos: Pos, sensors: &[SensorWithDistance]) -> bool {
    for sensor in sensors {
        let distance_to_sensor = manhattan_distance(pos, sensor.input.sensor);

        if distance_to_sensor <= sensor.distance {
            return true;
        }
    }

    false
}

fn count_covered_positions(y: i64, input: &[InputLine]) -> usize {
    let mut covered_positions = HashSet::new();

    for line in input {
        let distance = manhattan_distance(line.sensor, line.beacon);

        for x in -5_000_000..5_000_000 {
            let this_point = Vector2::new(x, y);
            let distance_to_sensor = manhattan_distance(this_point, line.sensor);

            if this_point != line.beacon && distance_to_sensor <= distance {
                covered_positions.insert(this_point);
            }
        }
    }

    covered_positions.len()
}

fn parse_input() -> Vec<InputLine> {
    let input = INPUT
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();
    input
}

fn calculate_distances(input: Vec<InputLine>) -> Vec<SensorWithDistance> {
    let mut distances = Vec::new();

    for line in input {
        let distance = manhattan_distance(line.sensor, line.beacon);
        distances.push(SensorWithDistance {
            input: line,
            distance,
        });
    }

    distances
}

fn edge_points(input: &SensorWithDistance, bound_min: Pos, bound_max: Pos) -> HashSet<Pos> {
    let pos = input.input.sensor;
    let mut points = Vec::new();
    let distance = input.distance + 1;

    points.push(pos + Pos::new(0, distance));
    points.push(pos + Pos::new(distance, 0));
    points.push(pos + Pos::new(0, -distance));
    points.push(pos + Pos::new(-distance, 0));

    for x in 1..distance {
        let y = distance - x;
        points.push(pos + Pos::new(x, y));
        points.push(pos + Pos::new(x, -y));
        points.push(pos + Pos::new(-x, y));
        points.push(pos + Pos::new(-x, -y));
    }

    assert_eq!(
        points.len(),
        (points.iter().copied().collect::<HashSet<_>>()).len()
    );

    points
        .into_iter()
        .filter(|point| {
            point.x >= bound_min.x
                && point.x <= bound_max.x
                && point.y >= bound_min.y
                && point.y <= bound_max.y
        })
        .collect()
}

pub fn day15a() {
    let input = parse_input();
    let result = count_covered_positions(2000000, &input);
    println!("Day 15a: {}", result);
}

pub fn day15b() {
    let start_time = std::time::Instant::now();
    let input = parse_input();
    let input = calculate_distances(input);

    let bound_min = Pos::new(0, 0);
    let bound_max = Pos::new(4_000_000, 4_000_000);
    let all_edge_points = input
        .par_iter()
        .flat_map(|input| edge_points(input, bound_min, bound_max))
        .collect::<HashSet<_>>();

    let uncovered_point = all_edge_points.par_iter().copied().find_any(|point| {
        if is_covered_by_sensor(*point, &input) {
            false
        } else {
            true
        }
    });

    if let Some(uncovered_point) = uncovered_point {
        println!("Found uncovered point: {:?}", uncovered_point);
        println!(
            "Day 15b: {}",
            uncovered_point.x * 4000000 + uncovered_point.y
        );
        println!("Took {:?}", start_time.elapsed());
    }
}
