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

#[derive(Debug)]
struct Game {
    id: u64,
    red: u64,
    green: u64,
    blue: u64,
}

impl FromStr for Game {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, seen_sets) = s.split_once(": ").ok_or_else(|| oops!("malformed line"))?;
        let game = game
            .strip_prefix("Game ")
            .ok_or_else(|| oops!("malformed game ID"))?;
        let id = game.parse::<u64>()?;
        seen_sets.split("; ").try_fold(
            Game {
                id,
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut game, seen_set| {
                for marbles in seen_set.split(", ") {
                    let (count, color) = marbles.split_once(' ').unwrap();
                    let count = count.parse::<u64>()?;
                    match color {
                        "red" => game.red = std::cmp::max(game.red, count),
                        "green" => game.green = std::cmp::max(game.green, count),
                        "blue" => game.blue = std::cmp::max(game.blue, count),
                        _ => return Err(oops!("unknown colour")),
                    }
                }
                Ok(game)
            },
        )
    }
}

#[derive(Debug)]
struct Puzzle {
    games: Vec<Game>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            games: s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    puzzle
        .games
        .iter()
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|game| game.id)
        .sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    puzzle
        .games
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum()
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

    const SAMPLE: &str = concat!(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
    );

    #[test]
    fn example1() {
        assert_eq!(8, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2286, part2(&parse(SAMPLE).unwrap()));
    }
}
