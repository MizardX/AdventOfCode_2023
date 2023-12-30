#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::num::TryFromIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::aoclib::{CommonParseError, Dir, Grid, Pos};

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const EXAMPLE3: &str = include_str!("example3.txt");
const EXAMPLE4: &str = include_str!("example4.txt");
const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if input is malformed.
pub fn run() {
    println!(".Day 10");

    println!("++Example1");
    let example1 = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 4)", part_1(&example1));

    println!("++Example2");
    let example2 = EXAMPLE2.parse().expect("Parse example 2");
    println!("|+-Part 1: {} (expected 8)", part_1(&example2));

    println!("++Example3");
    let example3 = EXAMPLE3.parse().expect("Parse example 3");
    println!("|+-Part 2: {} (expected 4)", part_2(&example3));

    println!("++Example4");
    let example4 = EXAMPLE4.parse().expect("Parse example 4");
    println!("|+-Part 2: {} (expected 10)", part_2(&example4));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6717)", part_1(&input));
    println!("|'-Part 2: {} (expected 381)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> isize {
    walk_path(input).1 / 2
}

#[must_use]
pub fn part_2(input: &Input) -> isize {
    walk_path(input).0
}

fn walk_path(input: &Input) -> (isize, isize) {
    let enter = input
        .neighbors(input.start)
        .into_iter()
        .flatten()
        .next()
        .unwrap()
        .0
        .reverse();

    let mut area = 0;
    let mut perimiter = 0;
    let mut pos = input.start;
    let mut dir = enter;
    loop {
        let (next_dir, next) = input
            .neighbors(pos)
            .into_iter()
            .flatten()
            .find(|(next_dir, _)| next_dir.reverse() != dir)
            .unwrap();
        area += pos.col() * next.row() - next.col() * pos.row();
        perimiter += 1;
        pos = next;
        dir = next_dir;
        if pos == input.start {
            break;
        }
    }
    ((area.abs() - perimiter) / 2 + 1, perimiter)
}

/// Underectional pipes
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    /// Empty
    X,
    /// Vertical North-South
    NS,
    /// Horizontal East-West
    EW,
    /// North-East turn
    NE,
    /// North-West turn
    NW,
    /// South-West turn
    SW,
    /// South-Eeast turn
    SE,
    /// Starting position, with unknown direction
    S,
}

impl Pipe {
    /// If this pipe has a connection in given direction
    pub const fn connected(self, dir: Dir) -> bool {
        matches!(
            (self, dir),
            (Pipe::S, Dir::E | Dir::N | Dir::S | Dir::W)
                | (Pipe::NE, Dir::E | Dir::N)
                | (Pipe::SE, Dir::E | Dir::S)
                | (Pipe::EW, Dir::E | Dir::W)
                | (Pipe::NS, Dir::N | Dir::S)
                | (Pipe::NW, Dir::N | Dir::W)
                | (Pipe::SW, Dir::S | Dir::W)
        )
    }
}

impl TryFrom<u8> for Pipe {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'|' => Pipe::NS,
            b'-' => Pipe::EW,
            b'L' => Pipe::NE,
            b'J' => Pipe::NW,
            b'7' => Pipe::SW,
            b'F' => Pipe::SE,
            b'.' => Pipe::X,
            b'S' => Pipe::S,
            ch => return Err(ParseInputError::InvalidSymbol(ch as char)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Pipe>,
    start: Pos,
}

impl Input {
    fn get(&self, pos: Pos) -> Option<Pipe> {
        self.grid.get(pos)
    }

    fn walk(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        let pipe_cur = self.get(pos)?;
        let target = pos + dir;
        let pipe_target = self.get(target)?;
        (pipe_cur.connected(dir) && pipe_target.connected(dir.reverse())).then_some(target)
    }

    fn neighbors(&self, pos: Pos) -> [Option<(Dir, Pos)>; 4] {
        [
            try { (Dir::N, self.walk(pos, Dir::N)?) },
            try { (Dir::E, self.walk(pos, Dir::E)?) },
            try { (Dir::S, self.walk(pos, Dir::S)?) },
            try { (Dir::W, self.walk(pos, Dir::W)?) },
        ]
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(char),
    #[error("Missing start")]
    MissingStart,
    #[error("Integer overflow: {0}")]
    Overflow(#[from] TryFromIntError),
    #[error("{0:?}")]
    CommonError(#[from] CommonParseError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid<Pipe> = s.parse()?;
        let start = grid.position(|p| p == Pipe::S).unwrap();
        if start == Pos::new(-1, -1) {
            return Err(ParseInputError::MissingStart);
        }
        let start_pipe = match [Dir::N, Dir::E, Dir::S, Dir::W].map(|d| {
            grid.get(start + d)
                .is_some_and(|p| p.connected(d.reverse()))
        }) {
            [true, true, false, false] => Pipe::NE,
            [true, false, true, false] => Pipe::NS,
            [true, false, false, true] => Pipe::NW,
            [false, true, true, false] => Pipe::SE,
            [false, true, false, true] => Pipe::EW,
            [false, false, true, true] => Pipe::SW,
            _ => return Err(ParseInputError::MissingStart),
        };
        grid.set(start, start_pipe);
        Ok(Self { grid, start })
    }
}

/// # Panics
///
/// Panics if input is malformed.

#[must_use]
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Real input")
}
