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

    fn get(&self, (x, y): (isize, isize)) -> char {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            return '.';
        }
        self.chars[y as usize][x as usize]
    }
}

fn connected_pipes(grid: &Grid, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
    let mut connected = Vec::new();
    let current = grid.get((x, y));

    match current {
        '|' => {
            if "F7|S".contains(grid.get((x, y - 1))) {
                connected.push((x, y - 1));
            }
            if "JL|S".contains(grid.get((x, y + 1))) {
                connected.push((x, y + 1));
            }
        }
        '-' => {
            if "FL-S".contains(grid.get((x - 1, y))) {
                connected.push((x - 1, y));
            }
            if "J7-S".contains(grid.get((x + 1, y))) {
                connected.push((x + 1, y));
            }
        }
        'L' => {
            if "F7|S".contains(grid.get((x, y - 1))) {
                connected.push((x, y - 1));
            }
            if "J7-S".contains(grid.get((x + 1, y))) {
                connected.push((x + 1, y));
            }
        }
        'J' => {
            if "F7|S".contains(grid.get((x, y - 1))) {
                connected.push((x, y - 1));
            }
            if "FL-S".contains(grid.get((x - 1, y))) {
                connected.push((x - 1, y));
            }
        }
        '7' => {
            if "JL|S".contains(grid.get((x, y + 1))) {
                connected.push((x, y + 1));
            }
            if "FL-S".contains(grid.get((x - 1, y))) {
                connected.push((x - 1, y));
            }
        }
        'F' => {
            if "JL|S".contains(grid.get((x, y + 1))) {
                connected.push((x, y + 1));
            }
            if "J7-S".contains(grid.get((x + 1, y))) {
                connected.push((x + 1, y));
            }
        }
        'S' => {
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if connected_pipes(grid, (nx, ny)).contains(&(x, y)) {
                    connected.push((nx, ny));
                }
            }
        }
        _ => {}
    }

    connected
}

fn find_loop(grid: &Grid, start: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut loop_tiles = HashSet::new();
    let mut current = start;
    loop_tiles.insert(current);

    loop {
        let next_positions = connected_pipes(grid, current);
        let next = next_positions.iter().find(|&pos| !loop_tiles.contains(pos));

        match next {
            Some(&pos) => {
                current = pos;
                loop_tiles.insert(current);
            }
            None => break,
        }

        if current == start {
            break;
        }
    }

    loop_tiles
}

pub fn part1(content: &str) -> usize {
    let (start, grid) = Grid::parse(content);
    let loop_tiles = find_loop(&grid, start);
    loop_tiles.len() / 2
}

pub fn part2(content: &str, _use_all_steps: bool) -> usize {
    let (start, grid) = Grid::parse(content);
    let loop_tiles = find_loop(&grid, start);

    let mut inside_count = 0;
    for y in 0..grid.h {
        let mut inside = false;
        let mut last_corner = ' ';
        for x in 0..grid.w {
            if loop_tiles.contains(&(x, y)) {
                let mut tile = grid.get((x, y));
                if tile == 'S' {
                    // Determine the correct pipe type for 'S'
                    let connected = connected_pipes(&grid, (x, y));
                    tile = match (
                        connected[0].0 - x,
                        connected[0].1 - y,
                        connected[1].0 - x,
                        connected[1].1 - y,
                    ) {
                        (0, -1, 0, 1) | (0, 1, 0, -1) => '|',
                        (-1, 0, 1, 0) | (1, 0, -1, 0) => '-',
                        (0, -1, 1, 0) | (1, 0, 0, -1) => 'L',
                        (0, -1, -1, 0) | (-1, 0, 0, -1) => 'J',
                        (0, 1, 1, 0) | (1, 0, 0, 1) => 'F',
                        (0, 1, -1, 0) | (-1, 0, 0, 1) => '7',
                        _ => panic!("Invalid 'S' connection"),
                    };
                }
                match tile {
                    '|' => inside = !inside,
                    'F' | 'L' => last_corner = tile,
                    '7' => {
                        if last_corner == 'L' {
                            inside = !inside
                        }
                    }
                    'J' => {
                        if last_corner == 'F' {
                            inside = !inside
                        }
                    }
                    _ => {}
                }
            } else if inside {
                inside_count += 1;
            }
        }
    }

    inside_count
}
