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

use aoc_2023::oops::Oops;
use aoc_2023::time;
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq)]
struct Key {
    unknowns_left: usize,
    records_left: usize,
    springs_matched: usize,
}

#[derive(Debug)]
struct Puzzle {
    recordses: Vec<Vec<usize>>,
    springses: Vec<String>,
    recordses5: Vec<Vec<usize>>,
    springses5: Vec<String>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springses, recordses) = s
            .lines()
            .map(|line| {
                let (springs, records) = line.split_once(' ').unwrap();
                (
                    springs.to_string(),
                    records
                        .split(',')
                        .map(|val| val.parse().unwrap())
                        .collect::<Vec<usize>>(),
                )
            })
            .unzip::<String, Vec<_>, Vec<_>, Vec<_>>();
        let recordses5 = recordses
            .iter()
            .map(|records| {
                records
                    .iter()
                    .copied()
                    .cycle()
                    .take(records.len() * 5)
                    .collect()
            })
            .collect();
        let springses5 = springses
            .iter()
            .map(|springs: &String| {
                let springs: &str = springs;
                [springs; 5].join("?")
            })
            .collect();

        Ok(Puzzle {
            recordses,
            springses,
            recordses5,
            springses5,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn recursive_solve(
    memoizer: &mut HashMap<Key, u64>,
    unknowns: &[usize],
    records: &[usize],
    springs: &str,
    springs_matched: usize,
) -> u64 {
    if records.is_empty() {
        // If there are any more broken springs, this subsequence cannot match.
        if springs.as_bytes()[springs_matched..]
            .iter()
            .any(|c| *c == b'#')
        {
            return 0;
        }
        return 1;
    }

    let next_group_size = records[0];
    let min_remaining_size = records.iter().sum::<usize>() + (records.len() - 1);
    let mut count = 0;
    for i in springs_matched..=springs.len() - min_remaining_size {
        if let Some(b'#') = springs.as_bytes()[springs_matched..i].iter().next_back() {
            return count;
        }
        // Try to find a position to slot the next group. A group can be slotted iff:
        // - the subsequence for the group contains only #s and ?s
        // - the element after the subsequence for the group is either EOL or '.' or '?'
        if springs.as_bytes()[i..i + next_group_size]
            .iter()
            .any(|c| *c == b'.')
        {
            continue;
        }
        match springs.as_bytes().get(i + next_group_size) {
            Some(b'#') => {
                // This is a group of next_group_size + 1 broken springs, so it cannot possibly be
                // a group of next_group_size broken springs.
                if springs.as_bytes()[i..]
                    .iter()
                    .take(next_group_size)
                    .all(|c| *c == b'#')
                {
                    return count;
                }
            }
            Some(&bch) if bch == b'?' || bch == b'.' => {
                // First, consume unknowns as working springs before this candidate position..
                let working = unknowns.iter().take_while(|idx| **idx < i).count();

                let broken = unknowns[working..]
                    .iter()
                    .take_while(|idx| **idx < i + next_group_size)
                    .count();

                // Finally, assign the boundary if needed.
                let boundary = usize::from(bch == b'?');

                let newly_assigned = working + broken + boundary;
                let remaining_unknowns = &unknowns[newly_assigned..];
                let remaining_records = &records[1..];
                let springs_matched = i + next_group_size + 1;

                let key = Key {
                    unknowns_left: remaining_unknowns.len(),
                    records_left: remaining_records.len(),
                    springs_matched,
                };

                if let Some(v) = memoizer.get(&key) {
                    count += *v;
                } else {
                    let v = recursive_solve(
                        memoizer,
                        remaining_unknowns,
                        remaining_records,
                        springs,
                        springs_matched,
                    );
                    count += v;
                    memoizer.insert(key, v);
                }
            }
            None => {
                if records.len() > 1 {
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
    std::iter::zip(puzzle.recordses.iter(), puzzle.springses.iter())
        .map(|(records, springs)| {
            let unknowns = springs
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
                .collect::<Vec<_>>();
            println!("trying {springs} with {records:?}");
            recursive_solve(&mut HashMap::new(), &unknowns, records, springs, 0)
        })
        .inspect(|val| println!("{val}"))
        .sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    std::iter::zip(puzzle.recordses5.iter(), puzzle.springses5.iter())
        .map(|(records, springs)| {
            let unknowns = springs
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
                .collect::<Vec<_>>();
            println!("trying {springs} with {records:?}");
            recursive_solve(&mut HashMap::new(), &unknowns, records, springs, 0)
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
