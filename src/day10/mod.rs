#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::num::TryFromIntError;
use std::ops::Add;
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

fn part_1(input: &Input) -> usize {
    // Pick one direction for the start tile
    let enter = {
        let mut it = input.neighbors(input.start).into_iter().flatten();
        // pick first as exit direction; ignored
        let _ = it.next();
        // the other is enter direction
        it.next().unwrap().0.reverse()
    };

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

fn part_2(input: &Input) -> usize {
    // Pick one direction for the start tile
    let (exit, enter) = {
        let mut it = input.neighbors(input.start).into_iter().flatten();
        (it.next().unwrap().0, it.next().unwrap().0.reverse())
    };

    let mut pipes = Grid::<Path>::new(input.pipes.width, input.pipes.height);
    pipes.set_if(input.start, Path::from_dir(enter, exit), Path::is_not_pipe);

    let mut pos = input.start;
    let mut dir = enter;
    loop {
        // The direction we didn't come from
        let (next_dir, next) = input
            .neighbors(pos)
            .into_iter()
            .flatten()
            .find(|(next_dir, _)| next_dir.reverse() != dir)
            .unwrap();
        let pipe = Path::from_dir(dir, next_dir);
        pipes.set_if(pos, pipe, Path::is_not_pipe);

        if let Path::E | Path::ES | Path::NE = pipe {
            pipes.set_if(pos + Dir::N, Path::Outside, Path::is_not_pipe);
        }
        if let Path::S | Path::SW | Path::ES = pipe {
            pipes.set_if(pos + Dir::E, Path::Outside, Path::is_not_pipe);
        }
        if let Path::W | Path::WN | Path::SW = pipe {
            pipes.set_if(pos + Dir::S, Path::Outside, Path::is_not_pipe);
        }
        if let Path::N | Path::NE | Path::WN = pipe {
            pipes.set_if(pos + Dir::W, Path::Outside, Path::is_not_pipe);
        }

        if let Path::W | Path::WS | Path::NW = pipe {
            pipes.set_if(pos + Dir::N, Path::Inside, Path::is_not_pipe);
        }
        if let Path::N | Path::NW | Path::EN = pipe {
            pipes.set_if(pos + Dir::E, Path::Inside, Path::is_not_pipe);
        }
        if let Path::E | Path::EN | Path::SE = pipe {
            pipes.set_if(pos + Dir::S, Path::Inside, Path::is_not_pipe);
        }
        if let Path::S | Path::SE | Path::WS = pipe {
            pipes.set_if(pos + Dir::W, Path::Inside, Path::is_not_pipe);
        }

        pos = next;
        dir = next_dir;
        if pos == input.start {
            break;
        }
    }
    for row in 0..pipes.height {
        for col in 0..pipes.width {
            let pos = Pos::from_usize(row, col).unwrap();
            let Some(val) = pipes.get(pos) else { continue };
            match val {
                Path::Inside => {
                    pipes.flood_fill(pos, Path::Inside, |x| matches!(x, Path::X));
                }
                Path::Outside => {
                    pipes.flood_fill(pos, Path::Outside, |x| matches!(x, Path::X));
                }
                _ => (),
            }
        }
    }
    // Inside/outside could be the other way around, since we are not checking path direction. Just pick the smaller one.
    let outside = pipes.count_if(|x| matches!(x, Path::Outside));
    let inside = pipes.count_if(|x| matches!(x, Path::Inside));
    outside.min(inside)
}

/// Underectional pipes
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
            (dir, self),
            (Dir::N, Pipe::NS | Pipe::NE | Pipe::NW | Pipe::S)
                | (Dir::E, Pipe::EW | Pipe::NE | Pipe::SE | Pipe::S)
                | (Dir::S, Pipe::NS | Pipe::SE | Pipe::SW | Pipe::S)
                | (Dir::W, Pipe::EW | Pipe::NW | Pipe::SW | Pipe::S)
        )
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

/// Grid position
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    pub const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    /// Tries to convrt usize into signed
    pub fn from_usize(row: usize, col: usize) -> Option<Self> {
        Some(Self::new(
            isize::try_from(row).ok()?,
            isize::try_from(col).ok()?,
        ))
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.row).field(&self.col).finish()
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::N => self.row -= 1,
            Dir::E => self.col += 1,
            Dir::S => self.row += 1,
            Dir::W => self.col -= 1,
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    /// North
    N,
    /// East
    E,
    /// South
    S,
    /// West
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

/// Directional pipes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Path {
    /// Unpsecified
    #[default]
    X,
    /// North turning west
    NW,
    /// North ahead
    N,
    /// North turning east
    NE,
    /// East turning north
    EN,
    /// East ahead
    E,
    /// East turning south
    ES,
    /// South turning east
    SE,
    /// South ahead
    S,
    /// South turning west
    SW,
    /// West turning south
    WS,
    /// West ahead
    W,
    /// West turning north
    WN,
    /// Outside
    Outside,
    /// Inside
    Inside,
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
            _ => unreachable!(),
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
    fn is_inside(&self, pos: Pos) -> bool {
        let Ok(height) = isize::try_from(self.height) else {
            return false;
        };
        let Ok(width) = isize::try_from(self.width) else {
            return false;
        };
        (0..height).contains(&pos.row) && (0..width).contains(&pos.col)
    }

    #[inline]
    fn to_index(&self, pos: Pos) -> Option<usize> {
        let Ok(width) = isize::try_from(self.width) else {
            return None;
        };
        if self.is_inside(pos) {
            usize::try_from(pos.row * width + pos.col).ok()
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
                next.extend([pos + Dir::N, pos + Dir::E, pos + Dir::S, pos + Dir::E]);
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
