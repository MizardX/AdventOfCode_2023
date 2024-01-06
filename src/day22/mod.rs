use bstr::ByteSlice;
use smallvec::SmallVec;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonParseError, Grid, Pos};

type Coordinate = crate::aoclib::Coordinate<u16>;

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

#[must_use]
pub fn parse_test_input() -> Board {
    INPUT.parse().expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn part_1(board: &Board) -> usize {
    let mut simulator = Simulator::new(board);
    simulator.settle();
    simulator.count_critical()
}

#[must_use]
pub fn part_2(board: &Board) -> usize {
    let mut simulator = Simulator::new(board);
    simulator.settle();
    simulator.sum_knocked_down()
}

#[derive(Debug)]
struct Simulator<'a> {
    board: &'a Board,
    z_offset: Vec<u16>,
    touching_from_above: Vec<SmallVec<[usize; 4]>>,
    touching_from_below: Vec<SmallVec<[usize; 4]>>,
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
            touching_from_above: touching_above,
            touching_from_below: touching_below,
        }
    }

    fn get_piece_high_z(&self, piece_ix: usize) -> u16 {
        self.board.pieces[piece_ix].high.z - self.z_offset[piece_ix]
    }

    fn lower_piece_onto(&mut self, piece_ix: usize, max_z: u16) {
        self.z_offset[piece_ix] = self.board.pieces[piece_ix].low.z - (max_z + 1);
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn settle(&mut self) {
        // Grid to keep track of the topmost brick in each column
        let mut grid: Grid<Option<usize>> = Grid::new(
            (self.board.max.x - self.board.min.x + 1) as usize,
            (self.board.max.y - self.board.min.y + 1) as usize,
        );

        // Bottom to top
        for (piece_ix, piece) in self.board.pieces.iter().enumerate() {
            // Find the height of the highest brick below this one
            let mut max_z = 0;
            for y in piece.low.y..=piece.high.y {
                for x in piece.low.x..=piece.high.x {
                    let column_below_ix = grid
                        .get_mut(Pos::new(
                            x as isize - self.board.min.x as isize,
                            y as isize - self.board.min.y as isize,
                        ))
                        .unwrap();

                    if let Some(column_below_ix) = *column_below_ix {
                        // There is a brick in this column. Compare it's top Z value
                        let below_high_z = self.get_piece_high_z(column_below_ix);
                        max_z = max_z.max(below_high_z);
                    }
                }
            }

            // Connect those that are high enough to touch where the new brick will land
            for y in piece.low.y..=piece.high.y {
                for x in piece.low.x..=piece.high.x {
                    let column_below_ix = grid
                        .get_mut(Pos::new(
                            x as isize - self.board.min.x as isize,
                            y as isize - self.board.min.y as isize,
                        ))
                        .unwrap();

                    if let Some(column_below_ix) = *column_below_ix {
                        // There is a brick in this column. Check if it reaches the new brick
                        let below_high_z = self.get_piece_high_z(column_below_ix);

                        if max_z == below_high_z
                            && !self.touching_from_below[piece_ix].contains(&column_below_ix)
                        {
                            // Connect them
                            self.touching_from_below[piece_ix].push(column_below_ix);
                            self.touching_from_above[column_below_ix].push(piece_ix);
                        }
                    }

                    // Update the top piece in the column
                    *column_below_ix = Some(piece_ix);
                }
            }

            self.lower_piece_onto(piece_ix, max_z);
        }
    }

    pub fn count_critical(&self) -> usize {
        let n = self.board.pieces.len();
        let mut is_critical: Vec<bool> = vec![false; n];
        let mut non_critical_count = n;
        for below_ixs in &self.touching_from_below {
            if let [single_below_ix] = below_ixs[..] {
                // Some piece has only one supporter (single_below_ix)
                if !is_critical[single_below_ix] {
                    is_critical[single_below_ix] = true;
                    non_critical_count -= 1;
                }
            }
        }
        non_critical_count
    }

    pub fn sum_knocked_down(&self) -> usize {
        let n = self.board.pieces.len();
        let mut count_falling = 0;
        let mut falling = vec![false; n];
        // Bottom to top
        for piece_ix in 0..n {
            for x in &mut falling {
                *x = false;
            }
            falling[piece_ix] = true;

            // Every piece above this one
            'falling: for falling_ix in piece_ix..n {
                if self.touching_from_below[falling_ix].is_empty() {
                    // Ground piece. It has no non-falling supporters, but it is supported by the ground.
                    continue;
                }

                for &below_ix in &self.touching_from_below[falling_ix] {
                    if !falling[below_ix] {
                        // A non-falling supporter, so this piece will not fall.
                        continue 'falling;
                    }
                }

                // This piece is now falling
                falling[falling_ix] = true;
                count_falling += 1;
            }
        }

        count_falling
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

impl<'a> TryFrom<&'a [u8]> for Piece {
    type Error = ParseInputError;

    fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        let tilde = s
            .find_byte(b'~')
            .ok_or(ParseInputError::ExpectedChar('~'))?;
        let start: Coordinate = s[..tilde].try_into()?;
        let end: Coordinate = s[tilde + 1..].try_into()?;
        debug_assert!(start.x <= end.x);
        debug_assert!(start.y <= end.y);
        debug_assert!(start.z <= end.z);
        Ok(Self::new(start, end))
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pieces: Vec<Piece>,
    min: Coordinate,
    max: Coordinate,
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
        for line in text.as_bytes().lines() {
            let piece: Piece = line.try_into()?;
            min = min.min_fields(piece.low);
            max = max.max_fields(piece.high);
            pieces.push(piece);
        }
        pieces.sort_unstable_by_key(|p| (p.high.z, p.low.x, p.low.y));
        Ok(Self::new(pieces, min, max))
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Expected character: '{0}'")]
    ExpectedChar(char),
    #[error("Not an integer: {0:?}")]
    Integer(#[from] ParseIntError),
    #[error("{0:?}")]
    CommonError(#[from] CommonParseError),
}
