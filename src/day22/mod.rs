#![warn(clippy::pedantic)]

use smallvec::SmallVec;
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
    println!("|'-Part 2: {} (expected 7)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 465)", part_1(&input));
    println!("|'-Part 2: {} (expected 79 042)", part_2(&input));
    println!("')");
}

#[allow(clippy::cast_possible_wrap)]
fn part_1(board: &Board) -> usize {
    let mut simulator = Simulator::new(board);
    simulator.settle();
    simulator.count_critical()
}

fn part_2(board: &Board) -> usize {
    let mut simulator = Simulator::new(board);
    simulator.settle();
    simulator.sum_knocked_down()
}

#[derive(Debug)]
struct Simulator<'a> {
    board: &'a Board,
    z_offset: Vec<u16>,
    touching_above: Vec<SmallVec<[usize; 4]>>,
    touching_below: Vec<SmallVec<[usize; 4]>>,
}

impl<'a> Simulator<'a> {
    fn new(board: &'a Board) -> Self {
        let n = board.pieces.len();
        let z_offset: Vec<u16> = vec![0; n];
        let touching_above: Vec<SmallVec<[usize; 4]>> = vec![SmallVec::new(); n];
        let touching_below: Vec<SmallVec<[usize; 4]>> = vec![SmallVec::new(); n];
        Self {
            board,
            z_offset,
            touching_above,
            touching_below,
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn settle(&mut self) {
        let mut grid: Grid<Option<usize>> = Grid::new(
            (self.board.max.x - self.board.min.x + 1) as usize,
            (self.board.max.y - self.board.min.y + 1) as usize,
        );
        for (piece_ix, piece) in self.board.pieces.iter().enumerate() {
            let mut max_z = 0;
            for y in piece.low.y..=piece.high.y {
                for x in piece.low.x..=piece.high.x {
                    let below_ix = grid
                        .get_mut(Pos::new(
                            x as isize - self.board.min.x as isize,
                            y as isize - self.board.min.y as isize,
                        ))
                        .unwrap();
                    if let Some(below_ix) = *below_ix {
                        let below_high_z =
                            self.board.pieces[below_ix].high.z - self.z_offset[below_ix];
                        max_z = max_z.max(below_high_z);
                    }
                }
            }
            for y in piece.low.y..=piece.high.y {
                for x in piece.low.x..=piece.high.x {
                    let below_ix = grid
                        .get_mut(Pos::new(
                            x as isize - self.board.min.x as isize,
                            y as isize - self.board.min.y as isize,
                        ))
                        .unwrap();
                    if let Some(below_ix) = *below_ix {
                        let below_high_z =
                            self.board.pieces[below_ix].high.z - self.z_offset[below_ix];
                        if max_z == below_high_z
                            && !self.touching_below[piece_ix].contains(&below_ix)
                        {
                            self.touching_below[piece_ix].push(below_ix);
                            self.touching_above[below_ix].push(piece_ix);
                        }
                    }
                    *below_ix = Some(piece_ix);
                }
            }
            self.z_offset[piece_ix] = piece.low.z - max_z - 1;
        }
    }

    pub fn count_critical(&self) -> usize {
        let n = self.board.pieces.len();
        let mut is_critical: Vec<bool> = vec![false; n];
        for below_ixs in &self.touching_below {
            if below_ixs.len() == 1 {
                is_critical[below_ixs[0]] = true;
            }
        }
        is_critical.iter().filter(|&&c| !c).count()
    }

    pub fn sum_knocked_down(&self) -> usize {
        let n = self.board.pieces.len();
        let mut sum = 0;
        let mut falling = vec![false; n];
        for piece_ix in 0..n {
            for x in &mut falling {
                *x = false;
            }
            falling[piece_ix] = true;
            'falling: for falling_ix in piece_ix..n {
                if self.touching_below[falling_ix].is_empty() {
                    continue;
                }
                for &below_ix in &self.touching_below[falling_ix] {
                    if !falling[below_ix] {
                        continue 'falling;
                    }
                }
                falling[falling_ix] = true;
            }
            sum += falling.iter().filter(|&&f| f).count() - 1;
        }
        sum
    }
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
