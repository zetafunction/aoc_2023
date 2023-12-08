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

use aoc_2023::{oops, oops::Oops};
use std::io::{self, Read};
use std::str::FromStr;

const DIGITS: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DIGITS_AND_DIGIT_WORDS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

fn find_left(s: &str, digits: &[&str]) -> u64 {
    let mut haystack = s;
    let mut matched_digit = None;
    for (needle, digit) in digits.iter().zip(0u64..) {
        match haystack.find(needle) {
            Some(idx) => {
                matched_digit = Some(digit % 10);
                haystack = &haystack[..idx + needle.len()];
            }
            None => continue,
        }
    }
    matched_digit.unwrap_or(0)
}

fn find_right(s: &str, digits: &[&str]) -> u64 {
    let mut haystack = s;
    let mut matched_digit = None;
    for (needle, digit) in digits.iter().zip(0u64..) {
        match haystack.rfind(needle) {
            Some(idx) => {
                matched_digit = Some(digit % 10);
                haystack = &haystack[idx..];
            }
            None => continue,
        }
    }
    matched_digit.unwrap_or(0)
}

#[derive(Debug)]
struct Value {
    calibration1: u64,
    calibration2: u64,
}

impl FromStr for Value {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value {
            calibration1: find_left(s, DIGITS) * 10 + find_right(s, DIGITS),
            calibration2: find_left(s, DIGITS_AND_DIGIT_WORDS) * 10
                + find_right(s, DIGITS_AND_DIGIT_WORDS),
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    values: Vec<Value>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            values: s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    puzzle.values.iter().map(|v| v.calibration1).sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    puzzle.values.iter().map(|v| v.calibration2).sum()
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

    const SAMPLE: &str = concat!("1abc2\n", "pqr3stu8vwx\n", "a1b2c3d4e5f\n", "treb7uchet\n",);
    const SAMPLE2: &str = concat!(
        "two1nine\n",
        "eightwothree\n",
        "abcone2threexyz\n",
        "xtwone3four\n",
        "4nineeightseven2\n",
        "zoneight234\n",
        "7pqrstsixteen\n",
    );

    #[test]
    fn example1() {
        assert_eq!(142, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(281, part2(&parse(SAMPLE2).unwrap()));
    }
}
