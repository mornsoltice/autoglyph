use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::HashSet;

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
    let mut rng = thread_rng(); // For randomization

    for (x, y) in positions {
        let mut directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        directions.shuffle(&mut rng); // Shuffle the directions to introduce randomness

        for (dx, dy) in directions {
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

pub fn part1(content: &str, steps: usize) -> usize {
    let (start, grid) = Grid::parse(content);

    let mut positions = HashSet::new();
    positions.insert(start);

    for _i in 0..steps {
        positions = step(&positions, &grid);
    }

    positions.len()
}

/// limited case of lagrange where you need three coordinates
/// of y, and x are 0, 1, 2 respectively;
fn modified_lagrange(px: f64, y1: f64, y2: f64, y3: f64) -> f64 {
    let a = y1 * 0.5 - y2 + y3 * 0.5;
    let b = y2 * 2.0 - (y1 * 3.0) * 0.5 - y3 * 0.5;
    let c = y1;
    a * px * px + b * px + c
}

pub fn part2(content: &str) -> isize {
    let (start, grid) = Grid::parse(content);

    let mut positions = HashSet::new();
    positions.insert(start);

    for _i in 0..65 {
        positions = step(&positions, &grid);
    }

    let y1 = positions.len() as f64;
    println!("y1: {y1}");

    for _i in 0..grid.w {
        positions = step(&positions, &grid);
    }

    let y2 = positions.len() as f64;
    println!("y2: {y2}");

    for _i in 0..grid.w {
        positions = step(&positions, &grid);
    }

    let y3 = positions.len() as f64;
    println!("y3: {y3}");

    modified_lagrange(202300f64, y1, y2, y3) as isize
}
