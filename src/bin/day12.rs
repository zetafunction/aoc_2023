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
struct Memoize {
    unknowns_assigned: usize,
    matched_until: usize,
    records_matched: usize,
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
    memoizer: &mut HashMap<Memoize, u64>,
    unknowns: &[usize],
    record: &[usize],
    spring: &mut [char],
    matched_until: usize,
    unknowns_assigned: usize,
    records_matched: usize,
) -> u64 {
    let key = Memoize {
        unknowns_assigned,
        matched_until,
        records_matched,
    };
    if let Some(count) = memoizer.get(&key) {
        return *count;
    }

    if records_matched == record.len() {
        // There must be no further #s.
        if spring[matched_until..].iter().any(|c| *c == '#') {
            memoizer.insert(key, 0);
            return 0;
        } else {
            memoizer.insert(key, 1);
            return 1;
        }
    }

    let next_group_size = record[records_matched];
    let remaining_records = record.len() - records_matched;
    let min_remaining_size =
        record.iter().rev().take(remaining_records).sum::<usize>() + (remaining_records - 1);
    let mut count = 0;
    for i in matched_until..spring.len() - min_remaining_size + 1 {
        if let Some('#') = spring[matched_until..i].iter().next_back() {
            memoizer.insert(key, count);
            return count;
        }
        // Try to find a position to slot the next group. A group can be slotted iff:
        // - the subsequence for the group contains only #s and ?s
        // - the element after the subsequence for the group is either EOL or '.' or '?'
        let candidate = &spring[i..];
        if candidate.iter().take(next_group_size).any(|c| *c == '.') {
            continue;
        }
        match candidate.get(next_group_size) {
            Some('#') => {
                if spring[i..].iter().take(next_group_size).all(|c| *c == '#') {
                    memoizer.insert(key, count);
                    return count;
                }
            }
            Some(&bch) if bch == '?' || bch == '.' => {
                // First, tentatively consume any unknowns before this position.
                let mut working = 0;
                while unknowns_assigned + working < unknowns.len()
                    && unknowns[unknowns_assigned + working] < i
                {
                    spring[unknowns[unknowns_assigned + working]] = '.';
                    working += 1;
                }

                // Now assign broken ones.
                let mut broken = 0;
                while unknowns_assigned + working + broken < unknowns.len()
                    && unknowns[unknowns_assigned + working + broken] < i + next_group_size
                {
                    spring[unknowns[unknowns_assigned + working + broken]] = '#';
                    broken += 1;
                }

                // Finally, assign the boundary if needed.
                let mut boundary = 0;
                if bch == '?' {
                    spring[i + next_group_size] = '.';
                    boundary = 1;
                }

                let newly_assigned = working + broken + boundary;
                // Now recurse!
                count += recursive_solve(
                    memoizer,
                    unknowns,
                    record,
                    spring,
                    i + next_group_size + 1,
                    unknowns_assigned + newly_assigned,
                    records_matched + 1,
                );

                // And then undo the changes to the spring by resetting them to '?'.
                for unk_idx in 0..newly_assigned {
                    spring[unknowns[unknowns_assigned + unk_idx]] = '?';
                }
            }
            None => {
                if remaining_records > 1 {
                    memoizer.insert(key, count);
                    return count;
                }
                memoizer.insert(key, count + 1);
                return count + 1;
            }
            _ => unreachable!(),
        }
    }
    memoizer.insert(key, count);
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
            let mut spring = Vec::from_iter(spring.chars());
            recursive_solve(&mut HashMap::new(), &unknowns, record, &mut spring, 0, 0, 0)
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
            let mut spring = Vec::from_iter(spring.chars());
            recursive_solve(&mut HashMap::new(), &unknowns, record, &mut spring, 0, 0, 0)
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
