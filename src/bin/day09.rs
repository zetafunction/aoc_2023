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

use aoc_2023::oops::Oops;
use aoc_2023::time;
use std::io::{self, Read};
use std::str::FromStr;

struct Puzzle {
    values: Vec<Vec<i64>>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            values: s
                .lines()
                .map(|line| {
                    Ok(line
                        .split_whitespace()
                        .map(str::parse)
                        .collect::<Result<Vec<_>, _>>()?)
                })
                .collect::<Result<Vec<_>, Oops>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn solve<'s, Seq>(seq: Seq) -> i64
where
    Seq: std::iter::Iterator<Item = &'s i64>,
{
    let mut accum = vec![];
    accum.push(seq.copied().collect::<Vec<_>>());
    for i in 0..accum[0].len() - 1 {
        let next_line = std::iter::zip(accum[i].iter(), accum[i].iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();
        let done = next_line.iter().all(|x| *x == 0);
        accum.push(next_line);
        if done {
            return accum
                .iter()
                .rev()
                .fold(0, |diff, seq| seq.last().unwrap() + diff);
        }
    }
    unreachable!();
}

fn part1(puzzle: &Puzzle) -> i64 {
    puzzle
        .values
        .iter()
        .map(|history| solve(history.iter()))
        .sum()
}

fn part2(puzzle: &Puzzle) -> i64 {
    puzzle
        .values
        .iter()
        .map(|history| solve(history.iter().rev()))
        .sum()
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
        "0 3 6 9 12 15\n", //
        "1 3 6 10 15 21\n",
        "10 13 16 21 30 45\n",
    );

    #[test]
    fn example1() {
        assert_eq!(114, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2, part2(&parse(SAMPLE).unwrap()));
    }
}
