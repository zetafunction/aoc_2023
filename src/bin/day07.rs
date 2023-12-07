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
use std::cmp::Ordering;
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

const NON_JOKERS: [Card; 13] = [
    Card::A,
    Card::K,
    Card::Q,
    Card::J,
    Card::T,
    Card::Nine,
    Card::Eight,
    Card::Seven,
    Card::Six,
    Card::Five,
    Card::Four,
    Card::Three,
    Card::Two,
];

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

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    rank: Rank,
}

#[derive(Debug, Eq)]
struct JokerHand {
    cards: [Card; 5],
    rank: Rank,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        std::iter::zip(self.cards.iter(), other.cards.iter()).all(|(x, y)| x == y)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (x, y) in std::iter::zip(self.cards.iter(), other.cards.iter()) {
                    match x.cmp(y) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => (),
                    }
                }
                return Ordering::Equal;
            }
        }
    }
}

impl PartialEq for JokerHand {
    fn eq(&self, other: &Self) -> bool {
        std::iter::zip(self.cards.iter(), other.cards.iter()).all(|(x, y)| x == y)
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (x, y) in std::iter::zip(self.cards.iter(), other.cards.iter()) {
                    match x.cmp(y) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => (),
                    }
                }
                return Ordering::Equal;
            }
        }
    }
}
#[derive(Debug, Eq)]
struct Line {
    hand: Hand,
    bid: u64,
}

#[derive(Debug, Eq)]
struct JokerLine {
    hand: JokerHand,
    bid: u64,
}

impl PartialEq for JokerLine {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl PartialOrd for JokerLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for JokerLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[derive(Debug)]
struct Puzzle {
    lines: Vec<Line>,
    joker_lines: Vec<JokerLine>,
}

fn classify(cards: &[Card; 5]) -> Rank {
    let mut unique_cards = HashMap::new();
    for card in cards {
        unique_cards
            .entry(card)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    match unique_cards.len() {
        5 => Rank::HighCard,
        4 => Rank::OnePair,
        3 => {
            if unique_cards.iter().any(|(_, &count)| count == 3) {
                Rank::Triple
            } else {
                Rank::TwoPair
            }
        }
        2 => {
            if unique_cards.iter().any(|(_, &count)| count == 1) {
                Rank::FourOfAKind
            } else {
                Rank::FullHouse
            }
        }
        1 => Rank::FiveOfAKind,
        _ => todo!(),
    }
}

fn classify_joker(cards: &[Card; 5]) -> Rank {
    let joker_count = cards.iter().filter(|&&card| card == Card::Joker).count();
    let non_jokers = cards
        .iter()
        .filter(|&&card| card != Card::Joker)
        .collect::<Vec<_>>();
    match joker_count {
        5 => Rank::FiveOfAKind,
        4 => Rank::FiveOfAKind,
        3 => {
            if non_jokers[0] == non_jokers[1] {
                Rank::FiveOfAKind
            } else {
                Rank::FourOfAKind
            }
        }
        2 => {
            let mut test_cards = [Card::Joker; 5];
            test_cards[0] = *non_jokers[0];
            test_cards[1] = *non_jokers[1];
            test_cards[2] = *non_jokers[2];
            NON_JOKERS
                .iter()
                .map(|&joker1| {
                    test_cards[3] = joker1;
                    NON_JOKERS
                        .iter()
                        .map(|&joker2| {
                            test_cards[4] = joker2;
                            classify(&test_cards)
                        })
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        }
        1 => {
            let mut test_cards = [Card::Joker; 5];
            test_cards[0] = *non_jokers[0];
            test_cards[1] = *non_jokers[1];
            test_cards[2] = *non_jokers[2];
            test_cards[3] = *non_jokers[3];
            NON_JOKERS
                .iter()
                .map(|&joker| {
                    test_cards[4] = joker;
                    classify(&test_cards)
                })
                .max()
                .unwrap()
        }
        0 => classify(cards),
        _ => todo!(),
    }
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
                JokerLine {
                    hand: JokerHand { cards, rank },
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
