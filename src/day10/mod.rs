#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::num::TryFromIntError;
use std::str::FromStr;

use thiserror::Error;

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

#[allow(unused)]
fn part_1(input: &Input) -> usize {
    let size = input.pipes.len();
    let (a, b) = {
        let mut it = input.neighbors(input.start).into_iter().flatten();
        (it.next().unwrap().0, it.next().unwrap().0.reverse())
    };
    let mut dist = 0;
    let mut cur = (input.start, b);
    loop {
        let (ndir, next) = input
            .neighbors(cur.0)
            .into_iter()
            .flatten()
            .find(|(d, p)| d.reverse() != cur.1)
            .unwrap();
        dist += 1;
        cur = (next, ndir);
        if cur.0 == input.start {
            break;
        }
    }
    dist / 2
}

#[allow(unused)]
//#[allow(clippy::too_many_lines)]
fn part_2(input: &Input) -> usize {
    let size = input.pipes.len();
    let (a, b) = {
        let mut it = input.neighbors(input.start).into_iter().flatten();
        (it.next().unwrap().0, it.next().unwrap().0.reverse())
    };
    let mut path = Grid::<Path>::new(input.pipes.width, input.pipes.height);
    path.set_if(input.start, Path::from_dir(b, a), Path::is_not_pipe);
    let mut cur = (input.start, b);
    loop {
        let (ndir, next) = input
            .neighbors(cur.0)
            .into_iter()
            .flatten()
            .find(|(d, p)| d.reverse() != cur.1)
            .unwrap();
        path.set_if(cur.0, Path::from_dir(cur.1, ndir), Path::is_not_pipe);
        match (cur.1, ndir) {
            // Straight
            (Dir::N, Dir::N) => {
                path.set_if(cur.0.west(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.east(), Path::I, Path::is_not_pipe);
            }
            (Dir::E, Dir::E) => {
                path.set_if(cur.0.north(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.south(), Path::I, Path::is_not_pipe);
            }
            (Dir::S, Dir::S) => {
                path.set_if(cur.0.east(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.west(), Path::I, Path::is_not_pipe);
            }
            (Dir::W, Dir::W) => {
                path.set_if(cur.0.south(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.north(), Path::I, Path::is_not_pipe);
            }

            // turn right
            (Dir::N, Dir::E) => {
                path.set_if(cur.0.west(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.north(), Path::O, Path::is_not_pipe);
            }
            (Dir::E, Dir::S) => {
                path.set_if(cur.0.north(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.east(), Path::O, Path::is_not_pipe);
            }
            (Dir::S, Dir::W) => {
                path.set_if(cur.0.east(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.south(), Path::O, Path::is_not_pipe);
            }
            (Dir::W, Dir::N) => {
                path.set_if(cur.0.south(), Path::O, Path::is_not_pipe);
                path.set_if(cur.0.west(), Path::O, Path::is_not_pipe);
            }

            // turn left
            (Dir::N, Dir::W) => {
                path.set_if(cur.0.north(), Path::I, Path::is_not_pipe);
                path.set_if(cur.0.east(), Path::I, Path::is_not_pipe);
            }
            (Dir::E, Dir::N) => {
                path.set_if(cur.0.east(), Path::I, Path::is_not_pipe);
                path.set_if(cur.0.south(), Path::I, Path::is_not_pipe);
            }
            (Dir::S, Dir::E) => {
                path.set_if(cur.0.south(), Path::I, Path::is_not_pipe);
                path.set_if(cur.0.west(), Path::I, Path::is_not_pipe);
            }
            (Dir::W, Dir::S) => {
                path.set_if(cur.0.west(), Path::I, Path::is_not_pipe);
                path.set_if(cur.0.north(), Path::I, Path::is_not_pipe);
            }
            _ => (),
        }
        cur = (next, ndir);
        if cur.0 == input.start {
            break;
        }
    }
    for row in 0..path.height {
        for col in 0..path.width {
            let pos = Pos::from_usize(row, col).unwrap();
            let Some(val) = path.get(pos) else { continue };
            match val {
                Path::I => {
                    path.flood_fill(pos, Path::I, |x| matches!(x, Path::X));
                }
                Path::O => {
                    path.flood_fill(pos, Path::O, |x| matches!(x, Path::X));
                }
                _ => (),
            }
        }
    }
    // Inside/outside could be the other way around, since we are not checking path direction. Just pick the smaller one.
    let outside = path.count_if(|x| matches!(x, Path::O));
    let inside = path.count_if(|x| matches!(x, Path::I));
    outside.min(inside)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    X,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    S,
}

impl Pipe {
    pub const fn connected_north(self) -> bool {
        matches!(self, Pipe::NS | Pipe::NE | Pipe::NW | Pipe::S)
    }
    pub const fn connected_east(self) -> bool {
        matches!(self, Pipe::EW | Pipe::NE | Pipe::SE | Pipe::S)
    }
    pub const fn connected_south(self) -> bool {
        matches!(self, Pipe::NS | Pipe::SE | Pipe::SW | Pipe::S)
    }
    pub const fn connected_west(self) -> bool {
        matches!(self, Pipe::EW | Pipe::NW | Pipe::SW | Pipe::S)
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    pub const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn from_usize(row: usize, col: usize) -> Option<Self> {
        Some(Self::new(
            isize::try_from(row).ok()?,
            isize::try_from(col).ok()?,
        ))
    }

    pub const fn north(self) -> Self {
        Self {
            row: self.row - 1,
            ..self
        }
    }

    pub const fn south(self) -> Self {
        Self {
            row: self.row + 1,
            ..self
        }
    }

    pub const fn east(self) -> Self {
        Self {
            col: self.col + 1,
            ..self
        }
    }

    pub const fn west(self) -> Self {
        Self {
            col: self.col - 1,
            ..self
        }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.row).field(&self.col).finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn reverse(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Path {
    #[default]
    X,
    NW,
    N,
    NE,
    EN,
    E,
    ES,
    SE,
    S,
    SW,
    WS,
    W,
    WN,
    O,
    I,
}

impl Path {
    pub fn is_pipe(self) -> bool {
        matches!(
            self,
            Self::NW
                | Self::N
                | Self::NE
                | Self::EN
                | Self::E
                | Self::ES
                | Self::SE
                | Self::S
                | Self::SW
                | Self::WS
                | Self::W
                | Self::WN
        )
    }

    pub fn is_not_pipe(self) -> bool {
        !self.is_pipe()
    }

    pub fn from_dir(enter: Dir, exit: Dir) -> Self {
        match (enter, exit) {
            (Dir::N, Dir::N) => Self::N,
            (Dir::N, Dir::E) => Self::NE,
            (Dir::N, Dir::W) => Self::NW,
            (Dir::E, Dir::N) => Self::EN,
            (Dir::E, Dir::E) => Self::E,
            (Dir::E, Dir::S) => Self::ES,
            (Dir::S, Dir::E) => Self::SE,
            (Dir::S, Dir::S) => Self::S,
            (Dir::S, Dir::W) => Self::SW,
            (Dir::W, Dir::N) => Self::WN,
            (Dir::W, Dir::S) => Self::WS,
            (Dir::W, Dir::W) => Self::W,
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Clone)]
struct Grid<T> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        Self {
            width,
            height,
            values: vec![T::default(); width * height],
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn from_vec(width: usize, height: usize, values: Vec<T>) -> Self {
        assert_eq!(values.len(), width * height);
        Self {
            width,
            height,
            values,
        }
    }

    pub fn get(&self, pos: Pos) -> Option<T> {
        Some(self.values[self.to_index(pos)?])
    }

    pub fn set(&mut self, pos: Pos, value: T) {
        if let Some(ix) = self.to_index(pos) {
            self.values[ix] = value;
        }
    }

    pub fn set_if(&mut self, pos: Pos, value: T, check: impl FnOnce(T) -> bool) {
        if let Some(ix) = self.to_index(pos) {
            if check(self.values[ix]) {
                self.values[ix] = value;
            }
        }
    }

    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    fn is_inside(&self, pos: Pos) -> bool {
        (0..self.height as isize).contains(&pos.row) && (0..self.width as isize).contains(&pos.col)
    }

    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    fn to_index(&self, pos: Pos) -> Option<usize> {
        if self.is_inside(pos) {
            usize::try_from(pos.row * self.width as isize + pos.col).ok()
        } else {
            None
        }
    }

    fn flood_fill(&mut self, start: Pos, replace_with: T, replace_if: impl Fn(T) -> bool) {
        assert!(!replace_if(replace_with));
        let mut current = Vec::new();
        current.push(start);
        let mut next = Vec::new();
        let mut first = true;
        while !current.is_empty() {
            for &pos in &current {
                let Some(val) = self.get(pos) else { continue };
                if !first && !replace_if(val) {
                    continue;
                }
                self.set(pos, replace_with);
                next.extend([pos.north(), pos.east(), pos.south(), pos.east()]);
            }
            first = false;
            std::mem::swap(&mut current, &mut next);
            next.clear();
        }
    }

    pub fn count_if(&self, check: impl Fn(&T) -> bool) -> usize {
        self.values.iter().copied().filter(check).count()
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stride = self.width;
        writeln!(f, "[")?;
        for r in 0..self.height {
            writeln!(f, "  {:?},", &self.values[r * stride..(r + 1) * stride])?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone)]
struct Input {
    pipes: Grid<Pipe>,
    start: Pos,
}

impl Input {
    fn get(&self, pos: Pos) -> Option<Pipe> {
        self.pipes.get(pos)
    }

    fn walk_north(&self, pos: Pos) -> Option<Pos> {
        let pipe_cur = self.get(pos)?;
        let target = pos.north();
        let pipe_tar = self.get(target)?;
        if pipe_cur.connected_north() && pipe_tar.connected_south() {
            Some(target)
        } else {
            None
        }
    }

    fn walk_east(&self, pos: Pos) -> Option<Pos> {
        let pipe_cur = self.get(pos)?;
        let target = pos.east();
        let pipe_tar = self.get(target)?;
        if pipe_cur.connected_east() && pipe_tar.connected_west() {
            Some(target)
        } else {
            None
        }
    }

    fn walk_south(&self, pos: Pos) -> Option<Pos> {
        let pipe_cur = self.get(pos)?;
        let target = pos.south();
        let pipe_tar = self.get(target)?;
        if pipe_cur.connected_south() && pipe_tar.connected_north() {
            Some(target)
        } else {
            None
        }
    }

    fn walk_west(&self, pos: Pos) -> Option<Pos> {
        let pipe_cur = self.get(pos)?;
        let target = pos.west();
        let pipe_tar = self.get(target)?;
        if pipe_cur.connected_west() && pipe_tar.connected_east() {
            Some(target)
        } else {
            None
        }
    }

    fn neighbors(&self, pos: Pos) -> [Option<(Dir, Pos)>; 4] {
        [
            self.walk_north(pos).map(|p| (Dir::N, p)),
            self.walk_east(pos).map(|p| (Dir::E, p)),
            self.walk_south(pos).map(|p| (Dir::S, p)),
            self.walk_west(pos).map(|p| (Dir::W, p)),
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

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().enumerate();
        let first = lines.next().ok_or(ParseError::EmptyInput)?.1;
        let width = first.len();
        let mut height = 1;
        let mut pipes = Vec::with_capacity(width * width);
        let mut start = Pos::new(-1, -1);
        for (c, ch) in first.bytes().enumerate() {
            let pipe = ch.try_into()?;
            pipes.push(pipe);
            if pipe == Pipe::S {
                start = Pos::new(0, isize::try_from(c)?);
            }
        }
        for (r, line) in lines {
            height = r + 1;
            for (c, ch) in line.bytes().enumerate() {
                let pipe = ch.try_into()?;
                pipes.push(pipe);
                if pipe == Pipe::S {
                    start = Pos::new(isize::try_from(r)?, isize::try_from(c)?);
                }
            }
        }
        if start == Pos::new(-1, -1) {
            return Err(ParseError::MissingStart);
        }
        Ok(Self {
            pipes: Grid::from_vec(width, height, pipes),
            start,
        })
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
