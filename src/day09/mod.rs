#![warn(clippy::pedantic)]

use std::num::ParseIntError;
use std::str::FromStr;

use num_traits::Num;
use thiserror::Error;

pub fn run() {
    println!(".Day 09");

    println!("++Example");
    let example = parse_input(include_str!("example.txt")).expect("Parse example");
    println!("|+-Part 1: {} (expected 114)", part_1(&example));
    println!("|'-Part 2: {} (expected 2)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt")).expect("Parse real input");
    println!("|+-Part 1: {} (expected 1939607039)", part_1(&input));
    println!("|'-Part 2: {} (expected 1041)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Input]) -> i64 {
    let mut deltas = Vec::with_capacity(41);
    let mut sum = 0;
    for item in input {
        sum += predict_one(item.values.iter().copied(), &mut deltas);
    }
    sum
}

#[allow(unused)]
fn part_2(input: &[Input]) -> i64 {
    let mut deltas = Vec::with_capacity(41);
    let mut sum = 0;
    for item in input {
        sum += predict_one(item.values.iter().rev().copied(), &mut deltas);
    }
    sum
}

fn predict_one<I, T>(source: I, buf: &mut Vec<T>) -> T
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
struct Input {
    values: Vec<i64>,
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Not an integer: {0}")]
    NotAnInteger(#[from] ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
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
