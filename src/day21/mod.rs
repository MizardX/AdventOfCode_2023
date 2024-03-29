use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonParseError, Dir, Grid, Pos, RepeatingGrid};

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

#[must_use]
pub fn parse_test_input() -> Garden {
    INPUT.parse().expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input, 64));
    black_box(part_2(&input, 26_501_365));
}

#[must_use]
pub fn part_1(garden: &Garden, target_dist: usize) -> i64 {
    plots_after_steps(garden, target_dist, false)
}

#[must_use]
pub fn part_2(garden: &Garden, target_dist: usize) -> i64 {
    plots_after_steps(garden, target_dist, true)
}

#[allow(clippy::cast_possible_wrap)]
fn plots_after_steps(garden: &Garden, target_dist: usize, large: bool) -> i64 {
    let mut walker = Walker::new(&garden.grid, garden.start_pos, large);
    let size = garden.grid.width();
    let mut samples = Vec::with_capacity(6);
    for step in 0.. {
        if step == target_dist {
            return walker.len() as i64;
        }
        if step % size == target_dist % size {
            samples.push(walker.len() as i64);
            // Validate that we have found four values that fit a quadratic function
            #[cfg(debug_assertions)]
            if let [.., x1, x2, x3, x4] = samples[..] {
                if x4 - 3 * x3 + 3 * x2 - x1 == 0 {
                    debug_assert_eq!((target_dist - step) % size, 0);
                    let delta = ((target_dist - step) / size) as i64;
                    let a = x2 - 2 * x3 + x4;
                    let b = x2 - 4 * x3 + 3 * x4;
                    let c = 2 * x4;
                    return ((a * delta + b) * delta + c) / 2;
                }
            }
            // Shortcut: If validation is skipped, we then don't need the last sample.
            // Unfortunetly, the sample input does not have the home row/col free, so we have to go a few more steps in that case.
            #[cfg(not(debug_assertions))]
            if garden.home_row_free || samples.len() >= 6 {
                if let [.., x1, x2, x3] = samples[..] {
                    let delta = ((target_dist - step) / size) as i64;
                    let a = x1 - 2 * x2 + x3;
                    let b = x1 - 4 * x2 + 3 * x3;
                    let c = 2 * x3;
                    return ((a * delta + b) * delta + c) / 2;
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
    current_fringe: Vec<Pos>,
    next_fringe: Vec<Pos>,
}

impl<'a> Walker<'a> {
    pub fn new(grid: &'a Grid<Tile>, start_pos: Pos, large: bool) -> Self {
        let (hs_capacity, vec_capacity) = if large {
            (7 << 15, 2_000)
        } else {
            (7 << 10, 300)
        };
        let mut current = HashSet::with_capacity(hs_capacity);
        let mut current_fringe = Vec::with_capacity(vec_capacity);
        let next = HashSet::with_capacity(hs_capacity);
        let next_fringe = Vec::with_capacity(vec_capacity);
        current_fringe.push(start_pos);
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
                    self.next_fringe.push(next_pos);
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

#[derive(Clone, Copy, Default)]
enum Tile {
    #[default]
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
pub struct Garden {
    grid: Grid<Tile>,
    start_pos: Pos,
    #[cfg_attr(debug_assertions, allow(unused))]
    home_row_free: bool,
}

impl Garden {
    fn new(grid: Grid<Tile>, start_pos: Pos, home_row_free: bool) -> Self {
        Self {
            grid,
            start_pos,
            home_row_free,
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("{0:?}")]
    CommonError(#[from] CommonParseError),
}

impl FromStr for Garden {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid<Tile> = text.parse()?;
        let start_pos = grid.position(|p| matches!(p, Tile::Start)).unwrap();
        grid.set(start_pos, Tile::GardenPlot);
        let mut home_row_free = grid
            .get_row(start_pos.row())
            .unwrap()
            .iter()
            .all(|tile| matches!(tile, Tile::GardenPlot));
        if home_row_free {
            for r in 0..grid.height() {
                #[allow(clippy::cast_possible_wrap)]
                if let Some(Tile::Rock) = grid.get(Pos::new(r as isize, start_pos.col())) {
                    home_row_free = false;
                    break;
                }
            }
        }
        Ok(Self::new(grid, start_pos, home_row_free))
    }
}
