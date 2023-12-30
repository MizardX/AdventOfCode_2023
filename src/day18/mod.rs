#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::str::{Bytes, FromStr};
use thiserror::Error;

use crate::aoclib::{Dir, MultiDir, Pos};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if input is malformed.
pub fn run() {
    println!(".Day 18");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 62)", part_1(&example));
    println!("|'-Part 2: {} (expected 952 408 144 115)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 47 139)", part_1(&input));
    println!(
        "|'-Part 2: {} (expected 173 152 345 887 206)",
        part_2(&input)
    );
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> i64 {
    sum_enclosed_area(input.instructions.iter().map(|instr| instr.movement))
}

#[must_use]
pub fn part_2(input: &Input) -> i64 {
    sum_enclosed_area(input.instructions.iter().map(|instr| instr.alt_movement))
}

fn sum_enclosed_area(it: impl Iterator<Item = MultiDir>) -> i64 {
    let mut pos = Pos::new(0, 0);
    let mut area = 0;
    let mut border = 0;
    for movement in it {
        let next_pos = pos + movement;
        area += (pos.col() * next_pos.row() - next_pos.col() * pos.row()) as i64;
        border += i64::try_from(movement.count()).unwrap();
        pos = next_pos;
    }
    (area.abs() + border) / 2 + 1 // +1 for the corners
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    movement: MultiDir,
    alt_movement: MultiDir,
}

impl FromStr for Instruction {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn expect(bs: &mut Bytes<'_>, check: fn(&u8) -> bool) -> Result<u8, ParseInputError> {
            match bs.next() {
                None => Err(ParseInputError::EmptyInput),
                Some(ch) if check(&ch) => Ok(ch),
                Some(ch) => Err(ParseInputError::InvalidChar(ch as char)),
            }
        }
        fn expect_hex_digit(bs: &mut Bytes<'_>) -> Result<usize, ParseInputError> {
            match bs.next() {
                None => Err(ParseInputError::EmptyInput),
                Some(ch) if ch.is_ascii_digit() => Ok((ch - b'0') as _),
                Some(ch @ b'A'..=b'F') => Ok((ch - b'A' + 10) as _),
                Some(ch @ b'a'..=b'f') => Ok((ch - b'a' + 10) as _),
                Some(ch) => Err(ParseInputError::InvalidChar(ch as char)),
            }
        }

        let mut bs = s.bytes();
        let dir = match bs.next() {
            Some(b'L') => Dir::W,
            Some(b'U') => Dir::N,
            Some(b'R') => Dir::E,
            Some(b'D') => Dir::S,
            Some(ch) => return Err(ParseInputError::InvalidChar(ch as char)),
            None => return Err(ParseInputError::EmptyInput),
        };
        expect(&mut bs, u8::is_ascii_whitespace)?;
        let mut dist = (expect(&mut bs, u8::is_ascii_digit)? - b'0') as usize;
        dist = match bs.next() {
            None => return Err(ParseInputError::EmptyInput),
            Some(d) if d.is_ascii_digit() => {
                expect(&mut bs, u8::is_ascii_whitespace)?;
                dist * 10 + (d - b'0') as usize
            }
            Some(w) if w.is_ascii_whitespace() => dist,
            Some(ch) => return Err(ParseInputError::InvalidChar(ch as char)),
        };
        expect(&mut bs, |&ch| ch == b'(')?;
        expect(&mut bs, |&ch| ch == b'#')?;
        let mut alt_distance = expect_hex_digit(&mut bs)? << 16;
        alt_distance |= expect_hex_digit(&mut bs)? << 12;
        alt_distance |= expect_hex_digit(&mut bs)? << 8;
        alt_distance |= expect_hex_digit(&mut bs)? << 4;
        alt_distance |= expect_hex_digit(&mut bs)?;
        let alt_direction = match bs.next() {
            Some(b'0') => Dir::E,
            Some(b'1') => Dir::S,
            Some(b'2') => Dir::W,
            Some(b'3') => Dir::N,
            Some(ch) => return Err(ParseInputError::InvalidChar(ch as char)),
            None => return Err(ParseInputError::EmptyInput),
        };
        expect(&mut bs, |&ch| ch == b')')?;

        let movement = dir * dist;
        let alt_movement = alt_direction * alt_distance;

        Ok(Self {
            movement,
            alt_movement,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let instructions = text.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { instructions })
    }
}

/// # Panics
///
/// Panics if input is malformed.

#[must_use]
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Parse input")
}
