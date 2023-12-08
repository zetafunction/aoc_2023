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

use aoc_2023::time;
use aoc_2023::{oops, oops::Oops};
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

enum Dir {
    Left,
    Right,
}

struct Node {
    left: String,
    right: String,
}

struct Puzzle {
    directions: Vec<Dir>,
    nodes: HashMap<String, Node>,
}

impl FromStr for Node {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(", ").ok_or_else(|| oops!("bad node"))?;
        let left = left
            .strip_prefix('(')
            .ok_or_else(|| oops!("bad left"))?
            .to_string();
        let right = right
            .strip_suffix(')')
            .ok_or_else(|| oops!("bad right"))?
            .to_string();
        Ok(Node { left, right })
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (directions, nodes) = s.split_once("\n\n").ok_or_else(|| oops!("bad input"))?;
        Ok(Puzzle {
            directions: directions
                .chars()
                .map(|c| {
                    Ok(match c {
                        'L' => Dir::Left,
                        'R' => Dir::Right,
                        _ => return Err(oops!("bad direction")),
                    })
                })
                .collect::<Result<_, _>>()?,
            nodes: nodes
                .lines()
                .map(|line| -> Result<_, Oops> {
                    let (src, dst) = line.split_once(" = ").ok_or_else(|| oops!("bad node"))?;
                    Ok((src.to_string(), dst.parse()?))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    let mut current = "AAA";
    for (step, dir) in std::iter::zip(1u64.., puzzle.directions.iter().cycle()) {
        let node = puzzle.nodes.get(current).unwrap();
        current = match dir {
            Dir::Left => &node.left,
            Dir::Right => &node.right,
        };
        if current == "ZZZ" {
            return step;
        }
    }
    0
}

fn part2(puzzle: &Puzzle) -> u64 {
    puzzle
        .nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        // Determine the number of steps for each cycle, assuming that the initial
        // journey provides the cycle length.
        .map(|mut current| {
            for (step, dir) in std::iter::zip(1u64.., puzzle.directions.iter().cycle()) {
                let node = puzzle.nodes.get(current).unwrap();
                current = match dir {
                    Dir::Left => &node.left,
                    Dir::Right => &node.right,
                };
                if current.ends_with('Z') {
                    return step;
                }
            }
            0
        })
        .fold(1, aoc_2023::math::lcm)
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
        "LLR\n", //
        "\n",
        "AAA = (BBB, BBB)\n",
        "BBB = (AAA, ZZZ)\n",
        "ZZZ = (ZZZ, ZZZ)\n",
    );

    const SAMPLE2: &str = concat!(
        "LR\n",
        "\n",
        "11A = (11B, XXX)\n",
        "11B = (XXX, 11Z)\n",
        "11Z = (11B, XXX)\n",
        "22A = (22B, XXX)\n",
        "22B = (22C, 22C)\n",
        "22C = (22Z, 22Z)\n",
        "22Z = (22B, 22B)\n",
        "XXX = (XXX, XXX)\n",
    );

    #[test]
    fn example1() {
        assert_eq!(6, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(6, part2(&parse(SAMPLE2).unwrap()));
    }
}
