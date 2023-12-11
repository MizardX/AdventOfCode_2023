#![warn(clippy::pedantic)]

use std::str::FromStr;

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

fn part_1(input: &Input) -> u64 {
    distance_between_galaxies(input, 2)
}

fn part_2(input: &Input) -> u64 {
    distance_between_galaxies(input, 1_000_000)
}

fn distance_between_galaxies(input: &Input, empty_scale: u64) -> u64 {
    assert!(empty_scale > 0);

    let mut row_prev = input.galaxy_rows[0];
    let mut row_short = 0;
    let mut row_long = 0;

    let mut col_prev = input.galaxy_cols[0];
    let mut col_short = 0;
    let mut col_long = 0;

    let mut short_sum = 0;
    let mut long_sum = 0;

    for i in 0..input.galaxy_cols.len() {
        let row_cur = input.galaxy_rows[i];
        let dela_row = (row_cur - row_prev) as u64;
        row_prev = row_cur;
        row_short += dela_row * i as u64;
        row_long += dela_row.saturating_sub(1) * i as u64;
        
        let col_cur = input.galaxy_cols[i];
        let dela_cols = (col_cur - col_prev) as u64;
        col_prev = col_cur;
        col_short += dela_cols * i as u64;
        col_long += dela_cols.saturating_sub(1) * i as u64;

        short_sum += row_short + col_short;
        long_sum += row_long + col_long;
    }

    short_sum + long_sum * (empty_scale - 1)
}

#[derive(Debug, Clone)]
struct Input {
    galaxy_rows: Vec<usize>,
    galaxy_cols: Vec<usize>,
}

#[derive(Debug, Error)]
enum ParseInputError {
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
        let mut galaxy_rows = Vec::new();
        let mut galaxy_cols = Vec::new();
        let mut lines = s.lines().enumerate();
        let (_, first) = lines.next().ok_or(ParseInputError::EmptyInput)?;
        let width = first.len();

        for (r, srow) in std::iter::once((0, first)).chain(lines) {
            if srow.len() != width {
                return Err(ParseInputError::InvaidInput);
            }
            for (c, ch) in srow.bytes().enumerate() {
                match ch {
                    b'#' => {
                        galaxy_rows.push(r);
                        galaxy_cols.push(c);
                    }
                    b'.' => (),
                    _ => return Err(ParseInputError::InvalidChar(ch as char)),
                }
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

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(parse_input(INPUT)));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| black_box(part_2(&input)));
    }
}
