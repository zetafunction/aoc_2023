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
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    galaxies: Vec<Point2>,
    // Map of empty cols/rows to the number of adjustments needed.
    empty_cols: BTreeMap<i32, i32>,
    empty_rows: BTreeMap<i32, i32>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = 0;
        let galaxies = std::iter::zip(0i32.., s.lines())
            .inspect(|(y, _)| height = std::cmp::max(height, *y))
            .flat_map(|(y, line)| {
                std::iter::zip(0i32.., line.chars()).filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Point2::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        let height = height + 1;
        let width = height;

        // (-1, -1) should never be a valid coordinate, but removes an edge case when looking up
        // how many adjustments are needed later.
        let mut empty_cols = (-1i32..width).collect::<BTreeSet<_>>();
        let mut empty_rows = (-1i32..height).collect::<BTreeSet<_>>();

        for galaxy in &galaxies {
            empty_cols.remove(&galaxy.x);
            empty_rows.remove(&galaxy.y);
        }

        let empty_cols = std::iter::zip(empty_cols, 0i32..).collect();
        let empty_rows = std::iter::zip(empty_rows, 0i32..).collect();

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

fn adjust_point_for_expansion_factor(puzzle: &Puzzle, point: Point2, factor: i32) -> Point2 {
    let x_adj = puzzle.empty_cols.range(-1..=point.x).next_back().unwrap().1;
    let y_adj = puzzle.empty_rows.range(-1..=point.y).next_back().unwrap().1;
    Point2::new(
        point.x + x_adj * (factor - 1),
        point.y + y_adj * (factor - 1),
    )
}

fn solve_with_expansion_factor(puzzle: &Puzzle, factor: i32) -> u64 {
    let memoized = puzzle
        .galaxies
        .iter()
        .map(|galaxy| adjust_point_for_expansion_factor(puzzle, *galaxy, factor))
        .collect::<Vec<_>>();
    (0..memoized.len())
        .flat_map(|i| {
            let memoized = &memoized;
            (i + 1..memoized.len())
                .map(move |j| {
                    let src = &memoized[i];
                    let dst = &memoized[j];
                    Point2::manhattan_distance(src, dst)
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
