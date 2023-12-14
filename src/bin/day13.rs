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

#[derive(Debug)]
struct Puzzle {
    horizontal_valleys: Vec<Vec<String>>,
    vertical_valleys: Vec<Vec<String>>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let horizontal_valleys: Vec<Vec<_>> = s
            .split("\n\n")
            .map(|block| block.lines().map(str::to_string).collect())
            .collect();
        let vertical_valleys = horizontal_valleys
            .iter()
            .map(|valley| {
                let rows = valley.len();
                let cols = valley[0].len();
                (0..cols)
                    .map(|col| {
                        (0..rows)
                            .map(|row| char::from(valley[row].as_bytes()[col]))
                            .collect()
                    })
                    .collect()
            })
            .collect();

        Ok(Puzzle {
            horizontal_valleys,
            vertical_valleys,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn reflects(valley: &[String]) -> Option<usize> {
    (1..valley.len()).find(|i| {
        (0..*i).all(|j| match (valley.get(i - j - 1), valley.get(i + j)) {
            (Some(x), Some(y)) if x == y => true,
            (Some(_), None) | (None, Some(_)) | (None, None) => true,
            (Some(_), Some(_)) => false,
        })
    })
}

fn almost_reflects(valley: &[String]) -> Option<usize> {
    (1..valley.len()).find(|i| {
        (0..*i)
            .try_fold(false, |found_almost_pair, j| {
                match (valley.get(i - j - 1), valley.get(i + j)) {
                    (Some(x), Some(y)) if x == y => Ok(found_almost_pair),
                    (Some(_), None) | (None, Some(_)) | (None, None) => Ok(found_almost_pair),
                    (Some(x), Some(y)) => {
                        if !found_almost_pair
                            && std::iter::zip(x.chars(), y.chars())
                                .filter(|(x, y)| x != y)
                                .count()
                                == 1
                        {
                            Ok(true)
                        } else {
                            Err(oops!("not this one"))
                        }
                    }
                }
            })
            .unwrap_or(false)
    })
}

fn part1(puzzle: &Puzzle) -> usize {
    std::iter::zip(
        puzzle.horizontal_valleys.iter(),
        puzzle.vertical_valleys.iter(),
    )
    .map(
        |(horizontal, vertical)| match (reflects(horizontal), reflects(vertical)) {
            (Some(rows), None) => rows * 100,
            (None, Some(cols)) => cols,
            _ => unreachable!(),
        },
    )
    .sum()
}

fn part2(puzzle: &Puzzle) -> usize {
    std::iter::zip(
        puzzle.horizontal_valleys.iter(),
        puzzle.vertical_valleys.iter(),
    )
    .map(
        |(horizontal, vertical)| match (almost_reflects(horizontal), almost_reflects(vertical)) {
            (Some(rows), None) => rows * 100,
            (None, Some(cols)) => cols,
            _ => unreachable!(),
        },
    )
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
        "#.##..##.\n",
        "..#.##.#.\n",
        "##......#\n",
        "##......#\n",
        "..#.##.#.\n",
        "..##..##.\n",
        "#.#.##.#.\n",
        "\n",
        "#...##..#\n",
        "#....#..#\n",
        "..##..###\n",
        "#####.##.\n",
        "#####.##.\n",
        "..##..###\n",
        "#....#..#\n",
    );

    #[test]
    fn example1() {
        assert_eq!(405, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(400, part2(&parse(SAMPLE).unwrap()));
    }
}
