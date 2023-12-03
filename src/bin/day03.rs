//  Copyright 2023 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use aoc_2023::geometry::Point2;
use aoc_2023::{oops, oops::Oops};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Number(usize),
    Symbol(char),
    Empty,
}

#[derive(Debug)]
struct Puzzle {
    cells: HashMap<Point2, Cell>,
    values: HashMap<usize, usize>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = HashMap::new();
        let mut values = HashMap::new();
        let mut value_id: usize = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x.try_into()?;
                let y = y.try_into()?;
                cells.insert(
                    Point2::new(x, y),
                    if c.is_numeric() {
                        let digit = c as usize - '0' as usize;
                        // Check if the previous point is a number.
                        if let Some(Cell::Number(previous_value_id)) =
                            cells.get(&Point2::new(x - 1, y))
                        {
                            let val = values.get_mut(previous_value_id).unwrap();
                            *val = *val * 10 + digit;
                            Cell::Number(*previous_value_id)
                        } else {
                            value_id += 1;
                            values.insert(value_id, digit);
                            Cell::Number(value_id)
                        }
                    } else if c == '.' {
                        Cell::Empty
                    } else {
                        Cell::Symbol(c)
                    },
                );
            }
        }
        Ok(Puzzle { cells, values })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> usize {
    let value_ids = puzzle
        .cells
        .iter()
        .filter_map(|(p, &c)| {
            let Cell::Number(value_id) = c else {
                return None;
            };
            if p.all_neighbors()
                .any(|neighbor| matches!(puzzle.cells.get(&neighbor), Some(Cell::Symbol(_))))
            {
                Some(value_id)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    value_ids
        .iter()
        .map(|value_id| puzzle.values.get(value_id).unwrap())
        .sum()
}

fn part2(puzzle: &Puzzle) -> usize {
    puzzle
        .cells
        .iter()
        .filter_map(|(p, &cell)| match cell {
            Cell::Symbol('*') => {
                let ids = p
                    .all_neighbors()
                    .filter_map(|neighbor| match puzzle.cells.get(&neighbor) {
                        Some(Cell::Number(value_id)) => Some(value_id),
                        _ => None,
                    })
                    .collect::<HashSet<_>>();
                match ids.len() {
                    2 => Some(
                        ids.into_iter()
                            .map(|id| puzzle.values.get(id).unwrap())
                            .product::<usize>(),
                    ),
                    _ => None,
                }
            }
            _ => None,
        })
        .sum()
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = parse(&input)?;

    println!("{}", part1(&puzzle));
    println!("{}", part2(&puzzle));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "467..114..\n",
        "...*......\n",
        "..35..633.\n",
        "......#...\n",
        "617*......\n",
        ".....+.58.\n",
        "..592.....\n",
        "......755.\n",
        "...$.*....\n",
        ".664.598..\n",
    );

    #[test]
    fn example1() {
        assert_eq!(4361, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(467835, part2(&parse(SAMPLE).unwrap()));
    }
}
