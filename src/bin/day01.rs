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

use aoc_2023::{oops, oops::Oops};
use std::io::{self, Read};
use std::str::FromStr;

const DIGITS: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DIGITS_AND_DIGIT_WORDS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

fn find_digit(s: &str, digits: &[&str], dir: Dir) -> Result<usize, Oops> {
    let mut haystack = s;
    let mut matched_digit = None;
    for (digit, needle) in digits.iter().enumerate() {
        match match dir {
            Dir::Left => haystack.find(needle),
            Dir::Right => haystack.rfind(needle),
        } {
            Some(idx) => {
                matched_digit = Some(digit % 10);
                haystack = match dir {
                    Dir::Left => &haystack[..idx + needle.len()],
                    Dir::Right => &haystack[idx..],
                }
            }
            None => continue,
        }
    }
    matched_digit.ok_or_else(|| oops!("not found!"))
}

#[derive(Debug)]
struct Value {
    calibration1: usize,
    calibration2: usize,
}

impl FromStr for Value {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value {
            calibration1: find_digit(s, DIGITS, Dir::Left)? * 10
                + find_digit(s, DIGITS, Dir::Right)?,
            calibration2: find_digit(s, DIGITS_AND_DIGIT_WORDS, Dir::Left)? * 10
                + find_digit(s, DIGITS_AND_DIGIT_WORDS, Dir::Right)?,
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

fn part1(puzzle: &Puzzle) -> usize {
    puzzle.values.iter().map(|v| v.calibration1).sum()
}

fn part2(puzzle: &Puzzle) -> usize {
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
