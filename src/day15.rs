use fnv::FnvHashSet;
use nalgebra::Vector2;
use once_cell::sync::OnceCell;
use rayon::prelude::*;
use regex::Regex;

const INPUT: &str = include_str!("./day15.input");

type Pos = Vector2<i64>;

#[derive(Debug)]
struct SensorAndBacon {
    sensor: Pos,
    beacon: Pos,

    distance: i64,
}

fn parse_line(line: &str) -> SensorAndBacon {
    static REGEX: OnceCell<Regex> = OnceCell::new();

    let regex = REGEX.get_or_init(|| {
        regex::Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap()
    });
    let captures = regex.captures(line).unwrap();

    let sensor = Pos::new(
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    );

    let beacon = Pos::new(
        captures.get(3).unwrap().as_str().parse().unwrap(),
        captures.get(4).unwrap().as_str().parse().unwrap(),
    );

    let distance = manhattan_distance(sensor, beacon);

    SensorAndBacon {
        sensor,
        beacon,
        distance,
    }
}

fn manhattan_distance(a: Pos, b: Pos) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn is_covered_by_sensor(pos: Pos, sensors: &[SensorAndBacon]) -> bool {
    for sensor in sensors {
        let distance_to_sensor = manhattan_distance(pos, sensor.sensor);

        if distance_to_sensor <= sensor.distance {
            return true;
        }
    }

    false
}

fn parse_input() -> Vec<SensorAndBacon> {
    INPUT.lines().map(|line| parse_line(line)).collect()
}

fn edge_points(sensor: &SensorAndBacon) -> Vec<Pos> {
    let pos = sensor.sensor;
    let mut points = Vec::new();
    let distance = sensor.distance + 1;

    for x in 0..distance {
        let y = distance - x;
        points.push(pos + Pos::new(x, y));
        points.push(pos + Pos::new(x, -y));
        points.push(pos + Pos::new(-x, y));
        points.push(pos + Pos::new(-x, -y));
    }

    points
}

fn points_on_line(y: i64, sensor: &SensorAndBacon) -> Vec<Pos> {
    let mut points = Vec::new();

    for x in -sensor.distance..=sensor.distance {
        let this_point = Vector2::new(sensor.sensor.x + x, y);

        if this_point == sensor.beacon {
            continue;
        }

        let distance_to_sensor = manhattan_distance(this_point, sensor.sensor);

        if distance_to_sensor <= sensor.distance {
            points.push(this_point);
        }
    }

    points
}

pub fn day15a() {
    let start_time = std::time::Instant::now();

    let input = parse_input();

    let result = {
        let input: &[SensorAndBacon] = &input;

        let covered_positions = input
            .par_iter()
            .flat_map(|sensor| points_on_line(2_000_000, sensor))
            .collect::<FnvHashSet<_>>();

        covered_positions.len()
    };

    println!("Day 15a: {}", result);
    println!("Took {:?}", start_time.elapsed());
}

pub fn day15b() {
    let start_time = std::time::Instant::now();

    let input = parse_input();

    let uncovered_point = input.par_iter().flat_map(edge_points).find_any(|point| {
        if is_covered_by_sensor(*point, &input) {
            false
        } else {
            true
        }
    });

    if let Some(uncovered_point) = uncovered_point {
        println!(
            "Day 15b: {}",
            uncovered_point.x * 4000000 + uncovered_point.y
        );
        println!("Took {:?}", start_time.elapsed());
    }
}
