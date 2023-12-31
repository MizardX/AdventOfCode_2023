use smallvec::SmallVec;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonParseError, Dir, Grid, Pos};

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 17");

    println!("++Example 1");
    let example = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 102)", part_1(&example));
    println!("|'-Part 2: {} (expected 94)", part_2(&example));

    println!("++Example 2");
    let example = EXAMPLE2.parse().expect("Parse example 2");
    println!("|'-Part 2: {} (expected 71)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 1099)", part_1(&input));
    println!("|'-Part 2: {} (expected 1266)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn part_1(input: &Input) -> usize {
    let start = State::new(Pos::new(0, 0), Dir::E, true);
    let goal_pos = Pos::new(
        input.grid.height() as isize - 1,
        input.grid.width() as isize - 1,
    );
    let (_path, dist) = pathfinding::prelude::astar(
        &start,
        |n| n.neighbors(0, 3, input),
        |n| n.pos.manhattan_distance(goal_pos),
        |n| n.pos.eq(&goal_pos),
    )
    .unwrap_or((vec![], 0));

    dist
}

#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn part_2(input: &Input) -> usize {
    let start = State::new(Pos::new(0, 0), Dir::E, true);
    let goal_pos = Pos::new(
        input.grid.height() as isize - 1,
        input.grid.width() as isize - 1,
    );
    let (_path, dist) = pathfinding::prelude::astar(
        &start,
        |n| n.neighbors(4, 10, input),
        |n| n.pos.manhattan_distance(goal_pos),
        |n| n.pos.eq(&goal_pos),
    )
    .unwrap_or((vec![], 0));

    dist
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    dir: Dir,
    is_start: bool,
}

impl State {
    pub fn new(pos: Pos, dir: Dir, is_start: bool) -> Self {
        Self { pos, dir, is_start }
    }

    fn neighbors(
        &self,
        min_dist_for_turn: u8,
        max_dist: u8,
        input: &Input,
    ) -> SmallVec<[(Self, usize); 14]> {
        let mut res = SmallVec::new();
        if self.is_start {
            for dir in [Dir::E, Dir::S] {
                let new_state = Self::new(self.pos, dir, false);
                res.push((new_state, 0));
            }
        } else {
            let mut pos = self.pos;
            let mut cost = 0;
            let turn_right = self.dir.turn_cw();
            let turn_left = self.dir.turn_ccw();
            for dist in 1..=max_dist {
                pos = pos + self.dir;
                if let Some(cell) = input.grid.get(pos) {
                    cost += cell.0 as usize;
                } else {
                    break;
                }
                if dist >= min_dist_for_turn {
                    for dir in [turn_left, turn_right] {
                        let cost2 = cost;
                        let new_state = Self::new(pos, dir, false);
                        res.push((new_state, cost2));
                    }
                }
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
            .finish()
    }
}

#[derive(Debug, Clone, Copy, Default)]
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
pub struct Input {
    grid: Grid<Cell>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("{0:?}")]
    CommonError(#[from] CommonParseError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid = text.parse()?;
        Ok(Self { grid })
    }
}
