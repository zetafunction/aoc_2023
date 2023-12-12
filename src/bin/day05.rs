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
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Range {
    begin: u64,
    // end is exclusive.
    end: u64,
}

impl Ord for Range {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.end
            .cmp(&rhs.end)
            .then_with(|| self.begin.cmp(&rhs.begin))
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

struct Puzzle {
    seeds: Vec<u64>,
    mappings: Vec<BTreeMap<Range, u64>>,
}

fn parse_mappings(s: &str) -> Result<BTreeMap<Range, u64>, Oops> {
    s.lines()
        .skip(1)
        .map(|line| {
            let mut nums = line.split_whitespace().map(str::parse::<u64>);
            let dst = nums.next().expect("dst missing")?;
            let src = nums.next().expect("src missing")?;
            let len = nums.next().expect("len missing")?;
            Ok((
                Range {
                    begin: src,
                    end: src + len,
                },
                dst,
            ))
        })
        .collect()
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, mappings) = s.split_once("\n\n").ok_or_else(|| oops!("bad input"))?;

        let seeds = seeds
            .strip_prefix("seeds: ")
            .ok_or_else(|| oops!("missing seeds: prefix"))?
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let mappings = mappings
            .split("\n\n")
            .map(parse_mappings)
            .collect::<Result<_, _>>()?;

        Ok(Puzzle { seeds, mappings })
    }
}

fn apply_mapping(src: u64, mapping: &BTreeMap<Range, u64>) -> u64 {
    let src_range = Range {
        begin: src,
        end: src,
    };
    if let Some((key, dst)) = mapping.range(src_range..).next() {
        if src >= key.begin {
            (src - key.begin) + dst
        } else {
            src
        }
    } else {
        src
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    puzzle
        .seeds
        .iter()
        .map(|seed| puzzle.mappings.iter().fold(*seed, apply_mapping))
        .min()
        .expect("no seeds")
}

fn part2(puzzle: &Puzzle) -> u64 {
    std::iter::zip(
        puzzle.seeds.iter().step_by(2),
        puzzle.seeds.iter().skip(1).step_by(2),
    )
    .flat_map(|(seed, range)| {
        (0..*range).map(move |i| puzzle.mappings.iter().fold(seed + i, apply_mapping))
    })
    .min()
    .expect("no seeds")
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
        "seeds: 79 14 55 13\n", //
        "\n",
        "seed-to-soil map:\n",
        "50 98 2\n",
        "52 50 48\n",
        "\n",
        "soil-to-fertilizer map:\n",
        "0 15 37\n",
        "37 52 2\n",
        "39 0 15\n",
        "\n",
        "fertilizer-to-water map:\n",
        "49 53 8\n",
        "0 11 42\n",
        "42 0 7\n",
        "57 7 4\n",
        "\n",
        "water-to-light map:\n",
        "88 18 7\n",
        "18 25 70\n",
        "\n",
        "light-to-temperature map:\n",
        "45 77 23\n",
        "81 45 19\n",
        "68 64 13\n",
        "\n",
        "temperature-to-humidity map:\n",
        "0 69 1\n",
        "1 0 69\n",
        "\n",
        "humidity-to-location map:\n",
        "60 56 37\n",
        "56 93 4\n",
    );

    #[test]
    fn example1() {
        assert_eq!(35, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(46, part2(&parse(SAMPLE).unwrap()));
    }
}
