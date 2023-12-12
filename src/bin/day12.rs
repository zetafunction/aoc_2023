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
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq)]
struct Key {
    unknowns_left: usize,
    record_left: usize,
    matched_until: usize,
}

#[derive(Debug)]
struct Puzzle {
    records: Vec<Vec<usize>>,
    springs: Vec<String>,
    records5: Vec<Vec<usize>>,
    springs5: Vec<String>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, records) = s
            .lines()
            .map(|line| {
                let (spring, record) = line.split_once(' ').unwrap();
                (
                    spring.to_string(),
                    record
                        .split(",")
                        .map(|val| val.parse().unwrap())
                        .collect::<Vec<usize>>(),
                )
            })
            .unzip::<String, Vec<_>, Vec<_>, Vec<_>>();
        let records5 = records
            .iter()
            .map(|record| {
                record
                    .iter()
                    .cloned()
                    .cycle()
                    .take(record.len() * 5)
                    .collect()
            })
            .collect();
        let springs5 = springs
            .iter()
            .map(|spring: &String| {
                let spring: &str = spring;
                [spring, spring, spring, spring, spring].join("?")
            })
            .collect();

        Ok(Puzzle {
            records,
            springs,
            records5,
            springs5,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn recursive_solve(
    memoizer: &mut HashMap<Key, u64>,
    unknowns: &[usize],
    record: &[usize],
    spring: &str,
    matched_until: usize,
) -> u64 {
    if record.is_empty() {
        // There must be no further #s.
        if spring.as_bytes()[matched_until..]
            .iter()
            .any(|c| *c == b'#')
        {
            return 0;
        } else {
            return 1;
        }
    }

    let next_group_size = record[0];
    let min_remaining_size = record.iter().sum::<usize>() + (record.len() - 1);
    let mut count = 0;
    for i in matched_until..spring.len() - min_remaining_size + 1 {
        if let Some(b'#') = spring.as_bytes()[matched_until..i].iter().next_back() {
            return count;
        }
        // Try to find a position to slot the next group. A group can be slotted iff:
        // - the subsequence for the group contains only #s and ?s
        // - the element after the subsequence for the group is either EOL or '.' or '?'
        if spring.as_bytes()[i..i + next_group_size]
            .iter()
            .any(|c| *c == b'.')
        {
            continue;
        }
        match spring.as_bytes().get(i + next_group_size) {
            Some(b'#') => {
                if spring.as_bytes()[i..]
                    .iter()
                    .take(next_group_size)
                    .all(|c| *c == b'#')
                {
                    return count;
                }
            }
            Some(&bch) if bch == b'?' || bch == b'.' => {
                // First, tentatively consume any unknowns before this position.
                let working = unknowns.iter().take_while(|idx| **idx < i).count();

                let broken = unknowns[working..]
                    .iter()
                    .take_while(|idx| **idx < i + next_group_size)
                    .count();

                // Finally, assign the boundary if needed.
                let boundary = if bch == b'?' { 1 } else { 0 };

                let newly_assigned = working + broken + boundary;
                let remaining_unknowns = &unknowns[newly_assigned..];
                let remaining_record = &record[1..];
                let matched_until = i + next_group_size + 1;

                let key = Key {
                    unknowns_left: remaining_unknowns.len(),
                    record_left: remaining_record.len(),
                    matched_until,
                };

                if let Some(v) = memoizer.get(&key) {
                    count += *v;
                } else {
                    let v = recursive_solve(
                        memoizer,
                        remaining_unknowns,
                        remaining_record,
                        spring,
                        matched_until,
                    );
                    count += v;
                    memoizer.insert(key, v);
                }
            }
            None => {
                if record.len() > 1 {
                    return count;
                }
                return count + 1;
            }
            _ => unreachable!(),
        }
    }
    count
}

fn part1(puzzle: &Puzzle) -> u64 {
    std::iter::zip(puzzle.records.iter(), puzzle.springs.iter())
        .map(|(record, spring)| {
            let unknowns = spring
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
                .collect::<Vec<_>>();
            println!("trying {spring} with {record:?}");
            recursive_solve(&mut HashMap::new(), &unknowns, record, &spring, 0)
        })
        .inspect(|val| println!("{val}"))
        .sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    std::iter::zip(puzzle.records5.iter(), puzzle.springs5.iter())
        .map(|(record, spring)| {
            let unknowns = spring
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
                .collect::<Vec<_>>();
            println!("trying {spring} with {record:?}");
            recursive_solve(&mut HashMap::new(), &unknowns, record, &spring, 0)
        })
        .inspect(|val| println!("{val}"))
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
        "???.### 1,1,3\n",
        ".??..??...?##. 1,1,3\n",
        "?#?#?#?#?#?#?#? 1,3,1,6\n",
        "????.#...#... 4,1,1\n",
        "????.######..#####. 1,6,5\n",
        "?###???????? 3,2,1\n",
    );

    #[test]
    fn example1() {
        assert_eq!(21, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(525152, part2(&parse(SAMPLE).unwrap()));
    }
}
