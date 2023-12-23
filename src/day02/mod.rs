#![warn(clippy::pedantic)]

use std::num::ParseIntError;
use std::str::FromStr;

use smallvec::SmallVec;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 02");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 8)", part_1(&example));
    println!("|'-Part 2: {} (expected 2286)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 2176)", part_1(&input));
    println!("|'-Part 2: {} (expected 63700)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &Input) -> usize {
    input
        .games
        .iter()
        .filter(|g| g.rounds.iter().copied().all(Round::is_possible))
        .map(|g| g.id)
        .sum()
}

#[allow(unused)]
fn part_2(input: &Input) -> usize {
    input
        .games
        .iter()
        .map(|g| {
            g.rounds
                .iter()
                .copied()
                .reduce(Round::max_components)
                .unwrap()
                .power()
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Input {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    rounds: SmallVec<[Round; 6]>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl Round {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn is_possible(self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    pub fn max_components(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn power(self) -> usize {
        (self.red as usize) * (self.green as usize) * (self.blue as usize)
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Expected character: {0:?}")]
    Expected(char),
    #[error("Expected number: {0:?}")]
    InvalidNumber(#[from] ParseIntError),
}

impl FromStr for Round {
    type Err = ParseInputError;

    fn from_str(piece: &str) -> Result<Self, Self::Err> {
        let mut res = Round::new();
        for cube in piece.split(", ") {
            let (num_str, color_str) =
                cube.split_once(' ').ok_or(ParseInputError::Expected(' '))?;
            let num = num_str.parse::<u8>()?;
            match color_str.as_bytes()[0] {
                b'r' => res.red += num,
                b'g' => res.green += num,
                b'b' => res.blue += num,
                _ => (),
            };
        }
        Ok(res)
    }
}

impl FromStr for Game {
    type Err = ParseInputError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = &line[5..]; //.strip_prefix("Game ").ok_or(ParseInputError::Expected(...))?;
        let (id_str, line) = line
            .split_once(": ")
            .ok_or(ParseInputError::Expected(':'))?;
        let id = id_str.parse::<usize>()?;
        let mut rounds = SmallVec::new();
        for round_str in line.split("; ") {
            let round = round_str.parse()?;
            rounds.push(round);
        }
        Ok(Game { id, rounds })
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Input, Self::Err> {
        let mut games: Vec<Game> = Vec::new();
        for line in text.lines() {
            games.push(line.parse()?);
        }
        Ok(Self { games })
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(INPUT.parse::<Input>()));
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
