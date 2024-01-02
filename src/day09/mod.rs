use bstr::ByteSlice;
use bstr_parse::{BStrParse, ParseIntError};
use num_traits::Num;
use smallvec::SmallVec;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 09");

    println!("++Example");
    let example = parse_input(EXAMPLE).expect("Parse example");
    println!("|+-Part 1: {} (expected 114)", part_1(&example));
    println!("|'-Part 2: {} (expected 2)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT).expect("Parse real input");
    println!("|+-Part 1: {} (expected 1939607039)", part_1(&input));
    println!("|'-Part 2: {} (expected 1041)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Vec<Input> {
    parse_input(INPUT).expect("Parse real input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_input(INPUT).expect("Parse real input");
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &[Input]) -> i64 {
    let mut deltas: SmallVec<[_; 41]> = SmallVec::new();
    let mut sum = 0;
    for item in input {
        sum += predict_one(item.values.iter().copied(), &mut deltas);
    }
    sum
}

#[must_use]
pub fn part_2(input: &[Input]) -> i64 {
    let mut deltas: SmallVec<[_; 41]> = SmallVec::new();
    let mut sum = 0;
    for item in input {
        sum += predict_one(item.values.iter().rev().copied(), &mut deltas);
    }
    sum
}

fn predict_one<I, T, const N: usize>(source: I, buf: &mut SmallVec<[T; N]>) -> T
where
    I: Iterator<Item = T>,
    T: Num + Copy + std::iter::Sum,
{
    buf.clear();
    for mut x in source {
        for y in &mut *buf {
            let prev_y = *y;
            *y = x;
            x = x - prev_y;
        }
        buf.push(x);
    }
    buf.iter().copied().sum::<I::Item>()
}

#[derive(Debug, Clone)]
pub struct Input {
    values: Vec<i64>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Not an integer: {0}")]
    NotAnInteger(#[from] ParseIntError),
}

impl<'a> TryFrom<&'a [u8]> for Input {
    type Error = ParseInputError;

    fn try_from(line: &'a [u8]) -> Result<Self, Self::Error> {
        let mut count = 1;
        let mut start = 0;
        while let Some(ix) = line[start..].find_byte(b' ') {
            count += 1;
            start += ix + 1;
        }
        start = 0;
        let mut values = Vec::with_capacity(count);
        while let Some(ix) = line[start..].find_byte(b' ') {
            values.push(line[start..start + ix].parse()?);
            start += ix + 1;
        }
        values.push(line[start..].parse()?);
        Ok(Self { values })
    }
}

fn parse_input(text: &str) -> Result<Vec<Input>, ParseInputError> {
    let line_count = text.as_bytes().lines().count();
    let mut res: Vec<Input> = Vec::with_capacity(line_count);
    for line in text.as_bytes().lines() {
        res.push(line.try_into()?);
    }
    Ok(res)
}
