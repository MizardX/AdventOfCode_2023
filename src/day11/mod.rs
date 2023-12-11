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
    let mut sum_distances = 0;
    for (i, &(r1, c1)) in input.galaxies.iter().enumerate() {
        for &(r2, c2) in &input.galaxies[i + 1..] {
            let dr = r1.abs_diff(r2);
            let dc = c1.abs_diff(c2);
            let empty_rows = input.sum_empty_rows[r1].abs_diff(input.sum_empty_rows[r2]);
            let empty_cols = input.sum_empty_cols[c1].abs_diff(input.sum_empty_cols[c2]);
            sum_distances += (dr + dc) as u64 + (empty_rows + empty_cols) as u64 * (empty_scale - 1);
        }
    }
    sum_distances
}

#[derive(Debug, Clone)]
struct Input {
    galaxies: Vec<(usize, usize)>,
    sum_empty_rows: Vec<usize>,
    sum_empty_cols: Vec<usize>,
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
        let mut galaxies = Vec::new();
        let mut lines = s.lines().enumerate();
        let (_, first) = lines.next().ok_or(ParseInputError::EmptyInput)?;
        let width = first.len();

        let mut sum_empty_cols = vec![1_usize; width];
        let mut sum_empty_rows = Vec::new();

        for (r, srow) in std::iter::once((0, first)).chain(lines) {
            sum_empty_rows.push(1);
            if srow.len() != width {
                return Err(ParseInputError::InvaidInput);
            }
            for (c, ch) in srow.bytes().enumerate() {
                match ch {
                    b'#' => {
                        galaxies.push((r, c));
                        sum_empty_rows[r] = 0;
                        sum_empty_cols[c] = 0;
                    }
                    b'.' => (),
                    _ => return Err(ParseInputError::InvalidChar(ch as char)),
                }
            }
        }
        let mut running_sum = 0;
        for row_sum in &mut sum_empty_rows {
            running_sum += *row_sum;
            *row_sum = running_sum;
        }
        running_sum = 0;
        for col_sum in &mut sum_empty_cols {
            running_sum += *col_sum;
            *col_sum = running_sum;
        }
        Ok(Self {
            galaxies,
            sum_empty_rows,
            sum_empty_cols,
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
