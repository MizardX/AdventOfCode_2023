#![warn(clippy::pedantic)]

use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day XX");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected XXX)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

fn part_1(_input: &Input) -> usize {
    0
}

fn part_2(_input: &Input) -> usize {
    0
}

#[derive(Debug, Clone)]
struct Input {}

#[derive(Debug, Error)]
enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(_text: &str) -> Result<Self, Self::Err> {
        todo!()
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