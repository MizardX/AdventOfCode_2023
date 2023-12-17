#![warn(clippy::pedantic)]

use smallvec::SmallVec;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonErrors, Dir, Grid, Pos};

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day XX");

    println!("++Example 1");
    let example = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 102)", part_1(&example));
    println!("|'-Part 2: {} (expected 94)", part_2(&example));

    println!("++Example 2");
    let example = EXAMPLE2.parse().expect("Parse example 2");
    println!("|'-Part 2: {} (expected 71)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let start = State::new(Pos::new(0, 0), Dir::E, 0, true);
    let goal_pos = Pos::new(
        isize::try_from(input.grid.height()).unwrap() - 1,
        isize::try_from(input.grid.width()).unwrap() - 1,
    );
    let (_, dist) = pathfinding::prelude::astar(
        &start,
        |n| {
            n.neighbors(0, 3)
                .into_iter()
                .filter_map(|n1| Some((n1, input.grid.get(n1.pos)?.0 as usize)))
                .collect::<SmallVec<[_; 3]>>()
        },
        |n| n.pos.distance(goal_pos),
        |n| n.pos.eq(&goal_pos),
    )
    .unwrap();

    dist
}

fn part_2(input: &Input) -> usize {
    let start = State::new(Pos::new(0, 0), Dir::E, 0, true);
    let goal_pos = Pos::new(
        isize::try_from(input.grid.height()).unwrap() - 1,
        isize::try_from(input.grid.width()).unwrap() - 1,
    );
    let (_, dist) = pathfinding::prelude::astar(
        &start,
        |n| {
            n.neighbors(4, 10)
                .into_iter()
                .filter_map(|n1| Some((n1, input.grid.get(n1.pos)?.0 as usize)))
                .collect::<SmallVec<[_; 3]>>()
        },
        |n| n.pos.distance(goal_pos),
        |n| n.pos.eq(&goal_pos) && n.straight_blocks >= 4,
    )
    .unwrap();

    dist
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    dir: Dir,
    straight_blocks: u8,
    is_start: bool,
}

impl State {
    pub fn new(pos: Pos, dir: Dir, straight_blocks: u8, is_start: bool) -> Self {
        Self {
            pos,
            dir,
            straight_blocks,
            is_start,
        }
    }

    fn neighbors(&self, min_dist_for_turn: u8, max_dist: u8) -> SmallVec<[Self; 3]> {
        let mut res = SmallVec::new();
        if self.is_start {
            res.push(Self::new(self.pos + Dir::E, Dir::E, 1, false));
            res.push(Self::new(self.pos + Dir::S, Dir::S, 1, false));
        } else {
            if self.straight_blocks < max_dist {
                res.push(Self::new(
                    self.pos + self.dir,
                    self.dir,
                    self.straight_blocks + 1,
                    false,
                ));
            }
            if self.straight_blocks >= min_dist_for_turn {
                let turn_right = self.dir.turn_cw();
                let turn_left = self.dir.turn_ccw();
                res.push(Self::new(self.pos + turn_right, turn_right, 1, false));
                res.push(Self::new(self.pos + turn_left, turn_left, 1, false));
            }
        }
        res
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.pos.row())
            .field(&self.pos.col())
            .field(&self.dir)
            .field(&self.straight_blocks)
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell(u8);

impl TryFrom<u8> for Cell {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        value
            .is_ascii_digit()
            .then_some(Self(value - b'0'))
            .ok_or(ParseInputError::InvalidChar(value as char))
    }
}

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<Cell>,
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

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid = text.parse()?;
        Ok(Self { grid })
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
