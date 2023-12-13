#![warn(clippy::pedantic)]

use std::str::FromStr;

use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 13");

    println!("++Example");
    let example = parse_input(EXAMPLE).expect("Parse example");
    println!("|+-Part 1: {} (expected 405)", part_1(&example));
    println!("|'-Part 2: {} (expected 400)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT).expect("Parse input");
    println!("|+-Part 1: {} (expected 35232)", part_1(&input));
    println!("|'-Part 2: {} (expected 37982)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Input]) -> usize {
    let mut sum = 0;
    for item in input {
        if let Some(r) = find_mirror_with_smudges::<0>(&item.row_masks) {
            sum += 100 * r;
        }
        if let Some(c) = find_mirror_with_smudges::<0>(&item.col_masks) {
            sum += c;
        }
    }
    sum
}

#[allow(unused)]
fn part_2(input: &[Input]) -> usize {
    let mut sum = 0;
    for item in input {
        if let Some(r) = find_mirror_with_smudges::<1>(&item.row_masks) {
            sum += 100 * r;
        }
        if let Some(c) = find_mirror_with_smudges::<1>(&item.col_masks) {
            sum += c;
        }
    }
    sum
}

fn find_mirror_with_smudges<const N: u32>(masks: &[u32]) -> Option<usize> {
    let len = masks.len();
    'outer: for (i, &[val1, val2]) in masks.array_windows::<2>().enumerate() {
        let mut diffs = (val1 ^ val2).count_ones();
        if diffs <= N {
            // Potential horizontal mirror
            for j1 in (2 * i + 2).saturating_sub(len)..i {
                let j2 = 2 * i - j1 + 1;
                diffs += (masks[j1] ^ masks[j2]).count_ones();
                if diffs > N {
                    continue 'outer;
                }
            }
            if diffs == N {
                return Some(i + 1);
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
struct Input {
    row_masks: Vec<u32>,
    col_masks: Vec<u32>,
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("Uneven row lengths; Expected {0} got {1}")]
    UnevenRows(usize, usize),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().enumerate();
        let (_, first) = lines.next().ok_or(ParseInputError::EmptyInput)?;
        let cols = first.len();
        let mut row_masks = Vec::with_capacity(20);
        let mut col_masks = vec![0_u32; cols];
        for (r, row) in std::iter::once((0, first)).chain(lines) {
            let mut row_mask = 0_u32;
            if row.len() != cols {
                return Err(ParseInputError::UnevenRows(cols, row.len()));
            }
            for (c, ch) in row.bytes().enumerate() {
                match ch {
                    b'.' => (),
                    b'#' => {
                        row_mask |= 1_u32 << c;
                        col_masks[c] |= 1_u32 << r;
                    }
                    ch => return Err(ParseInputError::InvalidChar(ch as char)),
                }
            }
            row_masks.push(row_mask);
        }
        Ok(Self {
            row_masks,
            col_masks,
        })
    }
}

fn parse_input(text: &str) -> Result<Vec<Input>, ParseInputError> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.split("\r\n\r\n") {
        // better way?
        //println!("PARSE: {line:?}");
        res.push(line.parse()?);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(parse_input(INPUT).expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = parse_input(INPUT).expect("Parse input");
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = parse_input(INPUT).expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
