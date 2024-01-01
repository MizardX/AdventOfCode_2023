use bstr::ByteSlice;
use bstr_parse::{BStrParse, ParseIntError};
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 04");

    println!("++Example");
    let example = parse_input(EXAMPLE).expect("Parse input");
    println!("|+-Part 1: {} (expected 13)", part_1(&example));
    println!("|'-Part 2: {} (expected 30)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT).expect("Parse input");
    println!("|+-Part 1: {} (expected 23235)", part_1(&input));
    println!("|'-Part 2: {} (expected 5920640)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Vec<Card> {
    parse_input(INPUT).expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &[Card]) -> usize {
    let mut sum = 0;
    for card in input {
        sum += card.score();
    }
    sum
}

#[must_use]
pub fn part_2(input: &[Card]) -> usize {
    let counts = &mut [1; 256][..input.len()];
    let mut sum = 0;
    for (i, c) in input.iter().enumerate() {
        let c_count = counts[i];
        for cnt in &mut counts[i + 1..=i + c.matches()] {
            *cnt += c_count;
        }
        sum += c_count;
    }
    sum
}

#[derive(Debug, Clone)]
pub struct Card {
    winning: u128,
    have: u128,
}

impl Card {
    fn matches(&self) -> usize {
        (self.winning & self.have).count_ones() as _
    }

    fn score(&self) -> usize {
        match self.matches() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("'Game ' prefix missing'")]
    MissingFirstPrefix,
    #[error("Missing ':' separator on first line")]
    MissingFirstColon,
    #[error("Missing '|' separator on first line")]
    MissingFirstSeparator,
    #[error("'Game ' prefix missing'")]
    MissingPrefix,
    #[error("Missing ':' separator")]
    MissingColon,
    #[error("Missing '|' separator")]
    MissingSeparator,
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

fn parse_input(text: &str) -> Result<Vec<Card>, ParseInputError> {
    let mut res: Vec<Card> = Vec::with_capacity(256);
    let mut lines = text.as_bytes().lines();
    if let Some(line) = lines.next() {
        #[cfg(debug_assertions)]
        if &line[..5] != b"Card " {
            return Err(ParseInputError::MissingFirstPrefix);
        }
        let start_winning = if let Some(colon) = line[5..].find_byte(b':') {
            colon + 7
        } else {
            return Err(ParseInputError::MissingFirstColon);
        };
        let end_winning = if let Some(bar) = line[start_winning..].find_byte(b'|') {
            bar + start_winning - 1
        } else {
            return Err(ParseInputError::MissingFirstSeparator);
        };
        let start_have = end_winning + 3;
        let end_have = line.len();
        let winning = (start_winning..end_winning)
            .step_by(3)
            .map(|i| line[i..i + 2].trim_ascii_start().parse::<u8>())
            .try_fold(0_u128, |s, n| {
                Result::<_, ParseInputError>::Ok(s | (1_u128 << n?))
            })?;
        let have = (start_have..end_have)
            .step_by(3)
            .map(|i| line[i..i + 2].trim_ascii_start().parse::<u8>())
            .try_fold(0_u128, |s, n| {
                Result::<_, ParseInputError>::Ok(s | (1_u128 << n?))
            })?;
        res.push(Card { winning, have });
        for line in lines {
            #[cfg(debug_assertions)]
            if &line[..5] != b"Card " {
                return Err(ParseInputError::MissingPrefix);
            }
            #[cfg(debug_assertions)]
            if &line[start_winning - 2..start_winning] != b": " {
                return Err(ParseInputError::MissingColon);
            }
            #[cfg(debug_assertions)]
            if &line[start_have - 3..start_have] != b" | " {
                return Err(ParseInputError::MissingSeparator);
            }
            let winning = (start_winning..end_winning)
                .step_by(3)
                .map(|i| line[i..i + 2].trim_ascii_start().parse::<u8>())
                .try_fold(0_u128, |s, n| {
                    Result::<_, ParseInputError>::Ok(s | (1_u128 << n?))
                })?;
            let have = (start_have..end_have)
                .step_by(3)
                .map(|i| line[i..i + 2].trim_ascii_start().parse::<u8>())
                .try_fold(0_u128, |s, n| {
                    Result::<_, ParseInputError>::Ok(s | (1_u128 << n?))
                })?;
            res.push(Card { winning, have });
        }
    }
    Ok(res)
}
