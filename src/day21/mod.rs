#![warn(clippy::pedantic)]

use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonErrors, Dir, Grid, Pos, RepeatingGrid};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 21");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1 (6): {} (expected 16)", part_1(&example, 6));
    println!(
        "|'-Part 2 (500): {} (expected 167 004)",
        part_2(&example, 500)
    );
    println!(
        "|'-Part 2 (1 000): {} (expected 668 697)",
        part_2(&example, 1000)
    );
    println!(
        "|'-Part 2 (5 000): {} (expected 16 733 044)",
        part_2(&example, 5000)
    );

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1 (64): {} (expected 3 639)", part_1(&input, 64));
    println!(
        "|'-Part 2 (26 501 365): {} (expected 604 592 315 958 630)",
        part_2(&input, 26_501_365)
    );
    println!("')");
}

fn part_1(garden: &Garden, target_dist: usize) -> i64 {
    plots_after_steps(garden, target_dist)
}

fn part_2(garden: &Garden, target_dist: usize) -> i64 {
    plots_after_steps(garden, target_dist)
}

#[allow(clippy::cast_possible_wrap)]
fn plots_after_steps(garden: &Garden, target_dist: usize) -> i64 {
    let mut walker = Walker::new(&garden.grid, garden.start_pos);
    let size = garden.grid.width();
    let mut samples = Vec::with_capacity(size * 4);
    for step in 0.. {
        if step == target_dist {
            return walker.len() as i64;
        }
        if step % size == target_dist % size {
            samples.push(walker.len() as i64);
            if samples.len() > 3 {
                let n = samples.len();
                let [x1, x2, x3, x4] = samples[n - 4..n] else {
                    unreachable!()
                };
                if x4 - 3 * x3 + 3 * x2 - x1 == 0 {
                    debug_assert_eq!((target_dist - step) % size, 0);
                    let delta = ((target_dist - step) / size + 3) as i64;
                    let a = (x1 - 2 * x2 + x3) / 2;
                    let b = (-3 * x1 + 4 * x2 - x3) / 2;
                    let c = x1;
                    return (a * delta + b) * delta + c;
                }
            }
        }
        walker.take_step();
    }
    unreachable!()
}

struct Walker<'a> {
    grid: RepeatingGrid<'a, Tile>,
    current: HashSet<Pos>,
    next: HashSet<Pos>,
    current_fringe: HashSet<Pos>,
    next_fringe: HashSet<Pos>,
}

impl<'a> Walker<'a> {
    pub fn new(grid: &'a Grid<Tile>, start_pos: Pos) -> Self {
        let mut current = HashSet::new();
        let mut current_fringe = HashSet::new();
        let next = HashSet::new();
        let next_fringe = HashSet::new();
        current_fringe.insert(start_pos);
        current.insert(start_pos);
        Self {
            grid: RepeatingGrid::new(grid),
            current,
            next,
            current_fringe,
            next_fringe,
        }
    }

    pub fn take_step(&mut self) {
        self.next_fringe.clear();
        for &pos in &self.current_fringe {
            for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
                let next_pos = pos + dir;
                if matches!(self.grid[next_pos], Tile::GardenPlot) && self.next.insert(next_pos) {
                    self.next_fringe.insert(next_pos);
                }
            }
        }
        std::mem::swap(&mut self.current, &mut self.next);
        std::mem::swap(&mut self.current_fringe, &mut self.next_fringe);
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
        b.iter(|| black_box(part_2(&input, 26_501_365)));
    }
}
