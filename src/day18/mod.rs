#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::str::{Bytes, FromStr};
use thiserror::Error;

use crate::aoclib::{Dir, Grid, MultiDir, Pos};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 18");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 62)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 47139)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let mut pos = Pos::new(0, 0);
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;
    for instr in &input.instructions {
        pos = pos + instr.movement;
        min_row = min_row.min(pos.row());
        max_row = max_row.max(pos.row());
        min_col = min_col.min(pos.col());
        max_col = max_col.max(pos.col());
    }
    let mut grid = Grid::new(max_col.abs_diff(min_col) + 1, max_row.abs_diff(min_row) + 1);
    pos = Pos::new(-min_row, -min_col);
    for instr in &input.instructions {
        for _ in 0..instr.movement.count() {
            pos = pos + instr.movement.dir();
            grid.set(pos, Tile::X);
        }
    }

    let mut area = 0;
    for r in 0..grid.height() {
        let mut inside = false;
        for c in 0..grid.width() {
            let pos = Pos::new(isize::try_from(r).unwrap(), isize::try_from(c).unwrap());
            let below = grid.get(pos + Dir::S).unwrap_or(Tile::O);
            let cell = grid.get_mut(pos).unwrap();
            (inside, *cell, area) = match (inside, *cell, below) {
                (false, Tile::X, Tile::X) | (true, Tile::X, Tile::O) | (true, Tile::O, _) => {
                    (true, Tile::A, area + 1)
                }
                (false, Tile::X, Tile::O) | (true, Tile::X, Tile::X) => (false, Tile::A, area + 1),
                (inside, cell, _) => (inside, cell, area),
            };
        }
    }

    area
}

fn part_2(_input: &Input) -> usize {
    0
}

#[derive(Clone, Copy, Default)]
enum Tile {
    #[default]
    O,
    X,
    A,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::O => write!(f, "."),
            Self::X => write!(f, "#"),
            Self::A => write!(f, "+"),
        }
    }
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Color")
            .field(&self.r)
            .field(&self.g)
            .field(&self.b)
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    movement: MultiDir,
    color: Color,
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
        fn expect_hex_digit(bs: &mut Bytes<'_>) -> Result<u8, ParseInputError> {
            match bs.next() {
                None => Err(ParseInputError::EmptyInput),
                Some(ch) if ch.is_ascii_digit() => Ok(ch - b'0'),
                Some(ch @ b'A'..=b'F') => Ok(ch - b'A' + 10),
                Some(ch @ b'a'..=b'f') => Ok(ch - b'a' + 10),
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
        let mut dist = expect(&mut bs, u8::is_ascii_digit)? - b'0';
        dist = match bs.next() {
            None => return Err(ParseInputError::EmptyInput),
            Some(d) if d.is_ascii_digit() => {
                expect(&mut bs, u8::is_ascii_whitespace)?;
                dist * 10 + (d - b'0')
            }
            Some(w) if w.is_ascii_whitespace() => dist,
            Some(ch) => return Err(ParseInputError::InvalidChar(ch as char)),
        };
        expect(&mut bs, |&ch| ch == b'(')?;
        expect(&mut bs, |&ch| ch == b'#')?;
        let r = expect_hex_digit(&mut bs)? * 16 + expect_hex_digit(&mut bs)?;
        let g = expect_hex_digit(&mut bs)? * 16 + expect_hex_digit(&mut bs)?;
        let b = expect_hex_digit(&mut bs)? * 16 + expect_hex_digit(&mut bs)?;
        expect(&mut bs, |&ch| ch == b')')?;

        let movement = dir * dist;
        let color = Color { r, g, b };

        Ok(Self { movement, color })
    }
}

#[derive(Debug, Clone)]
struct Input {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Error)]
enum ParseInputError {
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

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(INPUT.parse::<Input>().expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
