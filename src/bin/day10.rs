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

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

trait PointHelpers {
    #[must_use]
    fn north(&self) -> Self;
    #[must_use]
    fn east(&self) -> Self;
    #[must_use]
    fn south(&self) -> Self;
    #[must_use]
    fn west(&self) -> Self;
    #[must_use]
    fn in_direction(&self, direction: Direction) -> Self;
}

impl PointHelpers for Point2 {
    fn north(&self) -> Self {
        Point2::new(self.x, self.y - 1)
    }

    fn east(&self) -> Self {
        Point2::new(self.x + 1, self.y)
    }

    fn south(&self) -> Self {
        Point2::new(self.x, self.y + 1)
    }

    fn west(&self) -> Self {
        Point2::new(self.x - 1, self.y)
    }

    fn in_direction(&self, direction: Direction) -> Point2 {
        match direction {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    CornerL,
    CornerJ,
    Corner7,
    CornerF,
}

impl Pipe {
    fn has_exit(self, direction: Direction) -> bool {
        matches!(
            (direction, self),
            (
                Direction::North,
                Self::Vertical | Self::CornerL | Self::CornerJ
            ) | (
                Direction::East,
                Self::Horizontal | Self::CornerL | Self::CornerF
            ) | (
                Direction::South,
                Self::Vertical | Self::Corner7 | Self::CornerF
            ) | (
                Direction::West,
                Self::Horizontal | Self::CornerJ | Self::Corner7
            )
        )
    }
}

struct Puzzle {
    start: Point2,
    cells: HashMap<Point2, Pipe>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut cells, start) = (0i32..).zip(s.lines()).try_fold(
            (HashMap::new(), None),
            |(cells, start), (y, line)| {
                (0i32..)
                    .zip(line.chars())
                    .try_fold((cells, start), |(mut cells, start), (x, c)| {
                        let pipe = match c {
                            '|' => Pipe::Vertical,
                            '-' => Pipe::Horizontal,
                            'L' => Pipe::CornerL,
                            'J' => Pipe::CornerJ,
                            '7' => Pipe::Corner7,
                            'F' => Pipe::CornerF,
                            'S' => {
                                return if start.is_none() {
                                    Ok((cells, Some(Point2::new(x, y))))
                                } else {
                                    return Err(oops!("multiple starts"));
                                }
                            }
                            '.' => return Ok((cells, start)),
                            _ => return Err(oops!("invalid character")),
                        };
                        cells.insert(Point2::new(x, y), pipe);
                        Ok((cells, start))
                    })
            },
        )?;
        let start = start.ok_or_else(|| oops!("no start"))?;

        let start_directions = ALL_DIRECTIONS
            .into_iter()
            .filter(|direction| {
                if let Some(neighbor) = cells.get(&start.in_direction(*direction)) {
                    neighbor.has_exit(direction.opposite())
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        if start_directions.len() != 2 {
            return Err(oops!(
                "expected 2 connections to start, got {}",
                start_directions.len()
            ));
        }

        // Note that the directions in the match will be in the same order as ALL_DIRECTIONS.
        cells.insert(
            start,
            match start_directions[0..2] {
                [Direction::North, Direction::South] => Pipe::Vertical,
                [Direction::East, Direction::West] => Pipe::Horizontal,
                [Direction::North, Direction::East] => Pipe::CornerL,
                [Direction::North, Direction::West] => Pipe::CornerJ,
                [Direction::South, Direction::West] => Pipe::Corner7,
                [Direction::East, Direction::South] => Pipe::CornerF,
                _ => unreachable!(),
            },
        );

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
        let nexts = currents
            .iter()
            .filter_map(|current| {
                let pipe = puzzle.cells.get(current).expect("traversed to empty cell");
                ALL_DIRECTIONS.into_iter().find_map(|direction| {
                    let candidate = current.in_direction(direction);
                    let Some(candidate_pipe) = puzzle.cells.get(&candidate) else {
                        return None;
                    };
                    if pipe.has_exit(direction)
                        && candidate_pipe.has_exit(direction.opposite())
                        && !visited.contains(&candidate)
                    {
                        visited.insert(candidate);
                        Some(candidate)
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();
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
        let mut last_direction = None;
        for x in bounds.min.x..=bounds.max.x {
            let current = Point2::new(x, y);
            if visited.contains(&current) {
                let pipe = puzzle.cells.get(&current).unwrap();
                if pipe.has_exit(Direction::North) || pipe.has_exit(Direction::South) {
                    if *pipe == Pipe::Vertical {
                        in_loop = !in_loop;
                        last_direction = None;
                        continue;
                    }
                    match last_direction {
                        None => {
                            in_loop = !in_loop;
                            last_direction = Some(if pipe.has_exit(Direction::North) {
                                Direction::North
                            } else {
                                Direction::South
                            });
                        }
                        Some(direction) => {
                            if pipe.has_exit(direction) {
                                in_loop = !in_loop;
                            }
                            last_direction = None;
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
