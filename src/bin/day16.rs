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
use aoc_2023::oops::Oops;
use aoc_2023::time;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
enum Space {
    Empty,
    DiagonalMirror,
    AntiDiagonalMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug)]
struct Puzzle {
    // TODO: rework Matrix so get() returns an Option.
    spaces: HashMap<Point2, Space>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spaces = (0i32..)
            .zip(s.lines())
            .flat_map(|(y, line)| {
                (0i32..).zip(line.chars()).map(move |(x, c)| {
                    (
                        Point2::new(x, y),
                        match c {
                            '/' => Space::AntiDiagonalMirror,
                            '\\' => Space::DiagonalMirror,
                            '|' => Space::VerticalSplitter,
                            '-' => Space::HorizontalSplitter,
                            '.' => Space::Empty,
                            _ => unreachable!(),
                        },
                    )
                })
            })
            .collect();
        Ok(Puzzle { spaces })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cursor {
    position: Point2,
    direction: Direction,
}

struct EnergizedState {
    cursors: VecDeque<Cursor>,
    visited: HashSet<Cursor>,
}

impl EnergizedState {
    fn new(initial_cursor: Cursor) -> Self {
        EnergizedState {
            cursors: VecDeque::from([initial_cursor]),
            visited: HashSet::new(),
        }
    }

    fn next_cursor(&mut self) -> Option<Cursor> {
        self.cursors.pop_front()
    }

    fn push_cursor(&mut self, position: Point2, direction: Direction) {
        let cursor = Cursor {
            position,
            direction,
        };
        if self.visited.insert(cursor) {
            self.cursors.push_back(cursor);
        }
    }

    fn energized_count(&self) -> usize {
        self.visited
            .iter()
            .map(|cursor| cursor.position)
            .collect::<HashSet<_>>()
            .len()
    }
}

fn energize(puzzle: &Puzzle, initial_cursor: Cursor) -> usize {
    let mut state = EnergizedState::new(initial_cursor);
    while let Some(Cursor {
        position,
        direction,
    }) = state.next_cursor()
    {
        let next_position = match direction {
            Direction::Up => Point2::new(position.x, position.y - 1),
            Direction::Right => Point2::new(position.x + 1, position.y),
            Direction::Down => Point2::new(position.x, position.y + 1),
            Direction::Left => Point2::new(position.x - 1, position.y),
        };

        let Some(next_space) = puzzle.spaces.get(&next_position) else {
            continue;
        };

        match (next_space, direction) {
            (Space::VerticalSplitter, Direction::Left | Direction::Right) => {
                state.push_cursor(next_position, Direction::Up);
                state.push_cursor(next_position, Direction::Down);
            }
            (Space::HorizontalSplitter, Direction::Up | Direction::Down) => {
                state.push_cursor(next_position, Direction::Left);
                state.push_cursor(next_position, Direction::Right);
            }
            (Space::DiagonalMirror, direction) => {
                state.push_cursor(
                    next_position,
                    match direction {
                        Direction::Up => Direction::Left,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                    },
                );
            }
            (Space::AntiDiagonalMirror, direction) => {
                state.push_cursor(
                    next_position,
                    match direction {
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Right => Direction::Up,
                    },
                );
            }
            _ => {
                state.push_cursor(next_position, direction);
            }
        }
    }

    state.energized_count()
}

fn part1(puzzle: &Puzzle) -> usize {
    let initial_cursor = Cursor {
        position: Point2::new(-1, 0),
        direction: Direction::Right,
    };
    energize(puzzle, initial_cursor)
}

fn part2(puzzle: &Puzzle) -> usize {
    let bounds = Bounds2::from_points(puzzle.spaces.keys());

    (bounds.min.y..bounds.max.y)
        .map(|y| {
            std::cmp::max(
                {
                    let initial_cursor = Cursor {
                        position: Point2::new(bounds.min.x - 1, y),
                        direction: Direction::Right,
                    };
                    energize(puzzle, initial_cursor)
                },
                {
                    let initial_cursor = Cursor {
                        position: Point2::new(bounds.max.x + 1, y),
                        direction: Direction::Right,
                    };
                    energize(puzzle, initial_cursor)
                },
            )
        })
        .chain((bounds.min.x..bounds.max.x).map(|x| {
            std::cmp::max(
                {
                    let initial_cursor = Cursor {
                        position: Point2::new(x, bounds.min.y - 1),
                        direction: Direction::Down,
                    };
                    energize(puzzle, initial_cursor)
                },
                {
                    let initial_cursor = Cursor {
                        position: Point2::new(x, bounds.max.y + 1),
                        direction: Direction::Up,
                    };
                    energize(puzzle, initial_cursor)
                },
            )
        }))
        .max()
        .unwrap()
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

    const SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn example1() {
        assert_eq!(46, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()));
    }
}
