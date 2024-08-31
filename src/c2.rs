use std::{collections::HashSet, isize};

pub struct Grid {
    chars: Vec<Vec<char>>,
    w: isize,
    h: isize,
}

impl Grid {
    fn parse(content: &str) -> ((isize, isize), Self) {
        let mut chars = Vec::new();
        let mut start = (0, 0);
        for (y, line) in content.lines().enumerate() {
            let chs: Vec<_> = line.chars().collect();
            for (x, c) in chs.iter().enumerate() {
                if c == &'S' {
                    start = (x as isize, y as isize);
                }
            }
            chars.push(chs);
        }

        let h = chars.len() as isize;
        let w = chars.first().unwrap().len() as isize;

        let g = Self { chars, w, h };

        (start, g)
    }

    fn recursive_coordinates(&self, (x, y): (isize, isize)) -> (isize, isize) {
        let mut nx = x % self.w;
        let mut ny = y % self.h;

        if nx < 0 {
            nx += self.w;
        }

        if ny < 0 {
            ny += self.h;
        }

        (nx, ny)
    }

    fn get(&self, (x, y): (isize, isize)) -> char {
        let (nx, ny) = self.recursive_coordinates((x, y));
        self.chars[ny as usize][nx as usize]
    }
}

fn step(positions: &HashSet<(isize, isize)>, grid: &Grid) -> HashSet<(isize, isize)> {
    let mut new_positions = HashSet::new();
    for (x, y) in positions {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            let c = grid.get((nx, ny));

            if c == '#' {
                continue;
            }

            new_positions.insert((nx, ny));
        }
    }

    new_positions
}

pub fn part1(content: &str) -> usize {
    let (start, grid) = Grid::parse(content);

    let mut positions = HashSet::new();
    positions.insert(start);

    for _i in 0..4 {
        positions = step(&positions, &grid);
    }

    positions.len()
}

pub fn part2(content: &str, use_all_steps: bool) -> usize {
    let (start, grid) = Grid::parse(content);

    let mut positions = HashSet::new();
    positions.insert(start);

    let steps = if use_all_steps { 10 } else { 4 };
    for _i in 0..steps {
        positions = step(&positions, &grid);
    }

    positions.len()
}
