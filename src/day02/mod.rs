use bstr::ByteSlice;
use bstr_parse::BStrParse;
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

#[must_use]
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    input
        .games
        .iter()
        .filter(|g| g.rounds.iter().copied().all(Round::is_possible))
        .map(|g| g.id)
        .sum()
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    input
        .games
        .iter()
        .map(|g| {
            g.rounds
                .iter()
                .copied()
                .reduce(Round::max_components)
                .unwrap_or(Round::new())
                .power()
        })
        .sum()
}

#[derive(Debug, Clone)]
pub struct Input {
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
pub enum ParseInputError {
    #[error("Expected character: {0:?}")]
    Expected(char),
    #[error(transparent)]
    InvalidNumber(#[from] bstr_parse::ParseIntError),
}

impl TryFrom<&[u8]> for Round {
    type Error = ParseInputError;

    fn try_from(piece: &[u8]) -> Result<Self, Self::Error> {
        let mut res = Round::new();
        for cube in piece.split(|&ch| ch == b',') {
            let (num_str, color_str) = cube
                .trim_ascii_start()
                .split_once(|&ch| ch == b' ')
                .ok_or(ParseInputError::Expected(' '))?;
            let num = num_str.parse::<u8>()?;
            match color_str[0] {
                b'r' => res.red += num,
                b'g' => res.green += num,
                b'b' => res.blue += num,
                _ => (),
            };
        }
        Ok(res)
    }
}

impl TryFrom<&[u8]> for Game {
    type Error = ParseInputError;

    fn try_from(line: &[u8]) -> Result<Self, Self::Error> {
        let line = &line[5..]; //.strip_prefix("Game ").ok_or(ParseInputError::Expected(...))?;
        let (id_str, line) = line
            .split_once(|&ch| ch == b':')
            .ok_or(ParseInputError::Expected(':'))?;
        let id = id_str
            .parse::<usize>()?;
        let mut rounds = SmallVec::new();
        for round_str in line.split(|&ch| ch == b';') {
            let round = round_str.try_into()?;
            rounds.push(round);
        }
        Ok(Game { id, rounds })
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Input, Self::Err> {
        let mut games: Vec<Game> = Vec::new();
        for line in text.as_bytes().lines() {
            games.push(line.try_into()?);
        }
        Ok(Self { games })
    }
}
