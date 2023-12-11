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
use aoc_2023::oops::Oops;
use aoc_2023::time;
use std::collections::BTreeSet;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    galaxies: Vec<Point2>,
    empty_cols: BTreeSet<i32>,
    empty_rows: BTreeSet<i32>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.lines().flat_map(|line| line.chars()).collect::<Vec<_>>();
        let height = i32::try_from(s.lines().count())?;
        let width = i32::try_from(s.lines().count())?;

        // Find empty rows and columns first.
        let mut empty_cols = (0i32..width).collect::<BTreeSet<_>>();
        let mut empty_rows = (0i32..height).collect::<BTreeSet<_>>();

        let mut galaxies = vec![];

        for y in 0i32..height {
            for x in 0i32..width {
                if chars[usize::try_from(y * width + x)?] == '#' {
                    empty_cols.remove(&x);
                    empty_rows.remove(&y);
                    galaxies.push(Point2::new(x, y));
                }
            }
        }

        Ok(Puzzle {
            galaxies,
            empty_cols,
            empty_rows,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn adjust_point_for_expansion_factor(puzzle: &Puzzle, point: Point2, factor: usize) -> Point2 {
    Point2::new(
        point.x
            + i32::try_from(puzzle.empty_cols.range(0..point.x).count() * (factor - 1)).unwrap(),
        point.y
            + i32::try_from(puzzle.empty_rows.range(0..point.y).count() * (factor - 1)).unwrap(),
    )
}

fn solve_with_expansion_factor(puzzle: &Puzzle, factor: usize) -> u64 {
    (0..puzzle.galaxies.len())
        .flat_map(|i| {
            (i + 1..puzzle.galaxies.len())
                .map(move |j| {
                    let src = adjust_point_for_expansion_factor(puzzle, puzzle.galaxies[i], factor);
                    let dst = adjust_point_for_expansion_factor(puzzle, puzzle.galaxies[j], factor);
                    Point2::manhattan_distance(&src, &dst)
                })
                .map(u64::from)
        })
        .sum()
}
fn part1(puzzle: &Puzzle) -> u64 {
    solve_with_expansion_factor(puzzle, 2)
}

fn part2(puzzle: &Puzzle) -> u64 {
    solve_with_expansion_factor(puzzle, 1_000_000)
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
        assert_eq!(
            1030,
            solve_with_expansion_factor(&parse(SAMPLE).unwrap(), 10)
        );
        assert_eq!(
            8410,
            solve_with_expansion_factor(&parse(SAMPLE).unwrap(), 100)
        );
    }
}
