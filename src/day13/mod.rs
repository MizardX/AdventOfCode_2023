#![warn(clippy::pedantic)]

use std::str::FromStr;

use smallvec::SmallVec;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 13");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 405)", part_1(&example));
    println!("|'-Part 2: {} (expected 400)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 35232)", part_1(&input));
    println!("|'-Part 2: {} (expected 37982)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let mut sum = 0;
    for item in &input.patterns {
        if let Some(r) = find_mirror_with_smudges::<0>(&item.row_masks) {
            sum += 100 * r;
        }
        if let Some(c) = find_mirror_with_smudges::<0>(&item.col_masks) {
            sum += c;
        }
    }
    sum
}

fn part_2(input: &Input) -> usize {
    let mut sum = 0;
    for item in &input.patterns {
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

struct Pattern {
    row_masks: SmallVec<[u32; 20]>,
    col_masks: SmallVec<[u32; 20]>,
}

struct Input {
    patterns: Vec<Pattern>,
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("Uneven row lengths; Expected {0} got {1}")]
    UnevenRows(usize, usize),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut parser = PatternParser::new();
        for line in text.lines() {
            if !line.is_empty() {
                parser.parse_line(line)?;
            } else if !parser.is_empty() {
                // Empty line between patterns.
                patterns.push(parser.complete());
            }
        }

        if !parser.is_empty() {
            // Last pattern
            patterns.push(parser.complete());
        }

        Ok(Self { patterns })
    }
}

#[derive(Default)]
struct PatternParser {
    row_masks: SmallVec<[u32; 20]>,
    col_masks: SmallVec<[u32; 20]>,
    row: usize,
    width: usize,
}

impl PatternParser {
    pub fn new() -> Self {
        PatternParser::default()
    }

    pub fn parse_line(&mut self, line: &str) -> Result<(), ParseInputError> {
        if self.row == 0 {
            self.width = line.len();
            self.col_masks.extend(std::iter::repeat(0).take(self.width));
        } else if line.len() != self.width {
            return Err(ParseInputError::UnevenRows(self.width, line.len()));
        }

        let mut row_mask = 0_u32;
        for (col, ch) in line.bytes().enumerate() {
            match ch {
                b'.' => (),
                b'#' => {
                    row_mask |= 1_u32 << col;
                    self.col_masks[col] |= 1_u32 << self.row;
                }
                ch => return Err(ParseInputError::InvalidChar(ch as char)),
            }
        }
        self.row_masks.push(row_mask);
        self.row += 1;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.row == 0
    }

    pub fn complete(&mut self) -> Pattern {
        let row_masks = std::mem::take(&mut self.row_masks);
        let col_masks = std::mem::take(&mut self.row_masks);
        self.row = 0;
        Pattern {
            row_masks,
            col_masks,
        }
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
