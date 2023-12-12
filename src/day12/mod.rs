#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::str::FromStr;

use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 12");

    println!("++Example");
    let example = parse_input(EXAMPLE).expect("Parse example");
    println!("|+-Part 1: {} (expected 21)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT).expect("Parse input");
    println!("|+-Part 1: {} (expected 7694)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

fn part_1(input: &[Input]) -> usize {
    let mut sum = 0;
    for spring in input {
        let mut count = 0;
        'mask: for mask in 0_u32..(1 << spring.len) {
            if (mask & spring.broken) == spring.broken && (!mask & spring.working) == spring.working
            {
                let mut mask2 = mask;
                for s in spring.counts.iter().copied().rev() {
                    // mask:             1111000011110000
                    // mask-1:           1111000011101111
                    // mask^(mask-1):    0000000000010000 -> bit
                    // mask+bit:         1111000100000000
                    // !(mask+bit):      0000111011111111
                    // mask&!(mask+bit): 0000000011110000 -> group
                    let bit = mask2 ^ mask2.wrapping_sub(1);
                    let group = mask2 & !(mask2 + bit);
                    if group.count_ones() as usize != s {
                        continue 'mask;
                    }
                    mask2 &= !group;
                }
                if mask2 != 0 {
                    continue 'mask;
                }
                //println!("  {mask:00$b}", spring.len);
                count += 1;
            }
        }
        //println!("{spring:?} -> {count}");
        sum += count;
    }
    sum
}

#[allow(unused)]
fn part_2(input: &[Input]) -> usize {
    0
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Invalid character: {0}")]
    InvalidInput(char),
}

#[derive(Clone)]
struct Input {
    len: usize,
    working: u32,
    broken: u32,
    unknown: u32,
    counts: Vec<usize>,
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            len,
            working,
            broken,
            unknown,
            counts,
        } = self;
        write!(
            f,
            "<{working:0len$b} {broken:0len$b} {unknown:0len$b} {counts:?}>"
        )
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.bytes();
        let mut len = 0;
        let mut working = 0;
        let mut broken = 0;
        let mut unknown = 0;
        let mut counts = Vec::with_capacity(6);
        for b in &mut bytes {
            let (w, b, u) = match b {
                b'.' => (1, 0, 0),
                b'#' => (0, 1, 0),
                b'?' => (0, 0, 1),
                b' ' => break,
                b => return Err(ParseInputError::InvalidInput(b as char)),
            };
            working = (working << 1) | w;
            broken = (broken << 1) | b;
            unknown = (unknown << 1) | u;
            len += 1;
        }
        let mut num = 0;
        for b in bytes {
            num = match b {
                b'0'..=b'9' => 10 * num + (b - b'0') as usize,
                b',' => {
                    counts.push(num);
                    0
                }
                b => return Err(ParseInputError::InvalidInput(b as char)),
            };
        }
        counts.push(num);
        Ok(Self {
            len,
            working,
            broken,
            unknown,
            counts,
        })
    }
}

fn parse_input(text: &str) -> Result<Vec<Input>, ParseInputError> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
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
