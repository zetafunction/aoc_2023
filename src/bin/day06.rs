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
use std::io::{self, Read};
use std::str::FromStr;

struct Race {
    time: u64,
    distance: u64,
}

struct Puzzle {
    records1: Vec<Race>,
    record2: Race,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let Some(time_line) = lines.next() else {
            return Err(oops!("missing times"));
        };
        let Some(time_line) = time_line.strip_prefix("Time:") else {
            return Err(oops!("bad time format"));
        };
        let Some(distance_line) = lines.next() else {
            return Err(oops!("missing distances"));
        };
        let Some(distance_line) = distance_line.strip_prefix("Distance:") else {
            return Err(oops!("bad distance format"));
        };
        Ok(Puzzle {
            records1: std::iter::zip(
                time_line.split_whitespace(),
                distance_line.split_whitespace(),
            )
            .map(|(time, distance)| {
                Ok(Race {
                    time: time.parse()?,
                    distance: distance.parse()?,
                })
            })
            .collect::<Result<_, Self::Err>>()?,
            record2: Race {
                time: time_line.split_whitespace().collect::<String>().parse()?,
                distance: distance_line
                    .split_whitespace()
                    .collect::<String>()
                    .parse()?,
            },
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    puzzle
        .records1
        .iter()
        .map(|r| {
            u64::try_from(
                (0..r.time)
                    .filter(|pressed_time| (r.time - pressed_time) * (pressed_time) > r.distance)
                    .count(),
            )
            .unwrap()
        })
        .product()
}

fn part2(puzzle: &Puzzle) -> u64 {
    (0..puzzle.record2.time)
        .filter(|pressed_time| {
            (puzzle.record2.time - pressed_time) * (pressed_time) > puzzle.record2.distance
        })
        .count()
        .try_into()
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

    const SAMPLE: &str = concat!(
        "Time:      7  15   30\n", //
        "Distance:  9  40  200\n",
    );

    #[test]
    fn example1() {
        assert_eq!(288, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(71503, part2(&parse(SAMPLE).unwrap()));
    }
}
