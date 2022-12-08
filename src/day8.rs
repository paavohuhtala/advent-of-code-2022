const INPUT: &str = include_str!("./day8.input");

struct Map {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.width + x]
    }

    fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if self.is_edge(x, y) {
            return true;
        }

        let value = self.get(x, y);

        let visible_from_left = (0..x).all(|i| self.get(i, y) < value);
        let visible_from_right = (x + 1..self.width).all(|i| self.get(i, y) < value);
        let visible_from_top = (0..y).all(|i| self.get(x, i) < value);
        let visible_from_bottom = (y + 1..self.height).all(|i| self.get(x, i) < value);

        visible_from_left || visible_from_right || visible_from_top || visible_from_bottom
    }

    fn measure_viewing_distance(&self, x: usize, y: usize) -> usize {
        if self.is_edge(x, y) {
            return 0;
        }

        let value = self.get(x, y);

        let mut visible_to_left = 0;

        for i in (0..x).rev() {
            let cell_value = self.get(i, y);

            visible_to_left += 1;

            if cell_value >= value {
                break;
            }
        }

        let mut visible_to_right = 0;

        for i in x + 1..self.width {
            let cell_value = self.get(i, y);

            visible_to_right += 1;

            if cell_value >= value {
                break;
            }
        }

        let mut visible_to_top = 0;

        for i in (0..y).rev() {
            let cell_value = self.get(x, i);

            visible_to_top += 1;

            if cell_value >= value {
                break;
            }
        }

        let mut visible_to_bottom = 0;

        for i in y + 1..self.height {
            let cell_value = self.get(x, i);

            visible_to_bottom += 1;

            if cell_value >= value {
                break;
            }
        }

        visible_to_left * visible_to_right * visible_to_top * visible_to_bottom
    }
}

fn parse_input() -> Map {
    let height = INPUT.lines().count();
    let width = INPUT.lines().next().unwrap().len();

    let data = INPUT
        .lines()
        .flat_map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8))
        .collect();

    Map {
        width,
        height,
        data,
    }
}

pub fn day8a() {
    let map = parse_input();
    let mut visible = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if map.is_visible(x, y) {
                visible += 1;
            }
        }
    }

    println!("Day 8a: {}", visible);
}

pub fn day8b() {
    let map = parse_input();
    let mut highest_viewing_distance = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            let viewing_distance = map.measure_viewing_distance(x, y);
            if viewing_distance > highest_viewing_distance {
                highest_viewing_distance = viewing_distance;
            }
        }
    }

    println!("Day 8b: {}", highest_viewing_distance);
}
