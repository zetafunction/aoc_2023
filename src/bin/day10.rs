// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use aoc_2023::geometry::{Bounds2, Point2};
use aoc_2023::time;
use aoc_2023::{oops, oops::Oops};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

const NORTH: u8 = 1 << 1;
const EAST: u8 = 1 << 2;
const SOUTH: u8 = 1 << 3;
const WEST: u8 = 1 << 4;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    // Bitmask of NORTH/EAST/SOUTH/WEST values.
    connections: u8,
    start: bool,
}

struct Puzzle {
    start: Point2,
    cells: HashMap<Point2, Cell>,
}

// TODO: Find a better solution when needed to use str::parse with a char.
impl Cell {
    fn from_char(c: char) -> Result<Self, Oops> {
        Ok(match c {
            '|' => Cell {
                connections: NORTH | SOUTH,
                ..Default::default()
            },
            '-' => Cell {
                connections: EAST | WEST,
                ..Default::default()
            },
            'L' => Cell {
                connections: NORTH | EAST,
                ..Default::default()
            },
            'J' => Cell {
                connections: NORTH | WEST,
                ..Default::default()
            },
            '7' => Cell {
                connections: SOUTH | WEST,
                ..Default::default()
            },
            'F' => Cell {
                connections: SOUTH | EAST,
                ..Default::default()
            },
            '.' => Cell::default(),
            'S' => Cell {
                start: true,
                ..Default::default()
            },
            _ => return Err(oops!("invalid char")),
        })
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut cells, start) =
            (0i32..)
                .zip(s.lines())
                .fold((HashMap::new(), None), |(cells, start), (y, line)| {
                    (0i32..).zip(line.chars()).fold(
                        (cells, start),
                        |(mut cells, mut start), (x, c)| {
                            let cell = Cell::from_char(c).unwrap();
                            if cell.start {
                                start = Some(Point2::new(x, y));
                            }
                            cells.insert(Point2::new(x, y), cell);
                            (cells, start)
                        },
                    )
                });
        let Some(start) = start else {
            return Err(oops!("no start"));
        };
        let mut start_connections = 0;
        if let Some(cell) = cells.get(&start.north()) {
            if cell.connections & SOUTH != 0 {
                start_connections |= NORTH;
            }
        }
        if let Some(cell) = cells.get(&start.east()) {
            if cell.connections & WEST != 0 {
                start_connections |= EAST;
            }
        }
        if let Some(cell) = cells.get(&start.south()) {
            if cell.connections & NORTH != 0 {
                start_connections |= SOUTH;
            }
        }
        if let Some(cell) = cells.get(&start.west()) {
            if cell.connections & EAST != 0 {
                start_connections |= WEST;
            }
        }
        cells.get_mut(&start).unwrap().connections = start_connections;
        Ok(Puzzle { start, cells })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn solve(puzzle: &Puzzle) -> (u64, HashSet<Point2>) {
    let mut steps = 0;
    let mut visited = HashSet::new();
    let mut currents = vec![puzzle.start, puzzle.start];
    visited.insert(puzzle.start);
    loop {
        let mut nexts = vec![];
        for current in currents.iter() {
            let current_cell = puzzle.cells.get(current).unwrap();
            let candidate = current.north();
            if current_cell.connections & NORTH != 0 && !visited.contains(&candidate) {
                if let Some(cell) = puzzle.cells.get(&candidate) {
                    if cell.connections & SOUTH != 0 {
                        nexts.push(candidate);
                        visited.insert(candidate);
                        continue;
                    }
                }
            }
            let candidate = current.east();
            if current_cell.connections & EAST != 0 && !visited.contains(&candidate) {
                if let Some(cell) = puzzle.cells.get(&candidate) {
                    if cell.connections & WEST != 0 {
                        nexts.push(candidate);
                        visited.insert(candidate);
                        continue;
                    }
                }
            }
            let candidate = current.south();
            if current_cell.connections & SOUTH != 0 && !visited.contains(&candidate) {
                if let Some(cell) = puzzle.cells.get(&candidate) {
                    if cell.connections & NORTH != 0 {
                        nexts.push(candidate);
                        visited.insert(candidate);
                        continue;
                    }
                }
            }
            let candidate = current.west();
            if current_cell.connections & WEST != 0 && !visited.contains(&candidate) {
                if let Some(cell) = puzzle.cells.get(&candidate) {
                    if cell.connections & EAST != 0 {
                        nexts.push(candidate);
                        visited.insert(candidate);
                        continue;
                    }
                }
            }
        }
        if nexts.len() < 2 {
            return (steps + 1, visited);
        }
        steps += 1;
        currents = nexts;
    }
}

fn part1(puzzle: &Puzzle) -> u64 {
    solve(puzzle).0
}

fn part2(puzzle: &Puzzle) -> u64 {
    let (_, visited) = solve(puzzle);
    let bounds = Bounds2::from_points(visited.iter());
    let mut count = 0;
    for y in bounds.min.y..=bounds.max.y {
        let mut in_loop = false;
        let mut last_dir = None;
        for x in bounds.min.x..=bounds.max.x {
            let current = Point2::new(x, y);
            let current_cell = puzzle.cells.get(&current).unwrap();
            if visited.contains(&current) {
                if current_cell.connections & (NORTH | SOUTH) != 0 {
                    if current_cell.connections == NORTH | SOUTH {
                        in_loop = !in_loop;
                        last_dir = None;
                    } else {
                        match last_dir {
                            None => {
                                in_loop = !in_loop;
                                last_dir = Some(current_cell.connections & (NORTH | SOUTH));
                            }
                            Some(dir) => {
                                if dir & current_cell.connections != 0 {
                                    in_loop = !in_loop;
                                }
                                last_dir = None;
                            }
                        }
                    }
                }
                continue;
            }
            if in_loop {
                count += 1;
            }
        }
    }
    count
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = time!(parse(&input)?);

    println!("{}", time!(part1(&puzzle)));
    println!("{}", time!(part2(&puzzle)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "..F7.\n", //
        ".FJ|.\n", //
        "SJ.L7\n", //
        "|F--J\n", //
        "LJ...\n", //
    );

    const SAMPLE2: &str = concat!(
        "...........\n",
        ".S-------7.\n",
        ".|F-----7|.\n",
        ".||.....||.\n",
        ".||.....||.\n",
        ".|L-7.F-J|.\n",
        ".|..|.|..|.\n",
        ".L--J.L--J.\n",
        "...........\n",
    );

    const SAMPLE3: &str = concat!(
        ".F----7F7F7F7F-7....\n",
        ".|F--7||||||||FJ....\n",
        ".||.FJ||||||||L7....\n",
        "FJL7L7LJLJ||LJ.L-7..\n",
        "L--J.L7...LJS7F-7L7.\n",
        "....F-J..F7FJ|L7L7L7\n",
        "....L7.F7||L7|.L7L7|\n",
        ".....|FJLJ|FJ|F7|.LJ\n",
        "....FJL-7.||.||||...\n",
        "....L---J.LJ.LJLJ...\n",
    );

    const SAMPLE4: &str = concat!(
        "FF7FSF7F7F7F7F7F---7\n",
        "L|LJ||||||||||||F--J\n",
        "FL-7LJLJ||||||LJL-77\n",
        "F--JF--7||LJLJ7F7FJ-\n",
        "L---JF-JLJ.||-FJLJJ7\n",
        "|F|F-JF---7F7-L7L|7|\n",
        "|FFJF7L7F-JF7|JL---7\n",
        "7-L-JL7||F7|L7F-7F7|\n",
        "L.L7LFJ|||||FJL7||LJ\n",
        "L7JLJL-JLJLJL--JLJ.L\n",
    );

    #[test]
    fn example1() {
        assert_eq!(8, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(4, part2(&parse(SAMPLE2).unwrap()));
        assert_eq!(8, part2(&parse(SAMPLE3).unwrap()));
        assert_eq!(10, part2(&parse(SAMPLE4).unwrap()));
    }
}
