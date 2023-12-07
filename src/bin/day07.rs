//  Copyright 2022 Google LLC
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

use aoc_2023::time;
use aoc_2023::{oops, oops::Oops};
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Card {
    A = 15,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Rank {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    Triple = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn with_jokers(cards: &[Card; 5]) -> [Card; 5] {
    let mut cards = *cards;
    for card in &mut cards {
        if *card == Card::J {
            *card = Card::Joker;
        }
    }
    cards
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Hand {
    rank: Rank,
    cards: [Card; 5],
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Line {
    hand: Hand,
    bid: u64,
}

#[derive(Debug)]
struct Puzzle {
    lines: Vec<Line>,
    joker_lines: Vec<Line>,
}

fn classify(cards: &[Card; 5]) -> Rank {
    let unique = cards.iter().fold(HashMap::new(), |mut map, card| {
        map.entry(*card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        map
    });
    match unique.len() {
        5 => Rank::HighCard,
        4 => Rank::OnePair,
        3 => {
            if unique.iter().any(|(_, &count)| count == 3) {
                Rank::Triple
            } else {
                Rank::TwoPair
            }
        }
        2 => {
            if unique.iter().any(|(_, &count)| count == 1) {
                Rank::FourOfAKind
            } else {
                Rank::FullHouse
            }
        }
        1 => Rank::FiveOfAKind,
        _ => panic!("umm"),
    }
}

fn classify_joker(cards: &[Card; 5]) -> Rank {
    let mut cards = *cards;
    cards.sort();
    // Find the most common card
    let (replacement, _, _, _) = cards.iter().fold(
        (Card::Joker, 0, Card::Joker, 0),
        |(max_run_card, max_run_len, cur_run_card, cur_run_len), &card| {
            if card == max_run_card {
                (max_run_card, max_run_len + 1, max_run_card, max_run_len + 1)
            } else if max_run_card == Card::Joker {
                // A run of any non-joker card should supersede a run of any length of joker cards.
                (card, 1, card, 1)
            } else if card == cur_run_card {
                if cur_run_len + 1 >= max_run_len {
                    (cur_run_card, cur_run_len + 1, cur_run_card, cur_run_len + 1)
                } else {
                    (max_run_card, max_run_len, cur_run_card, cur_run_len + 1)
                }
            } else {
                (max_run_card, max_run_len, card, 1)
            }
        },
    );
    // Technically unnecessary for a hand of all jokers, but also harmless.
    for card in &mut cards {
        if *card != Card::Joker {
            break;
        }
        *card = replacement;
    }
    classify(&cards)
}

impl FromStr for Hand {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::Joker; 5];
        for (i, c) in s.chars().enumerate() {
            if i >= cards.len() {
                return Err(oops!("too many cards"));
            }
            cards[i] = match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => return Err(oops!("bad card")),
            };
        }
        let rank = classify(&cards);
        Ok(Hand { cards, rank })
    }
}

impl FromStr for Line {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((hand, bid)) = s.split_once(' ') else {
            return Err(oops!("invalid line"));
        };
        Ok(Line {
            hand: hand.parse()?,
            bid: bid.parse()?,
        })
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?;
        let mut joker_lines = lines
            .iter()
            .map(|line: &Line| {
                let cards = with_jokers(&line.hand.cards);
                let rank = classify_joker(&cards);
                Line {
                    hand: Hand { cards, rank },
                    bid: line.bid,
                }
            })
            .collect::<Vec<_>>();
        lines.sort();
        joker_lines.sort();
        Ok(Puzzle { lines, joker_lines })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> u64 {
    std::iter::zip(1u64.., puzzle.lines.iter())
        .map(|(i, line)| line.bid * i)
        .sum()
}

fn part2(puzzle: &Puzzle) -> u64 {
    std::iter::zip(1u64.., puzzle.joker_lines.iter())
        .map(|(i, line)| line.bid * i)
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
        "32T3K 765\n", //
        "T55J5 684\n",
        "KK677 28\n",
        "KTJJT 220\n",
        "QQQJA 483\n",
    );

    #[test]
    fn example1() {
        assert_eq!(6440, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(5905, part2(&parse(SAMPLE).unwrap()));
    }
}
