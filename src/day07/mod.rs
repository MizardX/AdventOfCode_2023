#![warn(clippy::pedantic)]

use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

pub fn run() {
    println!(".Day 07");

    println!("++Example");
    let example = parse_input(include_str!("example.txt"));
    println!("|+-Part 1: {} (expected 6440)", part_1(&example));
    println!("|'-Part 2: {} (expected 5905)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt"));
    println!("|+-Part 1: {} (expected 253313241)", part_1(&input));
    println!("|'-Part 2: {} (expected 253362743)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Input]) -> u64 {
    let mut hands = input.to_vec();
    hands.sort_unstable();
    let mut sum = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        sum += (i + 1) as u64 * hand.bid;
    }
    sum
}

#[allow(unused)]
fn part_2(input: &[Input]) -> u64 {
    let mut hands = input
        .into_iter()
        .cloned()
        .map(WithJokers)
        .collect::<Vec<_>>();
    hands.sort_unstable();
    let mut sum = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        sum += (i + 1) as u64 * hand.0.bid;
    }
    sum
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Invalid card: {0}")]
    InvalidCard(char),
    #[error("Input line too short")]
    TooShort,
    #[error("No separator (' ')")]
    MissingSeparator,
    #[error("Invalid bid integer: {0}")]
    InvalidBid(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<u8> for Card {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::Ten,
            b'J' => Card::Jack,
            b'Q' => Card::Queen,
            b'K' => Card::King,
            b'A' => Card::Ace,
            _ => return Err(ParseInputError::InvalidCard(value as char)),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Default, Eq)]
struct Input {
    cards: [Card; 5],
    bid: u64,
}

impl Input {
    pub fn classify(&self) -> HandType {
        let mut counts = [0; 13];
        for &card in &self.cards {
            counts[card as u8 as usize] += 1;
        }
        counts.sort_unstable();
        match (counts[11], counts[12]) {
            (_, 5) => HandType::FiveOfAKind,
            (_, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (_, 3) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPairs,
            (_, 2) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
    pub fn classify_joker(&self) -> HandType {
        let mut counts = [0_u8; 13];
        for &card in &self.cards {
            counts[card as u8 as usize] += 1;
        }
        let jokers = std::mem::take(&mut counts[Card::Jack as u8 as usize]);
        counts.sort_unstable();
        match (counts[11], counts[12], jokers) {
            (_, 5, _) | (_, 4, 1) | (_, 3, 2) | (_, 2, 3) | (_, _, 4 | 5) => HandType::FiveOfAKind,
            (_, 4, _) | (_, 3, 1) | (_, 2, 2) | (_, _, 3) => HandType::FourOfAKind,
            (2, 3, _) | (2, 2, 1) => HandType::FullHouse,
            (_, 3, _) | (_, 2, 1) | (_, _, 2) => HandType::ThreeOfAKind,
            (2, 2, _) => HandType::TwoPairs,
            (_, 2, _) | (_, _, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.classify(), self.cards).cmp(&(other.classify(), other.cards))
    }
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() < 7 {
            return Err(ParseInputError::TooShort);
        }
        let mut res = Self::default();
        for (i, &b) in bytes.iter().enumerate().take(5) {
            res.cards[i] = b.try_into()?;
        }
        if bytes[5] != b' ' {
            return Err(ParseInputError::MissingSeparator);
        }
        res.bid = s[6..].parse()?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct WithJokers(Input);

impl Ord for WithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.0.classify_joker().cmp(&other.0.classify_joker());
        if cmp.is_ne() {
            return cmp;
        }
        self.0
            .cards
            .iter()
            .zip(other.0.cards.iter())
            .map(|(a, b)| match (a, b) {
                (Card::Jack, Card::Jack) => Ordering::Equal,
                (Card::Jack, _) => Ordering::Less,
                (_, Card::Jack) => Ordering::Greater,
                (a, b) => a.cmp(b),
            })
            .find(|o| o.is_ne())
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for WithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(text: &str) -> Vec<Input> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
        res.push(line.parse().expect("Valid input"));
    }
    res
}
