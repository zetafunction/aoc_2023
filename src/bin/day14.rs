// Copyright 2022 Google LLC
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

use aoc_2023::matrix::Matrix;
use aoc_2023::time;
use aoc_2023::{oops, oops::Oops};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Cell {
    Round = 0,
    Cube = 1,
    Nothing = 2,
}

#[derive(Clone, Debug)]
struct Puzzle {
    platform: Matrix<Cell>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.lines().next().ok_or_else(|| oops!("no lines!"))?.len();

        let mut platform = Matrix::new(cols, rows, Cell::Nothing);

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                platform.set(
                    x,
                    y,
                    match c {
                        'O' => Cell::Round,
                        '#' => Cell::Cube,
                        '.' => continue,
                        _ => unreachable!(),
                    },
                )
            }
        }

        Ok(Puzzle { platform })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

impl Puzzle {
    fn tilt_north(&mut self) {
        for x in 0..self.platform.width() {
            let mut next_write = 0;
            for y in 0..self.platform.height() {
                match self.platform.get(x, y) {
                    Cell::Nothing => {
                        continue;
                    }
                    Cell::Cube => {
                        next_write = y + 1;
                        continue;
                    }
                    Cell::Round => {
                        self.platform.swap(x, y, x, next_write);
                        next_write += 1;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.platform.height() {
            let mut next_write = 0;
            for x in 0..self.platform.width() {
                match self.platform.get(x, y) {
                    Cell::Nothing => {
                        continue;
                    }
                    Cell::Cube => {
                        next_write = x + 1;
                        continue;
                    }
                    Cell::Round => {
                        self.platform.swap(x, y, next_write, y);
                        next_write += 1;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.platform.width() {
            let mut next_write = self.platform.height() - 1;
            for y in (0..self.platform.height()).rev() {
                match self.platform.get(x, y) {
                    Cell::Nothing => {
                        continue;
                    }
                    Cell::Cube => {
                        next_write = y.saturating_sub(1);
                        continue;
                    }
                    Cell::Round => {
                        self.platform.swap(x, y, x, next_write);
                        next_write = next_write.saturating_sub(1);
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.platform.height() {
            let mut next_write = self.platform.width() - 1;
            for x in (0..self.platform.width()).rev() {
                match self.platform.get(x, y) {
                    Cell::Nothing => {
                        continue;
                    }
                    Cell::Cube => {
                        next_write = x.saturating_sub(1);
                        continue;
                    }
                    Cell::Round => {
                        self.platform.swap(x, y, next_write, y);
                        next_write = next_write.saturating_sub(1);
                    }
                }
            }
        }
    }
}

fn calculate(puzzle: &Puzzle) -> usize {
    (0..puzzle.platform.width())
        .map(|x| {
            (0..puzzle.platform.height())
                .filter(|y| puzzle.platform.get(x, *y) == Cell::Round)
                .map(|y| puzzle.platform.height() - y)
                .sum::<usize>()
        })
        .sum()
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut puzzle = puzzle.clone();
    puzzle.tilt_north();
    calculate(&puzzle)
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut puzzle = puzzle.clone();
    let mut iteration = 0;
    let mut states_seen_map = HashMap::<u64, Vec<usize>>::new();
    let mut states_seen = vec![];

    'cycle_finder: while iteration < 1_000_000_000 {
        puzzle.tilt_north();
        puzzle.tilt_west();
        puzzle.tilt_south();
        puzzle.tilt_east();

        let mut hasher = DefaultHasher::new();
        puzzle.platform.hash(&mut hasher);
        let state = hasher.finish();
        states_seen.push(state);
        let previouses = states_seen_map.entry(state).or_insert_with(|| vec![]);

        if let Some(cycle_len) = previouses.iter().find_map(|previous| {
            let cycle_len = iteration - *previous;
            let current_slice = states_seen.get(iteration - cycle_len..iteration);
            let previous_slice = states_seen.get(iteration - cycle_len * 2..iteration - cycle_len);
            if current_slice == previous_slice {
                Some(cycle_len)
            } else {
                None
            }
        }) {
            iteration += 1;
            let remaining = 1_000_000_000 - iteration;
            iteration += (remaining / cycle_len) * cycle_len;
            break 'cycle_finder;
        }

        previouses.push(iteration);
        iteration += 1;
    }

    while iteration < 1_000_000_000 {
        puzzle.tilt_north();
        puzzle.tilt_west();
        puzzle.tilt_south();
        puzzle.tilt_east();
        iteration += 1;
    }

    calculate(&puzzle)
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
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....\n",
    );

    #[test]
    fn example1() {
        assert_eq!(136, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(64, part2(&parse(SAMPLE).unwrap()));
    }
}
