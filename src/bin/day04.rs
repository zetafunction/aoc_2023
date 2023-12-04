//  Copyright 2022 Google LLC
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

use aoc_2023::{oops, oops::Oops};
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

struct Card {
    winning: HashSet<u32>,
    have: Vec<u32>,
}

impl FromStr for Card {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(": ").ok_or_else(|| oops!("invalid format"))?;
        let (winning_str, have_str) = numbers
            .split_once(" | ")
            .ok_or_else(|| oops!("missing delim"))?;
        Ok(Card {
            winning: winning_str
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .collect::<Result<_, _>>()?,
            have: have_str
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

struct Puzzle {
    cards: Vec<Card>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            cards: s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    puzzle
        .cards
        .iter()
        .map(|c| {
            let count = c.have.iter().filter(|n| c.winning.contains(n)).count();
            if count > 0 {
                2u64.pow((count - 1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    let winning_counts = puzzle
        .cards
        .iter()
        .map(|c| c.have.iter().filter(|n| c.winning.contains(n)).count())
        .collect::<Vec<_>>();
    let mut copies = vec![1; winning_counts.len()];
    for (idx, count) in winning_counts.iter().enumerate() {
        let current = copies[idx];
        for copy_idx in idx + 1..idx + 1 + count {
            if let Some(count) = copies.get_mut(copy_idx) {
                *count += current;
            }
        }
    }
    copies.iter().sum()
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
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
    );

    #[test]
    fn example1() {
        assert_eq!(13, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(30, part2(&parse(SAMPLE).unwrap()));
    }
}
