#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::num::TryFromIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::aoclib::{Grid, Pos, Dir, CommonErrors};

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const EXAMPLE3: &str = include_str!("example3.txt");
const EXAMPLE4: &str = include_str!("example4.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 10");

    println!("++Example1");
    let example1 = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 4)", part_1(&example1));

    println!("++Example2");
    let example2 = EXAMPLE2.parse().expect("Parse example 2");
    println!("|+-Part 1: {} (expected 8)", part_1(&example2));

    println!("++Example3");
    let mut example3 = EXAMPLE3.parse().expect("Parse example 3");
    println!("|+-Part 2: {} (expected 4)", part_2(&mut example3));

    println!("++Example4");
    let mut example4 = EXAMPLE4.parse().expect("Parse example 4");
    println!("|+-Part 2: {} (expected 10)", part_2(&mut example4));

    println!("++Input");
    let mut input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6717)", part_1(&input));
    println!("|'-Part 2: {} (expected 381)", part_2(&mut input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    // Pick one direction for the start tile
    let enter = input
        .neighbors(input.start)
        .into_iter()
        .flatten()
        .next()
        .unwrap()
        .0
        .reverse();

    let mut dist = 0;
    let mut pos = input.start;
    let mut dir = enter;
    loop {
        let (next_dir, next) = input
            .neighbors(pos)
            .into_iter()
            .flatten()
            .find(|(next_dir, _)| next_dir.reverse() != dir)
            .unwrap();
        dist += 1;
        pos = next;
        dir = next_dir;
        if pos == input.start {
            break;
        }
    }
    dist / 2
}

fn part_2(input: &mut Input) -> usize {
    let enter = input
        .neighbors(input.start)
        .into_iter()
        .flatten()
        .next()
        .unwrap()
        .0
        .reverse();

    let mut pos = input.start;
    let mut dir = enter;
    loop {
        input.grid.map(pos, Pipe::to_pipe);
        let (next_dir, next) = input
            .neighbors(pos)
            .into_iter()
            .flatten()
            .find(|(next_dir, _)| next_dir.reverse() != dir)
            .unwrap();
        pos = next;
        dir = next_dir;
        if pos == input.start {
            break;
        }
    }
    let mut count_inside = 0;
    let mut inside = false;
    let mut from_above = false;
    for r in 0..isize::try_from(input.grid.height()).unwrap() {
        for &cell in input.grid.get_row(r).unwrap() {
            match cell {
                Pipe::PNS => {
                    inside = !inside;
                }
                Pipe::PEW => (),
                Pipe::PNE => {
                    from_above = true;
                }
                Pipe::PSE => {
                    from_above = false;
                }
                Pipe::PNW => {
                    if !from_above {
                        inside = !inside;
                    }
                }
                Pipe::PSW => {
                    if from_above {
                        inside = !inside;
                    }
                }
                _ if inside => {
                    count_inside += 1;
                }
                _ => (),
            }
        }
    }

    count_inside
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
    /// Path Vertical North-South
    PNS,
    /// Path Horizontal East-West
    PEW,
    /// Path North-East turn
    PNE,
    /// Path North-West turn
    PNW,
    /// Path South-West turn
    PSW,
    /// Path South-Eeast turn
    PSE,
}

impl Pipe {
    /// If this pipe has a connection in given direction
    pub const fn connected(self, dir: Dir) -> bool {
        matches!(
            (self, dir),
            (Pipe::S, Dir::E | Dir::N | Dir::S | Dir::W)
                | (Pipe::NE | Pipe::PNE, Dir::E | Dir::N)
                | (Pipe::PSE | Pipe::SE, Dir::E | Dir::S)
                | (Pipe::EW | Pipe::PEW, Dir::E | Dir::W)
                | (Pipe::NS | Pipe::PNS, Dir::N | Dir::S)
                | (Pipe::NW | Pipe::PNW, Dir::N | Dir::W)
                | (Pipe::PSW | Pipe::SW, Dir::S | Dir::W)
        )
    }

    pub const fn to_pipe(self) -> Self {
        match self {
            Pipe::NS => Pipe::PNS,
            Pipe::EW => Pipe::PEW,
            Pipe::NE => Pipe::PNE,
            Pipe::NW => Pipe::PNW,
            Pipe::SW => Pipe::PSW,
            Pipe::SE => Pipe::PSE,
            _ => self,
        }
    }
}

impl TryFrom<u8> for Pipe {
    type Error = ParseError;

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
            ch => return Err(ParseError::InvalidSymbol(ch as char)),
        })
    }
}

#[derive(Debug, Clone)]
struct Input {
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
enum ParseError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(char),
    #[error("Missing start")]
    MissingStart,
    #[error("Integer overflow: {0}")]
    Overflow(#[from] TryFromIntError),
}

impl CommonErrors for ParseError {
    fn empty_input() -> Self {
        Self::EmptyInput
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid<Pipe> = s.parse()?;
        let start = grid.position(|p| p == Pipe::S).unwrap();
        if start == Pos::new(-1, -1) {
            return Err(ParseError::MissingStart);
        }
        let start_pipe = match [Dir::N, Dir::E, Dir::S, Dir::W]
            .map(|d| grid.get(start + d).is_some_and(|p| p.connected(d.reverse())))
        {
            [true, true, false, false] => Pipe::NE,
            [true, false, true, false] => Pipe::NS,
            [true, false, false, true] => Pipe::NW,
            [false, true, true, false] => Pipe::SE,
            [false, true, false, true] => Pipe::EW,
            [false, false, true, true] => Pipe::SW,
            _ => return Err(ParseError::MissingStart),
        };
        grid.set(start, start_pipe);
        Ok(Self { grid, start })
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
        let mut input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_2(&mut input)));
    }
}
