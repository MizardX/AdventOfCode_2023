#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub fn run() {
    println!(".Day 05");

    println!("++Example");
    let example = parse_input(include_str!("example.txt")).expect("Example input parsed");
    println!("|+-Part 1: {} (expected 35)", part_1(&example));
    println!("|'-Part 2: {} (expected 46)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt")).expect("Real input parsed");
    println!("|+-Part 1: {} (expected 174137457)", part_1(&input));
    println!("|'-Part 2: {} (expected 1493866)", part_2(&input));

    println!("')");
}

#[allow(unused)]
fn part_1(input: &Input) -> isize {
    let mut min = isize::MAX;
    for &(mut seed) in &input.seeds {
        for mapping in &input.mappings {
            seed = mapping.iter().find_map(|r| r.apply(seed)).unwrap_or(seed);
        }
        min = min.min(seed);
    }
    min
}

#[allow(unused)]
fn part_2(input: &Input) -> isize {
    let seed_ranges = input
        .seeds
        .array_chunks::<2>()
        .map(|a| Range(a[0], a[0] + a[1]))
        .collect::<Vec<_>>();
    let mut location = 0;
    loop {
        let mut cur = location;
        let mut min_delta = Option::<isize>::None;
        for mapping in input.mappings.iter().rev() {
            if let Some((next, delta)) = mapping.iter().find_map(|r| r.reverse_apply(cur)) {
                cur = next;
                min_delta = Some(if let Some(min_delta) = min_delta {
                    min_delta.min(delta)
                } else {
                    delta
                });
            }
        }
        if seed_ranges.iter().any(|r| r.contains(cur)) {
            return location;
        }
        location += min_delta.unwrap_or(1);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    start: isize,
    end: isize,
    delta: isize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range(isize, isize);

impl Range {
    pub fn contains(&self, val: isize) -> bool {
        self.0 <= val && val < self.1
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{} ({})",
            numfmt(self.0),
            numfmt(self.1),
            numfmt(self.1 - self.0)
        )
    }
}

impl Mapping {
    pub fn new(start: isize, end: isize, delta: isize) -> Self {
        Self { start, end, delta }
    }

    fn apply(&self, val: isize) -> Option<isize> {
        (self.start <= val && val < self.end).then_some(val + self.delta)
    }

    fn reverse_apply(&self, val: isize) -> Option<(isize, isize)> {
        let candidate = val - self.delta;
        (self.start <= candidate && candidate < self.end)
            .then_some((candidate, self.end - candidate))
    }
}

impl Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}..{} -> {}..{} ({:+})>",
            numfmt(self.start),
            numfmt(self.end),
            numfmt(self.start + self.delta),
            numfmt(self.end + self.delta),
            self.delta
        )
    }
}

fn numfmt(x: isize) -> String {
    x.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(" ")
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("First line should start with 'Seeds:'")]
    SeedSuffix,
    #[error("The first number, source_start, is missing.")]
    MissingSource,
    #[error("The second number, destination_start, is missing.")]
    MissingDestination,
    #[error("The third number, length, is missing.")]
    MissingLen,
    #[error("One of the numbers could not be parsed as an integer: {0}")]
    NotInteger(#[from] ParseIntError),
    #[error("The line contains more values than expected")]
    ExtraneousValues,
}

impl FromStr for Mapping {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let destination_start: isize = it.next().ok_or(ParseError::MissingDestination)?.parse()?;
        let source_start: isize = it.next().ok_or(ParseError::MissingSource)?.parse()?;
        let len: isize = it.next().ok_or(ParseError::MissingLen)?.parse()?;
        if it.next().is_some() {
            return Err(ParseError::ExtraneousValues);
        }
        Ok(Self::new(
            source_start,
            source_start + len,
            destination_start - source_start,
        ))
    }
}

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<isize>,
    mappings: Vec<Vec<Mapping>>,
}

#[allow(unused)]
fn parse_input(text: &str) -> Result<Input, ParseError> {
    let mut lines = text.lines();
    let mut seeds: Vec<isize> = lines
        .next()
        .ok_or(ParseError::EmptyInput)?
        .strip_prefix("seeds: ")
        .ok_or(ParseError::SeedSuffix)?
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let mut mappings = Vec::new();
    let mut current = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.ends_with(':') {
            current.sort_unstable();
            mappings.push(std::mem::take(&mut current));
        } else {
            current.push(Mapping::from_str(line)?);
        }
    }
    mappings.push(current);

    Ok(Input { seeds, mappings })
}
