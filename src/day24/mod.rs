#![warn(clippy::pedantic)]

use std::num::ParseFloatError;
use std::ops::RangeBounds;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonParseError, Coordinate};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 24");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 2)", part_1(&example, 7.0..=27.0));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!(
        "|+-Part 1: {} (expected XXX)",
        part_1(&input, 2.0e14..=4.0e14)
    );
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

fn part_1<R: RangeBounds<f64>>(input: &Input, range: R) -> usize {
    let mut count = 0;
    for (i, hail1) in input.hails.iter().enumerate() {
        for hail2 in &input.hails[i + 1..] {
            if let Some(intersection) = intersection_2d(hail1, hail2) {
                if range.contains(&intersection.x) && range.contains(&intersection.y) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[derive(Debug, Clone)]
struct Intersection2d {
    x: f64,
    y: f64,
}

fn intersection_2d(hail1: &Hail, hail2: &Hail) -> Option<Intersection2d> {
    let denomenator = hail1.velocity.x * hail2.velocity.y - hail2.velocity.x * hail1.velocity.y;
    if denomenator.abs() < 1e-3 {
        return None;
    }
    let numerator1 = (hail1.position.y - hail2.position.y) * hail2.velocity.x
        + (hail2.position.x - hail1.position.x) * hail2.velocity.y;
    let time1 = numerator1 / denomenator;
    if time1.is_sign_negative() {
        return None;
    }
    let numerator2 = (hail1.position.y - hail2.position.y) * hail1.velocity.x
        + (hail2.position.x - hail1.position.x) * hail1.velocity.y;
    let time2 = numerator2 / denomenator;
    if time2.is_sign_negative() {
        return None;
    }
    let x = hail1.position.x + time1 * hail1.velocity.x;
    let y = hail1.position.y + time1 * hail1.velocity.y;
    Some(Intersection2d { x, y })
}

fn part_2(_input: &Input) -> usize {
    0
}

#[derive(Debug, Clone)]
struct Hail {
    position: Coordinate<f64>,
    velocity: Coordinate<f64>,
}

impl FromStr for Hail {
    type Err = ParseInputError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (pos_str, vel_str) = line
            .split_once('@')
            .ok_or(ParseInputError::ExpectedChar('@'))?;
        Ok(Self {
            position: pos_str.parse()?,
            velocity: vel_str.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Input {
    hails: Vec<Hail>,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut hails = Vec::new();
        for line in text.lines() {
            hails.push(line.parse()?);
        }
        Ok(Self { hails })
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
    #[error("Did not find expected char: '{0}'")]
    ExpectedChar(char),
    #[error("{0}")]
    CommonError(#[from] CommonParseError),
    #[error("Invalid float: {0}")]
    InvalidFloat(#[from] ParseFloatError),
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
        b.iter(|| black_box(part_1(&input, 2.0e14..=4.0e14)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
