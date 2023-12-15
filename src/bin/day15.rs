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

fn hash(input: &str) -> u8 {
    let mut value: u32 = 0;
    for c in input.chars() {
        value += u32::from(c);
        value *= 17;
        value %= 256;
    }
    value as u8
}

#[derive(Debug)]
struct Puzzle {
    steps: Vec<String>,
    parsed_steps: Vec<ParsedStep>,
}

#[derive(Debug)]
enum Op {
    Remove,
    Insert(u8),
}

#[derive(Debug)]
struct ParsedStep {
    label: String,
    op: Op,
}

impl FromStr for ParsedStep {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((label, _)) = s.split_once('-') {
            Ok(ParsedStep {
                label: label.to_string(),
                op: Op::Remove,
            })
        } else if let Some((label, focal_len)) = s.split_once('=') {
            Ok(ParsedStep {
                label: label.to_string(),
                op: Op::Insert(focal_len.parse()?),
            })
        } else {
            Err(oops!("unparseable step {s}"))
        }
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .split(',')
            .map(str::trim)
            .map(str::to_string)
            .collect::<Vec<_>>();
        let parsed_steps = steps
            .iter()
            .map(|step| step.parse())
            .collect::<Result<_, _>>()?;

        Ok(Puzzle {
            steps,
            parsed_steps,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    puzzle.steps.iter().map(|s| u64::from(hash(s))).sum()
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u8,
}

fn part2(puzzle: &Puzzle) -> u64 {
    let mut lenses_boxes = vec![Vec::<Lens>::default(); 256];

    for parsed_step in &puzzle.parsed_steps {
        let box_idx = usize::from(hash(&parsed_step.label));
        let lenses_box = &mut lenses_boxes[box_idx];
        let lens_idx = lenses_box
            .iter()
            .position(|lens| lens.label == parsed_step.label);
        match parsed_step.op {
            Op::Remove => {
                if let Some(lens_idx) = lens_idx {
                    lenses_box.remove(lens_idx);
                }
            }
            Op::Insert(focal_len) => {
                let lens = Lens {
                    label: &parsed_step.label,
                    focal_len,
                };
                match lens_idx {
                    Some(lens_idx) => lenses_box[lens_idx] = lens,
                    None => lenses_box.push(lens),
                }
            }
        }
    }

    (0u64..)
        .zip(lenses_boxes.iter())
        .map(|(box_idx, lenses_box)| {
            (0u64..)
                .zip(lenses_box.iter())
                .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * u64::from(lens.focal_len))
                .sum::<u64>()
        })
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

    const SAMPLE: &str = concat!("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",);

    #[test]
    fn example1() {
        assert_eq!(1320, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(145, part2(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn hash() {
        assert_eq!(52, super::hash("HASH"));
    }
}
