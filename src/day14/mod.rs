#![warn(clippy::pedantic)]

use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 14");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 136)", part_1(&example));
    println!("|'-Part 2: {} (expected 64)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 113_486)", part_1(&input));
    println!("|'-Part 2: {} (expected 104_409)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let mut input = input.clone();
    input.tilt_north();
    input.north_load()
}

const HASH_MODULO: usize = 2801;
const HASH_FACTOR: usize = 19;

fn part_2(input: &Input) -> usize {
    let mut input = input.clone();
    let mut seen = [None; HASH_MODULO];
    let mut step = 0;
    let repeat = loop {
        let hash = input.tilt_cycle();
        step += 1;
        if let Some(repeat) = seen[hash] {
            break repeat;
        }
        seen[hash] = Some(step);
    };
    let cycle = step - repeat;
    let remaining_cycles = (1_000_000_000 - step) % cycle;
    for _ in 0..remaining_cycles {
        input.tilt_cycle();
        step += 1;
    }
    input.north_load()
}

#[derive(Debug, Clone)]
struct Input {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Input {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
        Self {
            width,
            height,
            tiles,
        }
    }

    #[inline]
    fn is_inside(&self, r: usize, c: usize) -> bool {
        (0..self.height).contains(&r) && (0..self.width).contains(&c)
    }

    #[inline]
    fn index(&self, r: usize, c: usize) -> usize {
        assert!(self.is_inside(r, c));
        self.height * r + c
    }

    #[inline]
    fn get(&self, r: usize, c: usize) -> Tile {
        self.tiles[self.index(r, c)]
    }

    #[inline]
    fn get_mut(&mut self, r: usize, c: usize) -> &mut Tile {
        let ix = self.index(r, c);
        &mut self.tiles[ix]
    }

    fn get_row(&self, r: usize) -> &[Tile] {
        let ix = self.index(r, 0);
        &self.tiles[ix..ix + self.width]
    }

    fn north_load(&self) -> usize {
        let mut sum = 0;
        for r in 0..self.height {
            for &tile in self.get_row(r) {
                if let Tile::Rock = tile {
                    sum += self.height - r;
                }
            }
        }
        sum
    }

    fn tilt_cycle(&mut self) -> usize {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east()
    }

    fn tilt_north(&mut self) {
        for c in 0..self.width {
            let mut row_pos = 0;
            for r in 0..self.height {
                match self.get(r, c) {
                    Tile::Empty => (),
                    Tile::Fixed => row_pos = r + 1,
                    Tile::Rock => {
                        *self.get_mut(r, c) = Tile::Empty;
                        *self.get_mut(row_pos, c) = Tile::Rock;
                        row_pos += 1;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for c in 0..self.width {
            let mut row_pos = self.height - 1;
            for r in (0..self.height).rev() {
                match self.get(r, c) {
                    Tile::Empty => (),
                    Tile::Fixed => row_pos = r.saturating_sub(1),
                    Tile::Rock => {
                        *self.get_mut(r, c) = Tile::Empty;
                        *self.get_mut(row_pos, c) = Tile::Rock;
                        row_pos = row_pos.saturating_sub(1);
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for r in 0..self.height {
            let mut col_pos = 0;
            for c in 0..self.width {
                match self.get(r, c) {
                    Tile::Empty => (),
                    Tile::Fixed => col_pos = c + 1,
                    Tile::Rock => {
                        *self.get_mut(r, c) = Tile::Empty;
                        *self.get_mut(r, col_pos) = Tile::Rock;
                        col_pos += 1;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) -> usize {
        let mut hash = 0;
        for r in 0..self.height {
            let mut col_pos = self.width - 1;
            for c in (0..self.width).rev() {
                match self.get(r, c) {
                    Tile::Empty => (),
                    Tile::Fixed => col_pos = c.saturating_sub(1),
                    Tile::Rock => {
                        *self.get_mut(r, c) = Tile::Empty;
                        *self.get_mut(r, col_pos) = Tile::Rock;
                        hash = (hash * HASH_FACTOR + col_pos) % HASH_MODULO;
                        col_pos = col_pos.saturating_sub(1);
                    }
                }
            }
        }
        hash
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for tile in self.get_row(r) {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Rows of input is uneven")]
    UnevenRows,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut height = 0;
        let mut width = 0;
        for line in text.lines() {
            height += 1;
            if height == 1 {
                width = line.len();
            } else if line.len() != width {
                return Err(ParseInputError::UnevenRows);
            }
        }
        if height == 0 || width == 0 {
            return Err(ParseInputError::EmptyInput);
        }
        let mut tiles = Vec::with_capacity(width * height);
        for line in text.lines() {
            for cell in line.bytes() {
                tiles.push(cell.try_into()?);
            }
        }
        Ok(Self::new(width, height, tiles))
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Fixed,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Fixed => '#',
                Tile::Rock => 'O',
            }
        )
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Tile::Empty,
            b'#' => Tile::Fixed,
            b'O' => Tile::Rock,
            b => return Err(ParseInputError::InvalidChar(b as char)),
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
