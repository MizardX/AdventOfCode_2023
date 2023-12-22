#![warn(clippy::pedantic)]

use std::collections::HashSet;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{Grid, Pos};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 22");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 5)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 465)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[allow(clippy::cast_possible_wrap)]
fn part_1(board: &Board) -> usize {
    let mut grid: Grid<Option<usize>> = Grid::new(
        (board.max.x - board.min.x + 1) as usize,
        (board.max.y - board.min.y + 1) as usize,
    );
    let mut relations: HashSet<(usize, usize)> = HashSet::new();
    let mut z_offset = vec![0; board.pieces.len()];
    let mut below_count = vec![0; board.pieces.len()];
    let mut above_count = vec![0; board.pieces.len()];
    for (piece_ix, piece) in board.pieces.iter().enumerate() {
        let mut max_z = 0;
        for y in piece.low.y..=piece.high.y {
            for x in piece.low.x..=piece.high.x {
                let below_ix = grid
                    .get_mut(Pos::new(
                        x as isize - board.min.x as isize,
                        y as isize - board.min.y as isize,
                    ))
                    .unwrap();
                if let Some(below_ix) = *below_ix {
                    let below_high_z = board.pieces[below_ix].high.z - z_offset[below_ix];
                    max_z = max_z.max(below_high_z);
                }
            }
        }
        for y in piece.low.y..=piece.high.y {
            for x in piece.low.x..=piece.high.x {
                let below_ix = grid
                    .get_mut(Pos::new(
                        x as isize - board.min.x as isize,
                        y as isize - board.min.y as isize,
                    ))
                    .unwrap();
                if let Some(below_ix) = *below_ix {
                    let below_high_z = board.pieces[below_ix].high.z - z_offset[below_ix];
                    if max_z == below_high_z && relations.insert((piece_ix, below_ix)) {
                        below_count[piece_ix] += 1;
                        above_count[below_ix] += 1;
                    }
                }
                *below_ix = Some(piece_ix);
            }
        }
        z_offset[piece_ix] = piece.low.z - max_z - 1;
    }
    // A piece can be removed iif it is not the only one supporting some brick above
    // remove B iif for all A where B supports A and below_count[A] != 1
    let mut blocks = vec![0; board.pieces.len()];
    for (above_ix, &below_count) in below_count.iter().enumerate() {
        if below_count == 1 {
            for (below_ix, blocks) in blocks.iter_mut().enumerate() {
                if relations.contains(&(above_ix, below_ix)) {
                    *blocks += 1;
                }
            }
        }
    }
    blocks.iter().filter(|&&n| n == 0).count()
}

fn part_2(_input: &Board) -> usize {
    0
}

#[derive(Copy, Clone)]
struct Coordinate {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl Coordinate {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
    }

    pub fn min_fields(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
    pub fn max_fields(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl FromStr for Coordinate {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, rest) = s
            .split_once(',')
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let (y_str, z_str) = rest
            .split_once(',')
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let x = x_str.parse()?;
        let y = y_str.parse()?;
        let z = z_str.parse()?;
        Ok(Self::new(x, y, z))
    }
}

#[derive(Clone)]
struct Piece {
    pub low: Coordinate,
    pub high: Coordinate,
}

impl Piece {
    fn new(start: Coordinate, end: Coordinate) -> Self {
        Self {
            low: start,
            high: end,
        }
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?} : {:?}]", self.low, self.high)
    }
}

impl FromStr for Piece {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .split_once('~')
            .ok_or(ParseInputError::ExpectedChar('~'))?;
        let start: Coordinate = start_str.parse()?;
        let end: Coordinate = end_str.parse()?;
        debug_assert!(start.x <= end.x);
        debug_assert!(start.y <= end.y);
        debug_assert!(start.z <= end.z);
        Ok(Self::new(start, end))
    }
}

#[derive(Debug, Clone)]
struct Board {
    pub pieces: Vec<Piece>,
    pub min: Coordinate,
    pub max: Coordinate,
}

impl Board {
    fn new(pieces: Vec<Piece>, min: Coordinate, max: Coordinate) -> Self {
        Self { pieces, min, max }
    }
}

impl FromStr for Board {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut min = Coordinate::new(u16::MAX, u16::MAX, u16::MAX);
        let mut max = Coordinate::new(u16::MIN, u16::MIN, u16::MIN);
        let mut pieces = Vec::with_capacity(1400);
        for line in text.lines() {
            let piece: Piece = line.parse()?;
            min = min.min_fields(piece.low);
            max = max.max_fields(piece.high);
            pieces.push(piece);
        }
        pieces.sort_unstable_by_key(|p| (p.high.z, p.low.x, p.low.y));
        Ok(Self::new(pieces, min, max))
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
    #[error("Expected character: '{0}'")]
    ExpectedChar(char),
    #[error("Not an integer: {0:?}")]
    Integer(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(INPUT.parse::<Board>().expect("Parse input")));
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
