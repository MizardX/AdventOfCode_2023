#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonErrors, Dir, Grid, Pos};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 16");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 46)", part_1(&example));
    println!("|'-Part 2: {} (expected 51)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6605)", part_1(&input));
    println!("|'-Part 2: {} (expected 6766)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let width = input.grid.width();
    let height = input.grid.height();
    let mut visited = Grid::from_vec(width, height, vec![DirMap::default(); width * height]);
    shoot_laser(input, State::new(Pos::new(0, 0), Dir::E), &mut visited)
}

fn shoot_laser(input: &Input, start: State, visited: &mut Grid<DirMap<bool>>) -> usize {
    visited.reset(DirMap::default());
    let mut pending = Vec::new();
    pending.push(start);
    while let Some(mut current) = pending.pop() {
        'inner: loop {
            match visited.get_mut(current.pos) {
                None => {
                    // exists map
                    break 'inner;
                }
                Some(v) => {
                    if v[current.dir] {
                        // already visited
                        break 'inner;
                    }
                    v[current.dir] = true;
                }
            }
            current = match (input.grid.get(current.pos).unwrap(), current.dir) {
                (Tile::MirrorTlbr, Dir::N) | (Tile::MirrorTrbl, Dir::S) => {
                    current.turn(Dir::W).step()
                }
                (Tile::MirrorTlbr, Dir::E) | (Tile::MirrorTrbl, Dir::W) => {
                    current.turn(Dir::S).step()
                }
                (Tile::MirrorTlbr, Dir::S) | (Tile::MirrorTrbl, Dir::N) => {
                    current.turn(Dir::E).step()
                }
                (Tile::MirrorTlbr, Dir::W) | (Tile::MirrorTrbl, Dir::E) => {
                    current.turn(Dir::N).step()
                }

                (Tile::SplitterLr, Dir::N | Dir::S) => {
                    pending.push(current.turn(Dir::E).step());
                    current.turn(Dir::W).step()
                }
                (Tile::SplitterTb, Dir::W | Dir::E) => {
                    pending.push(current.turn(Dir::S).step());
                    current.turn(Dir::N).step()
                }

                _ => current.step(),
            };
        }
    }
    visited.count_if(|v| v[Dir::N] || v[Dir::E] || v[Dir::S] || v[Dir::W])
}

fn part_2(input: &Input) -> usize {
    let width = input.grid.width();
    let height = input.grid.height();
    let mut visited = Grid::from_vec(width, height, vec![DirMap::new(false); width * height]);
    let width = isize::try_from(width).unwrap();
    let height = isize::try_from(height).unwrap();
    let mut max = 0;
    for r in 0..height {
        max = max.max(shoot_laser(
            input,
            State::new(Pos::new(r, 0), Dir::E),
            &mut visited,
        ));
        max = max.max(shoot_laser(
            input,
            State::new(Pos::new(r, width - 1), Dir::W),
            &mut visited,
        ));
    }
    for c in 0..width {
        max = max.max(shoot_laser(
            input,
            State::new(Pos::new(0, c), Dir::S),
            &mut visited,
        ));
        max = max.max(shoot_laser(
            input,
            State::new(Pos::new(height - 1, c), Dir::N),
            &mut visited,
        ));
    }
    max
}

#[derive(Clone, Copy, Default)]
struct DirMap<T>([T; 4]);

impl<T> DirMap<T>
where
    T: Copy,
{
    pub const fn new(initial_value: T) -> Self {
        Self([initial_value; 4])
    }
}

impl Debug for DirMap<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in [Dir::N, Dir::E, Dir::S, Dir::W] {
            if self[d] {
                write!(f, "{d:?}")?;
            } else {
                write!(f, "_")?;
            }
        }
        Ok(())
    }
}

impl<T> Index<Dir> for DirMap<T> {
    type Output = T;

    fn index(&self, index: Dir) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Dir> for DirMap<T> {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<Tile>,
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.parse()?;
        Ok(Self { grid })
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    MirrorTrbl,
    MirrorTlbr,
    SplitterTb,
    SplitterLr,
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Tile::Empty,
            b'/' => Tile::MirrorTrbl,
            b'\\' => Tile::MirrorTlbr,
            b'|' => Tile::SplitterTb,
            b'-' => Tile::SplitterLr,
            b => return Err(ParseInputError::InvalidChar(b as char)),
        })
    }
}

#[derive(Clone, Copy)]
struct State {
    pos: Pos,
    dir: Dir,
}

impl State {
    fn new(pos: Pos, dir: Dir) -> Self {
        Self { pos, dir }
    }

    fn step(mut self) -> Self {
        self.pos = self.pos + self.dir;
        self
    }

    fn turn(mut self, dir: Dir) -> Self {
        self.dir = dir;
        self
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
