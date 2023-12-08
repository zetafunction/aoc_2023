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
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::str::FromStr;

struct Range {
    dest: u64,
    len: u64,
}

struct Puzzle {
    initial_seeds: Vec<u64>,
    seeds_to_soils: BTreeMap<u64, Range>,
    soils_to_fertilizers: BTreeMap<u64, Range>,
    fertilizers_to_waters: BTreeMap<u64, Range>,
    waters_to_lights: BTreeMap<u64, Range>,
    lights_to_temperatures: BTreeMap<u64, Range>,
    temperatures_to_humidities: BTreeMap<u64, Range>,
    humidities_to_locations: BTreeMap<u64, Range>,
}

fn parse_mapping(s: &str) -> Result<BTreeMap<u64, Range>, Oops> {
    let mut mapping = BTreeMap::new();
    for line in s.lines() {
        let mut nums = line.split_whitespace().map(str::parse::<u64>);
        let dest = nums.next().expect("dest missing")?;
        let src = nums.next().expect("src missing")?;
        let len = nums.next().expect("len missing")?;
        mapping.insert(src, Range { dest, len });
    }
    Ok(mapping)
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n\n");

        let initial_seeds = sections
            .next()
            .expect("seeds")
            .strip_prefix("seeds: ")
            .expect("missing prefix")
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let seeds_to_soils = parse_mapping(
            sections
                .next()
                .expect("missing section")
                .strip_prefix("seed-to-soil map:\n")
                .expect("missing prefix"),
        )?;

        let soils_to_fertilizers = parse_mapping(
            sections
                .next()
                .expect("missing section")
                .strip_prefix("soil-to-fertilizer map:\n")
                .expect("missing prefix"),
        )?;

        let fertilizers_to_waters = parse_mapping(
            sections
                .next()
                .expect("missing section")
                .strip_prefix("fertilizer-to-water map:\n")
                .expect("missing prefix"),
        )?;

        let waters_to_lights = parse_mapping(
            sections
                .next()
                .expect("missing section")
                .strip_prefix("water-to-light map:\n")
                .expect("missing prefix"),
        )?;

        let lights_to_temperatures = parse_mapping(
            sections
                .next()
                .expect("missing section")
                .strip_prefix("light-to-temperature map:\n")
                .expect("missing prefix"),
        )?;

        let temperatures_to_humidities = parse_mapping(
            sections
                .next()
                .expect("missing section")
                .strip_prefix("temperature-to-humidity map:\n")
                .expect("missing prefix"),
        )?;

        let humidities_to_locations = parse_mapping(
            sections
                .next()
                .expect("missing section")
                .strip_prefix("humidity-to-location map:\n")
                .expect("missing prefix"),
        )?;

        Ok(Puzzle {
            initial_seeds,
            seeds_to_soils,
            soils_to_fertilizers,
            fertilizers_to_waters,
            waters_to_lights,
            lights_to_temperatures,
            temperatures_to_humidities,
            humidities_to_locations,
        })
    }
}

fn do_mapping(src: u64, m: &BTreeMap<u64, Range>) -> u64 {
    let mut elements = m.range(..=src).rev();
    if let Some((key, info)) = elements.next() {
        if src >= *key && src < *key + info.len {
            (src - *key) + info.dest
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
        .initial_seeds
        .iter()
        .map(|seed| {
            do_mapping(
                do_mapping(
                    do_mapping(
                        do_mapping(
                            do_mapping(
                                do_mapping(
                                    do_mapping(*seed, &puzzle.seeds_to_soils),
                                    &puzzle.soils_to_fertilizers,
                                ),
                                &puzzle.fertilizers_to_waters,
                            ),
                            &puzzle.waters_to_lights,
                        ),
                        &puzzle.lights_to_temperatures,
                    ),
                    &puzzle.temperatures_to_humidities,
                ),
                &puzzle.humidities_to_locations,
            )
        })
        .min()
        .expect("no seeds")
}

fn part2(puzzle: &Puzzle) -> u64 {
    puzzle
        .initial_seeds
        .iter()
        .step_by(2)
        .zip(puzzle.initial_seeds.iter().skip(1).step_by(2))
        .flat_map(|(seed, range)| {
            (0..*range).map(|i| {
                do_mapping(
                    do_mapping(
                        do_mapping(
                            do_mapping(
                                do_mapping(
                                    do_mapping(
                                        do_mapping(*seed + i, &puzzle.seeds_to_soils),
                                        &puzzle.soils_to_fertilizers,
                                    ),
                                    &puzzle.fertilizers_to_waters,
                                ),
                                &puzzle.waters_to_lights,
                            ),
                            &puzzle.lights_to_temperatures,
                        ),
                        &puzzle.temperatures_to_humidities,
                    ),
                    &puzzle.humidities_to_locations,
                )
            })
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
