use bstr::ByteSlice;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{Dir, MultiDir, Pos};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

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
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> i64 {
    sum_enclosed_area(input.instructions.iter().map(|instr| instr.movement))
}

#[must_use]
pub fn part_2(input: &Input) -> i64 {
    sum_enclosed_area(input.instructions.iter().map(|instr| instr.alt_movement))
}

#[allow(clippy::cast_possible_wrap)]
fn sum_enclosed_area(it: impl Iterator<Item = MultiDir>) -> i64 {
    let mut pos = Pos::new(0, 0);
    let mut area = 0;
    let mut perimiter = 0;
    for movement in it {
        let next_pos = pos + movement;
        area += (pos.col() * next_pos.row() - next_pos.col() * pos.row()) as i64; // Shoelace formula
        perimiter += movement.count() as i64;
        pos = next_pos;
    }
    // The sholace area goes out to the center of the tiles. There is an additional half tile border to get
    // the full area. And going the full perimiter, there are four more turns to the right than left, each
    // of which is 1/4 tile, which adds to one full tile for the full perimiter.
    (area.abs() + perimiter) / 2 + 1
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    movement: MultiDir,
    alt_movement: MultiDir,
}

impl<'a> TryFrom<&'a [u8]> for Instruction {
    type Error = ParseInputError;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        fn expect(
            bs: &mut impl Iterator<Item = u8>,
            check: fn(&u8) -> bool,
        ) -> Result<u8, ParseInputError> {
            match bs.next().ok_or(ParseInputError::EmptyInput)? {
                ch if check(&ch) => Ok(ch),
                ch => Err(ParseInputError::InvalidChar(ch as char)),
            }
        }
        fn expect_hex_digit(bs: &mut impl Iterator<Item = u8>) -> Result<usize, ParseInputError> {
            Ok(match bs.next().ok_or(ParseInputError::EmptyInput)? {
                ch if ch.is_ascii_digit() => (ch - b'0') as _,
                ch @ b'A'..=b'F' => (ch - b'A' + 10) as _,
                ch @ b'a'..=b'f' => (ch - b'a' + 10) as _,
                ch => return Err(ParseInputError::InvalidChar(ch as char)),
            })
        }

        let mut bs = s.bytes();
        let dir = match bs.next().ok_or(ParseInputError::EmptyInput)? {
            b'L' => Dir::W,
            b'U' => Dir::N,
            b'R' => Dir::E,
            b'D' => Dir::S,
            ch => return Err(ParseInputError::InvalidChar(ch as char)),
        };
        expect(&mut bs, u8::is_ascii_whitespace)?;
        let mut dist = (expect(&mut bs, u8::is_ascii_digit)? - b'0') as usize;
        dist = match bs.next().ok_or(ParseInputError::EmptyInput)? {
            d if d.is_ascii_digit() => {
                expect(&mut bs, u8::is_ascii_whitespace)?;
                dist * 10 + (d - b'0') as usize
            }
            w if w.is_ascii_whitespace() => dist,
            ch => return Err(ParseInputError::InvalidChar(ch as char)),
        };
        expect(&mut bs, |&ch| ch == b'(')?;
        expect(&mut bs, |&ch| ch == b'#')?;
        let mut alt_distance = expect_hex_digit(&mut bs)? << 16;
        alt_distance |= expect_hex_digit(&mut bs)? << 12;
        alt_distance |= expect_hex_digit(&mut bs)? << 8;
        alt_distance |= expect_hex_digit(&mut bs)? << 4;
        alt_distance |= expect_hex_digit(&mut bs)?;
        let alt_direction = match bs.next().ok_or(ParseInputError::EmptyInput)? {
            b'0' => Dir::E,
            b'1' => Dir::S,
            b'2' => Dir::W,
            b'3' => Dir::N,
            ch => return Err(ParseInputError::InvalidChar(ch as char)),
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
        let mut instructions = Vec::with_capacity(700);
        for line in text.as_bytes().lines() {
            let instr = line.try_into()?;
            instructions.push(instr);
        }
        Ok(Self { instructions })
    }
}
