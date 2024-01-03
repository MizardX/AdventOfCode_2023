use bstr::ByteSlice;
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
pub fn part_1(input: &Input) -> usize {
    let mut input = input.clone();
    input.tilt_north();
    input.north_load()
}

const HASH_MODULO: usize = 1697;
const HASH_FACTOR: usize = 17;

#[must_use]
pub fn part_2(input: &Input) -> usize {
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
struct Row {
    fixed: u128,
    rocks: u128,
}

#[derive(Debug, Clone)]
pub struct Input {
    width: usize,
    height: usize,
    rows: Vec<Row>,
}

impl Input {
    fn north_load(&self) -> usize {
        let mut sum = 0;
        for (r, row) in self.rows.iter().enumerate() {
            let count = row.rocks.count_ones() as usize;
            sum += count * (self.height - r);
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
            let bit = 1_u128 << c;
            let mut w = 0;
            for r in 0..self.height {
                if (self.rows[r].fixed & bit) != 0 {
                    w = r + 1;
                }
                if (self.rows[r].rocks & bit) != 0 {
                    self.rows[r].rocks &= !bit;
                    self.rows[w].rocks |= bit;
                    w += 1;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for c in 0..self.width {
            let bit = 1_u128 << c;
            let mut w = self.height - 1;
            for r in (0..self.height).rev() {
                if (self.rows[r].fixed & bit) != 0 {
                    w = r.saturating_sub(1);
                }
                if (self.rows[r].rocks & bit) != 0 {
                    self.rows[r].rocks &= !bit;
                    self.rows[w].rocks |= bit;
                    w = w.saturating_sub(1);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in &mut self.rows {
            let rocks = &mut row.rocks;
            let mut fixed = row.fixed;
            let mut last_fixed_bit = 1_u128;
            while fixed != 0 {
                let fixed_bit = 1_u128 << fixed.trailing_zeros();
                let mask = fixed_bit - last_fixed_bit;
                let num_rocks = (*rocks & mask).count_ones();
                *rocks = *rocks & !mask | ((last_fixed_bit << num_rocks) - last_fixed_bit);
                fixed &= !fixed_bit;
                last_fixed_bit = fixed_bit << 1;
            }
            let fixed_bit = 1_u128 << self.width;
            let mask = fixed_bit - last_fixed_bit;
            let num_rocks = (*rocks & mask).count_ones();
            *rocks = *rocks & !mask | ((last_fixed_bit << num_rocks) - last_fixed_bit);
        }
    }

    fn tilt_east(&mut self) -> usize {
        let mut hash = 0;
        for row in &mut self.rows {
            let rocks = &mut row.rocks;
            let mut fixed = row.fixed;
            let mut last_fixed_bit = 1_u128;
            while fixed != 0 {
                let fixed_bit = 1_u128 << fixed.trailing_zeros();
                let mask = fixed_bit - last_fixed_bit;
                let num_rocks = (*rocks & mask).count_ones();
                *rocks = *rocks & !mask | (fixed_bit - (fixed_bit >> num_rocks));
                fixed &= !fixed_bit;
                last_fixed_bit = fixed_bit;
                hash = (hash * HASH_FACTOR + num_rocks as usize) % HASH_MODULO;
            }
            let fixed_bit = 1_u128 << self.width;
            let mask = fixed_bit - last_fixed_bit;
            let num_rocks = (*rocks & mask).count_ones();
            *rocks = *rocks & !mask | (fixed_bit - (fixed_bit >> num_rocks));
            hash = (hash * HASH_FACTOR + num_rocks as usize) % HASH_MODULO;
        }
        hash
    }

    fn parse_line(line: &[u8]) -> Result<Row, ParseInputError> {
        let mut row = Row { fixed: 0, rocks: 0 };
        for (i, ch) in line.bytes().enumerate() {
            match ch {
                b'#' => row.fixed |= 1_u128 << i,
                b'O' => row.rocks |= 1_u128 << i,
                b'.' => (),
                ch => return Err(ParseInputError::InvalidChar(ch as char)),
            };
        }
        Ok(row)
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for c in 0..self.width {
                let bit = 1_u128 << c;
                if (row.fixed & bit) != 0 {
                    write!(f, "#")?;
                } else if (row.rocks & bit) != 0 {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Rows of input is uneven")]
    UnevenRows,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.as_bytes().lines();
        let first_line = lines.next().ok_or(ParseInputError::EmptyInput)?;
        let width = first_line.len();
        let mut rows = Vec::with_capacity(width);
        for line in [first_line].into_iter().chain(lines) {
            #[cfg(debug_assertions)]
            if line.len() != width {
                return Err(ParseInputError::UnevenRows);
            }
            let row = Self::parse_line(line)?;
            rows.push(row);
        }
        let height = rows.len();
        Ok(Self {
            width,
            height,
            rows,
        })
    }
}
