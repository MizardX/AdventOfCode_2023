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

#[allow(unused)]
fn part_1(input: &Input) -> usize {
    let mut sum = 0;
    for item in &input.items {
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
fn part_2(input: &Input) -> usize {
    let mut sum = 0;
    for item in &input.items {
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

struct Item {
    row_masks: SmallVec<[u32; 20]>,
    col_masks: SmallVec<[u32; 20]>,
}

struct Input {
    items: Vec<Item>,
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
        // I'm parsing these together, to not require allocating a vec for the lines,
        // or making a more complicated double line splitter.
        let mut items: Vec<Item> = Vec::new();
        let mut row = 0;
        let mut width = 0;
        let mut row_masks = SmallVec::new();
        let mut col_masks = SmallVec::new();
        for line in text.lines() {
            if line.is_empty() {
                // Empty line between test cases.
                // Store the current test case, and reset for the next.
                items.push(Item {
                    row_masks: std::mem::take(&mut row_masks),
                    col_masks: std::mem::take(&mut col_masks),
                });
                row = 0;
            } else {
                // Line inside a test-case
                if row == 0 {
                    // First line of a test case
                    width = line.len();
                    col_masks.extend(std::iter::repeat(0).take(width));
                } else if line.len() != width {
                    return Err(ParseInputError::UnevenRows(width, line.len()));
                }
                let mut row_mask = 0_u32;
                for (col, ch) in line.bytes().enumerate() {
                    match ch {
                        b'.' => (),
                        b'#' => {
                            row_mask |= 1_u32 << col;
                            col_masks[col] |= 1_u32 << row;
                        }
                        ch => return Err(ParseInputError::InvalidChar(ch as char)),
                    }
                }
                row_masks.push(row_mask);
                row += 1;
            }
        }
        if row > 0 {
            // Last uncompleted test case
            items.push(Item {
                row_masks,
                col_masks,
            });
        }
        Ok(Self { items })
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
