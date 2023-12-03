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
use aoc_2023::oops::Oops;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Id(u32);

#[derive(Clone, Copy, Debug)]
enum Cell {
    Number(Id),
    Symbol(char),
}

#[derive(Debug)]
struct Puzzle {
    cells: HashMap<Point2, Cell>,
    values: HashMap<Id, u64>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = HashMap::new();
        let mut values = HashMap::<Id, u64>::new();
        let mut next_id: Id = Id(0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x.try_into()?, y.try_into()?);
                if let Some(digit) = c.to_digit(10).map(u64::from) {
                    let id = match cells.get(&Point2::new(x - 1, y)) {
                        Some(Cell::Number(previous_id)) => *previous_id,
                        _ => {
                            next_id.0 += 1;
                            next_id
                        }
                    };
                    cells.insert(Point2::new(x, y), Cell::Number(id));
                    values
                        .entry(id)
                        .and_modify(|val| *val = *val * 10 + digit)
                        .or_insert(digit);
                } else if c != '.' {
                    cells.insert(Point2::new(x, y), Cell::Symbol(c));
                };
            }
        }
        Ok(Puzzle { cells, values })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    puzzle
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
        .collect::<HashSet<_>>()
        .iter()
        .map(|value_id| puzzle.values.get(value_id).unwrap())
        .sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    puzzle
        .cells
        .iter()
        .map(|(p, &cell)| match cell {
            Cell::Symbol('*') => {
                let ids = p
                    .all_neighbors()
                    .filter_map(|neighbor| match puzzle.cells.get(&neighbor) {
                        Some(Cell::Number(value_id)) => Some(value_id),
                        _ => None,
                    })
                    .collect::<HashSet<_>>();
                match ids.len() {
                    2 => ids
                        .into_iter()
                        .map(|id| puzzle.values.get(id).unwrap())
                        .product(),
                    _ => 0,
                }
            }
            _ => 0,
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
