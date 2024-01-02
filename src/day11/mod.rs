use std::str::FromStr;

use bstr::ByteSlice;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 11");

    println!("++Example");
    let example = parse_input(EXAMPLE);
    println!("|+-Part 1: {} (expected 374)", part_1(&example));
    println!(
        "|'-Part 2; 10: {} (expected 1_030)",
        distance_between_galaxies(&example, 10)
    );
    println!(
        "|'-Part 2; 100: {} (expected 8_410)",
        distance_between_galaxies(&example, 100)
    );

    println!("++Input");
    let input = parse_input(INPUT);
    println!("|+-Part 1: {} (expected 9_724_940)", part_1(&input));
    println!("|'-Part 2: {} (expected 569_052_586_852)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Real input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> u64 {
    distance_between_galaxies(input, 2)
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    distance_between_galaxies(input, 1_000_000)
}

fn distance_between_galaxies(input: &Input, empty_scale: u64) -> u64 {
    assert!(empty_scale > 0);

    distance_1d(&input.galaxy_rows, empty_scale) + distance_1d(&input.galaxy_cols, empty_scale)
}

fn distance_1d(positions: &[usize], empty_scale: u64) -> u64 {
    let mut pos_prev = positions[0];
    let mut pos_short = 0;
    let mut pos_long = 0;

    let mut short_sum = 0;
    let mut long_sum = 0;

    for (i, &pos_cur) in positions.iter().enumerate() {
        let dela_pos = (pos_cur - pos_prev) as u64;
        pos_prev = pos_cur;
        pos_short += dela_pos * i as u64;
        pos_long += dela_pos.saturating_sub(1) * i as u64;

        short_sum += pos_short;
        long_sum += pos_long;
    }

    short_sum + long_sum * (empty_scale - 1)
}

#[derive(Debug, Clone)]
pub struct Input {
    galaxy_rows: Vec<usize>,
    galaxy_cols: Vec<usize>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Lines not same length")]
    InvaidInput,
    #[error("Found invalid character: {0:?}")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxy_rows = Vec::with_capacity(450);
        let mut galaxy_cols = Vec::with_capacity(450);
        let mut lines = s.as_bytes().lines().enumerate();
        let first_line = lines.next().ok_or(ParseInputError::EmptyInput)?;
        #[cfg(debug_assertions)]
        let width = first_line.1.len();

        for (r, srow) in [first_line].into_iter().chain(lines) {
            #[cfg(debug_assertions)]
            if srow.len() != width {
                return Err(ParseInputError::InvaidInput);
            }
            let mut start = 0;
            while let Some(c) = srow[start..].find_byte(b'#') {
                #[cfg(debug_assertions)]
                if !srow[start..start + c].bytes().all(|ch| ch == b'.') {
                    return Err(ParseInputError::InvaidInput);
                }
                galaxy_rows.push(r);
                galaxy_cols.push(start + c);
                start += c + 1;
            }
        }
        galaxy_cols.sort_unstable();
        Ok(Self {
            galaxy_rows,
            galaxy_cols,
        })
    }
}

fn parse_input(text: &str) -> Input {
    text.parse::<Input>().expect("Parse input")
}
