#![warn(clippy::pedantic)]

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

pub fn run() {
    println!(".Day 07");

    println!("++Example");
    let mut example = parse_input(include_str!("example.txt"));
    println!("|+-Part 1: {} (expected 6440)", part_1(&mut example));
    println!("|'-Part 2: {} (expected 5905)", part_2(&mut example));

    println!("++Input");
    let mut input = parse_input(include_str!("input.txt"));
    println!("|+-Part 1: {} (expected 253313241)", part_1(&mut input));
    println!("|'-Part 2: {} (expected 253362743)", part_2(&mut input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &mut [Input]) -> u64 {
    input.sort_unstable_by_key(Input::score);
    let mut sum = 0;
    for (i, hand) in input.iter().enumerate() {
        sum += (i + 1) as u64 * hand.bet;
    }
    sum
}

#[allow(unused)]
fn part_2(input: &mut [Input]) -> u64 {
    input.sort_unstable_by_key(Input::score_joker);
    let mut sum = 0;
    for (i, hand) in input.iter().enumerate() {
        sum += (i + 1) as u64 * hand.bet;
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
    Two = 1,
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
struct Input {
    cards: [Card; 5],
    bet: u64,
}

impl Input {
    pub fn classify(&self) -> HandType {
        let mut cards = self.cards;
        cards.sort_unstable();
        if cards[0] == cards[4] {
            HandType::FiveOfAKind
        } else if cards[0] == cards[3] || cards[1] == cards[4] {
            HandType::FourOfAKind
        } else if cards[0] == cards[1]
            && cards[3] == cards[4]
            && (cards[1] == cards[2] || cards[2] == cards[3])
        {
            HandType::FullHouse
        } else if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            HandType::ThreeOfAKind
        } else {
            let num_pairs = cards.windows(2).filter(|w| w[0] == w[1]).count();
            if num_pairs == 2 {
                HandType::TwoPairs
            } else if num_pairs == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }

    pub fn classify_joker(&self) -> HandType {
        let mut cards = self.cards;
        cards.sort_unstable();
        let mut jokers = 0;
        for c in &cards {
            if let Card::Jack = c {
                jokers += 1;
            }
        }
        if cards[0] == cards[4] {
            HandType::FiveOfAKind
        } else if cards[0] == cards[3] || cards[1] == cards[4] {
            match jokers {
                0 => HandType::FourOfAKind,
                _ => HandType::FiveOfAKind,
            }
        } else if cards[0] == cards[1]
            && cards[3] == cards[4]
            && (cards[1] == cards[2] || cards[2] == cards[3])
        {
            match jokers {
                0 => HandType::FullHouse,
                _ => HandType::FiveOfAKind,
            }
        } else if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            match jokers {
                0 => HandType::ThreeOfAKind,
                _ => HandType::FourOfAKind,
            }
        } else {
            let num_pairs = cards.windows(2).filter(|w| w[0] == w[1]).count();
            match (num_pairs, jokers) {
                (2, 0) => HandType::TwoPairs,
                (2, 1) => HandType::FullHouse,
                (2, 2) => HandType::FourOfAKind,
                (1, 0) | (0, 1) => HandType::OnePair,
                (1, 1 | 2) => HandType::ThreeOfAKind,
                _ => HandType::HighCard,
            }
        }
    }

    pub fn score(&self) -> u32 {
        self.cards
            .iter()
            .fold(u32::from(self.classify() as u8), |s, &c| {
                s * 0x10 + u32::from(c as u8)
            })
    }

    pub fn score_joker(&self) -> u32 {
        self.cards
            .iter()
            .fold(u32::from(self.classify_joker() as u8), |s, &c| match c {
                Card::Jack => s * 0x10,
                c => s * 0x10 + u32::from(c as u8),
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
        for (i, &b) in bytes.iter().enumerate().take(5) {
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
