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
    println!("|'-Part 2: {} (expected 47)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!(
        "|+-Part 1: {} (expected 31_921)",
        part_1(&input, 2e14..=4e14)
    );
    println!(
        "|'-Part 2: {} (expected 761_691_907_059_631)",
        part_2(&input)
    );
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

#[allow(clippy::cast_precision_loss)]
fn intersection_2d(hail1: &Hail, hail2: &Hail) -> Option<Intersection2d> {
    let denomenator = hail1.velocity.x * hail2.velocity.y - hail2.velocity.x * hail1.velocity.y;
    if denomenator == 0 {
        return None;
    }
    let numerator1 = (hail1.position.y - hail2.position.y) * hail2.velocity.x
        + (hail2.position.x - hail1.position.x) * hail2.velocity.y;
    if numerator1.is_negative() ^ denomenator.is_negative() {
        return None;
    }
    let numerator2 = (hail1.position.y - hail2.position.y) * hail1.velocity.x
        + (hail2.position.x - hail1.position.x) * hail1.velocity.y;
    if numerator2.is_negative() ^ denomenator.is_negative() {
        return None;
    }
    let x = hail1.position.x as f64
        + ((numerator1 as f64 * hail1.velocity.x as f64) / denomenator as f64);
    let y = hail1.position.y as f64
        + ((numerator1 as f64 * hail1.velocity.y as f64) / denomenator as f64);
    Some(Intersection2d { x, y })
}

fn part_2(input: &Input) -> i128 {
    let p1 = &input.hails[0].position;
    let v1 = &input.hails[0].velocity;
    let p2 = &input.hails[1].position;
    let v2 = &input.hails[1].velocity;
    let p3 = &input.hails[2].position;
    let v3 = &input.hails[2].velocity;
    let num1 = (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)) * (v2.z - v3.z)
        + (p2.x * (p1.z - p3.z) + p1.x * (-p2.z + p3.z) + p3.x * (p2.z - p1.z)) * (v2.y - v3.y)
        + (p1.y * (p2.z - p3.z) + p2.y * (-p1.z + p3.z) + p3.y * (p1.z - p2.z)) * (v2.x - v3.x);
    let denom1 = (v1.y * (v2.z - v3.z) + v2.y * (-v1.z + v3.z) + v3.y * (v1.z - v2.z))
        * (p2.x - p3.x)
        + ((v2.x - v3.x) * v1.z + (-v1.x + v3.x) * v2.z + (v1.x - v2.x) * v3.z) * (p2.y - p3.y)
        + (v1.x * (v2.y - v3.y) + v2.x * (-v1.y + v3.y) + v3.x * (v1.y - v2.y)) * (p2.z - p3.z);
    let num2 = (p1.y * (p2.z - p3.z) + p2.y * (-p1.z + p3.z) + p3.y * (p1.z - p2.z))
        * (v1.x - v3.x)
        + ((p2.x - p3.x) * p1.z + (-p1.x + p3.x) * p2.z + (p1.x - p2.x) * p3.z) * (v1.y - v3.y)
        + (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)) * (v1.z - v3.z);
    let denom2 = (v1.y * (v2.z - v3.z) + v2.y * (-v1.z + v3.z) + v3.y * (v1.z - v2.z))
        * (p1.x - p3.x)
        + ((v2.x - v3.x) * v1.z + (-v1.x + v3.x) * v2.z + (v1.x - v2.x) * v3.z) * (p1.y - p3.y)
        + (v1.x * (v2.y - v3.y) + v2.x * (-v1.y + v3.y) + v3.x * (v1.y - v2.y)) * (p1.z - p3.z);
    let time1 = num1 / denom1;
    let time2 = num2 / denom2;
    let collission1 = *p1 + *v1 * time1;
    let collission2 = *p2 + *v2 * time2;
    let velocity = (collission2 - collission1) / (time2 - time1);
    let position = collission1 - velocity * time1;
    position.x + position.y + position.z
}

#[derive(Debug, Clone)]
struct Hail {
    position: Coordinate<i128>,
    velocity: Coordinate<i128>,
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
        b.iter(|| black_box(part_1(&input, 2e14..=4e14)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
