use core::panic;
use std::{collections::HashMap, io::Write};

use itertools::Itertools;
use nalgebra::Vector2;

const INPUT: &str = include_str!("./day14.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Sand,
}

type Pos = Vector2<i32>;

struct SparseGrid {
    data: HashMap<Pos, Cell>,
    moving_sand_pos: Pos,
    has_floor: bool,
    max_y: i32,
}

impl SparseGrid {
    const SPAWN_POS: Pos = Pos::new(500, 0);

    fn new(has_floor: bool) -> SparseGrid {
        SparseGrid {
            data: HashMap::new(),
            moving_sand_pos: Self::SPAWN_POS,
            has_floor,
            max_y: i32::MIN,
        }
    }

    fn get(&self, pos: Pos) -> Option<Cell> {
        if let Some(cell) = self.data.get(&pos) {
            Some(*cell)
        } else if self.has_floor && pos.y >= self.max_y + 2 {
            Some(Cell::Wall)
        } else {
            None
        }
    }

    fn set(&mut self, pos: Pos, cell: Cell) {
        self.data.insert(pos, cell);

        // Only update bounds if we're setting a wall
        // Otherwise floor moves down constantly
        if cell == Cell::Wall {
            self.max_y = self.max_y.max(pos.y);
        }
    }

    fn set_sand_pos(&mut self, pos: Pos) {
        self.moving_sand_pos = pos;
    }

    fn is_sand_falling_infinitely(&self) -> bool {
        self.moving_sand_pos.y > self.max_y
    }

    fn is_spawn_blocked(&self) -> bool {
        self.get(SparseGrid::SPAWN_POS) == Some(Cell::Sand)
    }

    fn create_wall(&mut self, start: Pos, end: Pos) {
        if start.x == end.x {
            // Vertical line
            let (min_y, max_y) = if start.y < end.y {
                (start.y, end.y)
            } else {
                (end.y, start.y)
            };

            for y in min_y..=max_y {
                self.set(Vector2::new(start.x, y), Cell::Wall);
            }
        } else if start.y == end.y {
            // Horizontal line
            let (min_x, max_x) = if start.x < end.x {
                (start.x, end.x)
            } else {
                (end.x, start.x)
            };

            for x in min_x..=max_x {
                self.set(Vector2::new(x, start.y), Cell::Wall);
            }
        } else {
            panic!("Invalid line");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpdateResult {
    Moved,
    Settled,
}

fn update_grid(grid: &mut SparseGrid) -> UpdateResult {
    let pos = grid.moving_sand_pos;

    let down = pos + Vector2::new(0, 1);
    let down_left = pos + Vector2::new(-1, 1);
    let down_right = pos + Vector2::new(1, 1);

    for neighbor in [down, down_left, down_right] {
        match grid.get(neighbor) {
            None => {
                grid.set_sand_pos(neighbor);
                return UpdateResult::Moved;
            }
            Some(_) => {}
        }
    }

    grid.set(pos, Cell::Sand);
    grid.moving_sand_pos = SparseGrid::SPAWN_POS;
    UpdateResult::Settled
}

fn create_grid(has_floor: bool) -> SparseGrid {
    let mut grid = SparseGrid::new(has_floor);

    let lines = INPUT.lines().map(|line| {
        line.split(" -> ")
            .map(|point| {
                let (x, y) = point
                    .split(",")
                    .map(|coord| coord.parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap();

                Vector2::new(x, y)
            })
            .collect_vec()
    });

    for line in lines {
        for (prev, next) in line.iter().tuple_windows() {
            grid.create_wall(*prev, *next);
        }
    }

    grid
}

fn count_settled_sand_until(has_floor: bool, condition: impl Fn(&SparseGrid) -> bool) -> usize {
    let mut grid = create_grid(has_floor);
    let mut sand_settled = 0;

    loop {
        let result = update_grid(&mut grid);

        if result == UpdateResult::Settled {
            sand_settled += 1;
        }

        if condition(&grid) {
            break;
        }
    }

    sand_settled
}

pub fn day14a() {
    let sand_settled = count_settled_sand_until(false, |grid| grid.is_sand_falling_infinitely());
    println!("Day14a: {}", sand_settled);
}

pub fn day14b() {
    let sand_settled = count_settled_sand_until(true, |grid| grid.is_spawn_blocked());
    println!("Day14b: {}", sand_settled);
}

#[allow(dead_code)]
fn render_grid<W: Write>(grid: &SparseGrid) {
    let mut min_y = grid.data.keys().map(|pos| pos.y).min().unwrap();
    let mut max_y = grid.data.keys().map(|pos| pos.y).max().unwrap() + 1;
    let mut max_x = grid.data.keys().map(|pos| pos.x).max().unwrap() + 2;
    let mut min_x = grid.data.keys().map(|pos| pos.x).min().unwrap() - 2;

    let sand_pos = grid.moving_sand_pos;

    min_y = min_y.min(sand_pos.y);
    max_y = max_y.max(sand_pos.y);
    max_x = max_x.max(sand_pos.x);
    min_x = min_x.min(sand_pos.x);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Vector2::new(x, y);

            if sand_pos == pos {
                print!("+");
                continue;
            }

            let cell = grid.get(pos);

            let ch = match cell {
                Some(Cell::Wall) => "#",
                Some(Cell::Sand) => "o",
                None => ".",
            };

            print!("{}", ch);
        }

        println!();
    }
}
