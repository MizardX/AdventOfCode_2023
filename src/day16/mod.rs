#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{Dir, Grid, Pos, CommonParseError};

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
    MirrorMaze::from(input).shoot_laser(Turtle::new(0, 0, Dir::E))
}

fn part_2(input: &Input) -> usize {
    let mut maze = MirrorMaze::from(input);
    let width = isize::try_from(input.grid.width()).unwrap();
    let height = isize::try_from(input.grid.height()).unwrap();
    let mut max = 0;
    for r in 0..height {
        max = max.max(maze.shoot_laser(Turtle::new(r, 0, Dir::E)));
        max = max.max(maze.shoot_laser(Turtle::new(r, width - 1, Dir::W)));
    }
    for c in 0..width {
        max = max.max(maze.shoot_laser(Turtle::new(0, c, Dir::S)));
        max = max.max(maze.shoot_laser(Turtle::new(height - 1, c, Dir::N)));
    }
    max
}

#[derive(Clone, Copy, Default)]
struct DirMap<T>([T; 4]);

impl DirMap<bool> {
    pub fn is_empty(self) -> bool {
        self.0 == [false, false, false, false]
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

#[derive(Debug, Clone)]
struct MirrorMaze<'a> {
    grid: &'a Grid<Tile>,
    visited: Grid<DirMap<bool>>,
}

impl<'a> MirrorMaze<'a> {
    fn from(input: &'a Input) -> Self {
        let width = input.grid.width();
        let height = input.grid.height();
        let visited = Grid::from_vec(width, height, vec![DirMap::default(); width * height]);
        Self {
            grid: &input.grid,
            visited,
        }
    }
}

impl<'a> MirrorMaze<'a> {
    fn shoot_laser(&mut self, start: Turtle) -> usize {
        self.visited.reset(DirMap::default());
        let mut pending = Vec::new();
        pending.push(start);
        'outer: while let Some(mut current) = pending.pop() {
            loop {
                match self.visited.get_mut(current.pos) {
                    None => {
                        // exists map
                        continue 'outer;
                    }
                    Some(v) => {
                        if v[current.dir] {
                            // already visited
                            continue 'outer;
                        }
                        v[current.dir] = true;
                    }
                }
                current = match (self.grid.get(current.pos).unwrap(), current.dir) {
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
        self.visited.count_if(|v| !v.is_empty())
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("{0:?}")]
    CommonError(#[from] CommonParseError)
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.parse()?;
        Ok(Self { grid })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Empty = b'.',
    MirrorTrbl = b'/',
    MirrorTlbr = b'\\',
    SplitterTb = b'|',
    SplitterLr = b'-',
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
struct Turtle {
    pos: Pos,
    dir: Dir,
}

impl Turtle {
    fn new(row: isize, col: isize, dir: Dir) -> Self {
        let pos = Pos::new(row, col);
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

impl Debug for Turtle {
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
