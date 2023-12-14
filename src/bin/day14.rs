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

use aoc_2023::time;
use aoc_2023::{oops, oops::Oops};
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Round = 0,
    Cube = 1,
    Nothing = 2,
}

#[derive(Debug)]
struct Puzzle {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'O' => Cell::Round,
                        '#' => Cell::Cube,
                        '.' => Cell::Nothing,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = cells.len();
        let cols = cells[0].len();

        Ok(Puzzle { cells, rows, cols })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn tilt_north(puzzle: &Puzzle, cells: &mut [Vec<Cell>]) {
    for x in 0..puzzle.cols {
        let mut next_write = 0;
        for y in 0..puzzle.rows {
            match cells[y][x] {
                Cell::Nothing => {
                    continue;
                }
                Cell::Cube => {
                    next_write = y + 1;
                    continue;
                }
                Cell::Round => {
                    let temp = cells[next_write][x];
                    cells[next_write][x] = cells[y][x];
                    cells[y][x] = temp;
                    next_write += 1;
                }
            }
        }
    }
}

fn tilt_west(puzzle: &Puzzle, cells: &mut [Vec<Cell>]) {
    for y in 0..puzzle.rows {
        let mut next_write = 0;
        for x in 0..puzzle.cols {
            match cells[y][x] {
                Cell::Nothing => {
                    continue;
                }
                Cell::Cube => {
                    next_write = x + 1;
                    continue;
                }
                Cell::Round => {
                    let temp = cells[y][next_write];
                    cells[y][next_write] = cells[y][x];
                    cells[y][x] = temp;
                    next_write += 1;
                }
            }
        }
    }
}

fn tilt_south(puzzle: &Puzzle, cells: &mut [Vec<Cell>]) {
    for x in 0..puzzle.cols {
        let mut next_write = puzzle.rows - 1;
        for y in (0..puzzle.rows).rev() {
            match cells[y][x] {
                Cell::Nothing => {
                    continue;
                }
                Cell::Cube => {
                    next_write = y.saturating_sub(1);
                    continue;
                }
                Cell::Round => {
                    let temp = cells[next_write][x];
                    cells[next_write][x] = cells[y][x];
                    cells[y][x] = temp;
                    next_write = next_write.saturating_sub(1);
                }
            }
        }
    }
}

fn tilt_east(puzzle: &Puzzle, cells: &mut [Vec<Cell>]) {
    for y in 0..puzzle.rows {
        let mut next_write = puzzle.cols - 1;
        for x in (0..puzzle.cols).rev() {
            match cells[y][x] {
                Cell::Nothing => {
                    continue;
                }
                Cell::Cube => {
                    next_write = x.saturating_sub(1);
                    continue;
                }
                Cell::Round => {
                    let temp = cells[y][next_write];
                    cells[y][next_write] = cells[y][x];
                    cells[y][x] = temp;
                    next_write = next_write.saturating_sub(1);
                }
            }
        }
    }
}

fn calculate(puzzle: &Puzzle, cells: &[Vec<Cell>]) -> usize {
    (0..puzzle.cols)
        .map(|x| {
            (0..puzzle.rows)
                .filter(|y| cells[*y][x] == Cell::Round)
                .map(|y| puzzle.cols - y)
                .sum::<usize>()
        })
        .sum()
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut cells = puzzle.cells.clone();

    tilt_north(puzzle, &mut cells);

    calculate(puzzle, &cells)
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut cells = puzzle.cells.clone();
    let mut iteration = 0;
    let mut scores_seen_map = HashMap::<usize, Vec<usize>>::new();
    let mut scores_seen = vec![];

    while iteration < 1_000 {
        tilt_north(puzzle, &mut cells);
        tilt_west(puzzle, &mut cells);
        tilt_south(puzzle, &mut cells);
        tilt_east(puzzle, &mut cells);
        let score = calculate(puzzle, &cells);
        scores_seen.push(score);
        scores_seen_map
            .entry(score)
            .and_modify(|seen| seen.push(iteration))
            .or_insert_with(|| vec![iteration]);
        iteration += 1;
    }

    'cycle_finder: while iteration < 1_000_000_000 {
        tilt_north(puzzle, &mut cells);
        tilt_west(puzzle, &mut cells);
        tilt_south(puzzle, &mut cells);
        tilt_east(puzzle, &mut cells);
        let score = calculate(puzzle, &cells);
        scores_seen.push(score);
        if let Some(previous_its) = scores_seen_map.get_mut(&score) {
            // If not a cycle, just append and continue.
            for previous_it in &*previous_its {
                let cycle_len = iteration - *previous_it;
                if (1..cycle_len)
                    .all(|i| scores_seen[iteration - i] == scores_seen[iteration - cycle_len - i])
                {
                    let cycle_scores: Vec<_> = (0..cycle_len)
                        .rev()
                        .map(|i| scores_seen[iteration - i])
                        .collect();
                    println!("{cycle_scores:?}");
                    // Fast forward.
                    let remaining = 1_000_000_000 - iteration;
                    iteration += (remaining / cycle_len) * cycle_len;
                    break 'cycle_finder;
                }
            }
            // Not a cycle, just continue.
            previous_its.push(iteration);
            iteration += 1;
        } else {
            scores_seen_map.insert(score, vec![iteration]);
            iteration += 1;
        }
    }

    while iteration < 1_000_000_000 {
        tilt_north(puzzle, &mut cells);
        tilt_west(puzzle, &mut cells);
        tilt_south(puzzle, &mut cells);
        tilt_east(puzzle, &mut cells);
        iteration += 1;
        println!("{}", calculate(puzzle, &cells));
    }

    calculate(puzzle, &cells)
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
