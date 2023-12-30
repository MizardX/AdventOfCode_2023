#![warn(clippy::pedantic)]

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 07");

    println!("++Example");
    let example = parse_input(EXAMPLE);
    println!("|+-Part 1: {} (expected 6440)", part_1(&example));
    println!("|'-Part 2: {} (expected 5905)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT);
    println!("|+-Part 1: {} (expected 253313241)", part_1(&input));
    println!("|'-Part 2: {} (expected 253362743)", part_2(&input));
    println!("')");
}

#[allow(unused)]
#[must_use]
pub fn part_1(input: &[Input]) -> u64 {
    let mut input = input.to_vec();
    input.sort_unstable_by_key(Input::score);
    let mut sum = 0;
    for (i, hand) in input.iter().enumerate() {
        sum += (i + 1) as u64 * hand.bet;
    }
    sum
}

#[allow(unused)]
#[must_use]
pub fn part_2(input: &[Input]) -> u64 {
    let mut input = input.to_vec();
    input.sort_unstable_by_key(Input::score_joker);
    let mut sum = 0;
    for (i, hand) in input.iter().enumerate() {
        sum += (i + 1) as u64 * hand.bet;
    }
    sum
}

#[derive(Debug, Error)]
pub enum ParseInputError {
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
    Two = 1, // Start at 1, since joker becomes zero when scoring
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum HandType {
    #[default]
    None,
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    cards: [Card; 5],
    bet: u64,
}

impl Input {
    fn classify(&self) -> HandType {
        let c = &self.cards;
        match u8::from(c[0] == c[1])
            + u8::from(c[0] == c[2])
            + u8::from(c[0] == c[3])
            + u8::from(c[0] == c[4])
            + u8::from(c[1] == c[2])
            + u8::from(c[1] == c[3])
            + u8::from(c[1] == c[4])
            + u8::from(c[2] == c[3])
            + u8::from(c[2] == c[4])
            + u8::from(c[3] == c[4])
        {
            10 => HandType::FiveOfAKind,
            6 => HandType::FourOfAKind,
            4 => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 => HandType::TwoPairs,
            1 => HandType::OnePair,
            0 => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    fn classify_joker(&self) -> HandType {
        let mut jokers = 0;
        for c in self.cards {
            jokers += u8::from(c == Card::Jack);
        }
        match (jokers, self.classify()) {
            (0, t) => t,
            (1, HandType::TwoPairs) => HandType::FullHouse,
            (_, HandType::FiveOfAKind | HandType::FourOfAKind | HandType::FullHouse) => {
                HandType::FiveOfAKind
            }
            (_, HandType::ThreeOfAKind | HandType::TwoPairs) => HandType::FourOfAKind,
            (_, HandType::OnePair) => HandType::ThreeOfAKind,
            (_, HandType::HighCard) => HandType::OnePair,
            (_, HandType::None) => HandType::None,
        }
    }

    fn score(&self) -> u32 {
        self.cards
            .iter()
            .fold(u32::from(self.classify() as u8), |s, &c| {
                (s << 4) | u32::from(c as u8)
            })
    }

    fn score_joker(&self) -> u32 {
        self.cards
            .iter()
            .fold(u32::from(self.classify_joker() as u8), |s, &c| match c {
                Card::Jack => s << 4,
                c => (s << 4) | u32::from(c as u8),
            })
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
        for (i, &b) in bytes[0..5].iter().enumerate() {
            res.cards[i] = b.try_into()?;
        }
        if bytes[5] != b' ' {
            return Err(ParseInputError::MissingSeparator);
        }
        res.bet = s[6..].parse()?;
        Ok(res)
    }
}

fn parse_input(text: &str) -> Vec<Input> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
        res.push(line.parse().expect("Valid input"));
    }
    res
}

/// # Panics
///
/// Panics if input is malformed.

#[must_use]
pub fn parse_test_input() -> Vec<Input> {
    parse_input(INPUT)
}
