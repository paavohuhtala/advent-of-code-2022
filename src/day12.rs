use itertools::Itertools;
use nalgebra::Vector2;
use pathfinding::directed::bfs;

const INPUT: &str = include_str!("./day12.input");

type Pos = Vector2<isize>;

struct Heightmap {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

struct PointsOfInterest {
    start: Pos,
    destination: Pos,
}

impl Heightmap {
    fn get(&self, pos: Pos) -> u8 {
        self.data[(pos.y as usize * self.width + pos.x as usize) as usize]
    }

    fn is_out_of_bounds(&self, pos: Pos) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.width as isize || pos.y >= self.height as isize
    }

    fn get_legal_neighbors(&self, pos: Pos, out: &mut Vec<Pos>) {
        out.clear();

        let current_height = self.get(pos);

        let left = pos + Vector2::new(-1, 0);
        let right = pos + Vector2::new(1, 0);
        let up = pos + Vector2::new(0, -1);
        let down = pos + Vector2::new(0, 1);

        let neighbors = [left, right, up, down];

        for neighbor in neighbors.iter() {
            if self.is_out_of_bounds(*neighbor) {
                continue;
            }

            let neighbor_height = self.get(*neighbor);

            if neighbor_height > current_height + 1 {
                continue;
            }

            out.push(*neighbor);
        }
    }

    fn index_to_pos(&self, index: usize) -> Pos {
        let x = index % self.width;
        let y = index / self.width;
        Vector2::new(x as isize, y as isize)
    }

    fn find_path(&self, start: Pos, end: Pos) -> Option<Vec<Pos>> {
        bfs::bfs(
            &start,
            |pos| {
                let mut neighbors = Vec::new();
                self.get_legal_neighbors(*pos, &mut neighbors);
                neighbors
            },
            |pos| *pos == end,
        )
    }
}

fn read_input() -> (Heightmap, PointsOfInterest) {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();
    let mut data = vec![0; width * height];
    let mut start_pos = None;
    let mut destination_pos = None;

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = y * width + x;
            match c {
                'S' => {
                    data[pos] = 0;
                    start_pos = Some(Vector2::new(x as isize, y as isize));
                }
                'E' => {
                    data[pos] = 'z' as u8 - 'a' as u8;
                    destination_pos = Some(Vector2::new(x as isize, y as isize));
                }
                'a'..='z' => data[pos] = c as u8 - 'a' as u8,
                _ => panic!("Invalid input"),
            }
        }
    }

    let start_pos = start_pos.unwrap();
    let destination_pos = destination_pos.unwrap();

    let heightmap = Heightmap {
        data,
        width,
        height,
    };

    let points_of_interest = PointsOfInterest {
        start: start_pos,
        destination: destination_pos,
    };

    (heightmap, points_of_interest)
}

pub fn day12a() {
    let start_time = std::time::Instant::now();

    let (heightmap, points_of_interest) = read_input();
    let path = heightmap
        .find_path(points_of_interest.start, points_of_interest.destination)
        .unwrap();

    println!("Day 12a: {}", path.len() - 1);
    println!("Time: {}ms", start_time.elapsed().as_millis());
}

pub fn day12b() {
    let start_time = std::time::Instant::now();

    let (heightmap, points_of_interest) = read_input();

    let starting_points = heightmap
        .data
        .iter()
        .enumerate()
        .filter(|(_, height)| **height == 0)
        .map(|(i, _)| heightmap.index_to_pos(i))
        .collect_vec();

    let mut min_path_length = std::usize::MAX;

    for start in starting_points {
        let path = heightmap.find_path(start, points_of_interest.destination);
        if let Some(path) = path {
            min_path_length = min_path_length.min(path.len() - 1);
        }
    }

    println!("Day 12b: {}", min_path_length);
    println!("Time: {}ms", start_time.elapsed().as_millis());
}

#[allow(dead_code)]
fn print_path(heightmap: &Heightmap, path: &[Pos], points_of_interest: &PointsOfInterest) {
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            let pos = Vector2::new(x as isize, y as isize);
            if pos == points_of_interest.start {
                print!("S");
            } else if pos == points_of_interest.destination {
                print!("E");
            } else if path.contains(&pos) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
