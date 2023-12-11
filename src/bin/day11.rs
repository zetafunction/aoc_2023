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

use aoc_2023::geometry::Point2;
use aoc_2023::time;
use aoc_2023::{oops, oops::Oops};
use std::collections::BTreeSet;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    galaxies: Vec<Point2>,
    mega_galaxies: Vec<Point2>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.lines().flat_map(|line| line.chars()).collect::<Vec<_>>();
        let height = i32::try_from(s.lines().count())?;
        let width = i32::try_from(s.lines().count())?;
        // Find empty rows and columns first.
        let mut empty_columns = BTreeSet::new();
        empty_columns.extend(0i32..width);
        let mut empty_rows = BTreeSet::new();
        empty_rows.extend(0i32..height);

        let mut raw_galaxies = vec![];

        for y in 0i32..height {
            for x in 0i32..width {
                if chars[usize::try_from(y * width + x)?] == '#' {
                    empty_columns.remove(&x);
                    empty_rows.remove(&y);
                    raw_galaxies.push(Point2::new(x, y));
                }
            }
        }

        let galaxies = raw_galaxies
            .iter()
            .map(|galaxy| {
                let x_adjustment = i32::try_from(empty_columns.range(0..galaxy.x).count())?;
                let y_adjustment = i32::try_from(empty_rows.range(0..galaxy.y).count())?;
                Ok(Point2::new(
                    galaxy.x + x_adjustment,
                    galaxy.y + y_adjustment,
                ))
            })
            .collect::<Result<Vec<_>, Oops>>()?;

        let mega_galaxies = raw_galaxies
            .iter()
            .map(|galaxy| {
                let x_adjustment = i32::try_from(empty_columns.range(0..galaxy.x).count())?;
                let y_adjustment = i32::try_from(empty_rows.range(0..galaxy.y).count())?;
                Ok(Point2::new(
                    galaxy.x + x_adjustment * 999999,
                    galaxy.y + y_adjustment * 999999,
                ))
            })
            .collect::<Result<Vec<_>, Oops>>()?;

        Ok(Puzzle {
            galaxies,
            mega_galaxies,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u32 {
    (0..puzzle.galaxies.len())
        .flat_map(|i| {
            (i + 1..puzzle.galaxies.len()).map(move |j| {
                i32::abs_diff(puzzle.galaxies[i].x, puzzle.galaxies[j].x)
                    + i32::abs_diff(puzzle.galaxies[i].y, puzzle.galaxies[j].y)
            })
        })
        .sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    (0..puzzle.mega_galaxies.len())
        .flat_map(|i| {
            (i + 1..puzzle.galaxies.len())
                .map(move |j| {
                    i32::abs_diff(puzzle.mega_galaxies[i].x, puzzle.mega_galaxies[j].x)
                        + i32::abs_diff(puzzle.mega_galaxies[i].y, puzzle.mega_galaxies[j].y)
                })
                .map(u64::from)
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

    const SAMPLE: &str = concat!(
        "...#......\n",
        ".......#..\n",
        "#.........\n",
        "..........\n",
        "......#...\n",
        ".#........\n",
        ".........#\n",
        "..........\n",
        ".......#..\n",
        "#...#.....\n",
    );

    #[test]
    fn example1() {
        assert_eq!(374, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()));
    }
}
