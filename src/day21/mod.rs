#![warn(clippy::pedantic)]

use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonErrors, Dir, Grid, Pos, RepeatingGrid};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day XX");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 16)", part_1(&example, 6));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 3639)", part_1(&input, 64));
    // println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

fn part_1(garden: &Garden, target_dist: usize) -> usize {
    let mut walker = Walker::new(&garden.grid, garden.start_pos);
    for _ in 0..target_dist {
        walker.take_step();
    }
    walker.len()
}

fn part_2(_garden: &Garden) -> usize {
    0
}

struct Walker<'a> {
    grid: RepeatingGrid<'a, Tile>,
    current: Vec<Pos>,
    next: Vec<Pos>,
    next_set: HashSet<Pos>,
}

impl<'a> Walker<'a> {
    pub fn new(grid: &'a Grid<Tile>, start_pos: Pos) -> Self {
        let mut current = Vec::with_capacity(256);
        current.push(start_pos);
        Self {
            grid: RepeatingGrid::new(grid),
            current,
            next: Vec::with_capacity(256),
            next_set: HashSet::new(),
        }
    }

    pub fn take_step(&mut self) {
        self.next_set.clear();
        for &pos in &self.current {
            for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
                let next_pos = pos + dir;
                if matches!(self.grid[next_pos], Tile::GardenPlot) && self.next_set.insert(next_pos)
                {
                    self.next.push(next_pos);
                }
            }
        }
        self.current.clear();
        std::mem::swap(&mut self.current, &mut self.next);
    }

    pub fn len(&self) -> usize {
        self.current.len()
    }
}

#[derive(Clone, Copy)]
enum Tile {
    GardenPlot,
    Rock,
    Start,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GardenPlot => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Start => write!(f, "S"),
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Tile::GardenPlot,
            b'#' => Tile::Rock,
            b'S' => Tile::Start,
            ch => return Err(ParseInputError::InvalidChar(ch as char)),
        })
    }
}

#[derive(Debug, Clone)]
struct Garden {
    grid: Grid<Tile>,
    start_pos: Pos,
}

impl Garden {
    fn new(grid: Grid<Tile>, start_pos: Pos) -> Self {
        Self { grid, start_pos }
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl CommonErrors for ParseInputError {
    fn empty_input() -> Self {
        Self::EmptyInput
    }
}

impl FromStr for Garden {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid<Tile> = text.parse()?;
        let start_pos = grid.position(|p| matches!(p, Tile::Start)).unwrap();
        grid.set(start_pos, Tile::GardenPlot);
        Ok(Self::new(grid, start_pos))
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(INPUT.parse::<Garden>().expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_1(&input, 64)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
