use bstr::ByteSlice;
use bstr_parse::{BStrParse, ParseIntError};
use std::cmp::Ordering;
use std::fmt::Debug;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 05");

    println!("++Example");
    let example = parse_input(EXAMPLE).expect("Example input parsed");
    println!("|+-Part 1: {} (expected 35)", part_1(&example));
    println!("|'-Part 2: {} (expected 46)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT).expect("Real input parsed");
    println!("|+-Part 1: {} (expected 174137457)", part_1(&input));
    println!("|'-Part 2: {} (expected 1493866)", part_2(&input));

    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Input {
    parse_input(INPUT).expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> isize {
    let mut min = isize::MAX;
    for &(mut seed) in &input.seeds {
        for mapping in &input.mappings {
            seed += if let Ok(ix) = mapping.binary_search_by(|m| m.cmp_source(seed)) {
                mapping[ix].delta
            } else {
                0
            }
        }
        min = min.min(seed);
    }
    min
}

#[must_use]
pub fn part_2(input: &Input) -> isize {
    let mut location = 0;
    loop {
        let mut cur = location;
        let mut min_delta = isize::MAX;
        for mapping in input.mappings2.iter().rev() {
            match mapping.binary_search_by(|m| m.cmp_dest(cur)) {
                Ok(ix) => {
                    let m = &mapping[ix];
                    min_delta = min_delta.min(m.end + m.delta - cur);
                    cur -= m.delta;
                }
                Err(ix) if ix < mapping.len() => {
                    let m = &mapping[ix]; // next
                    min_delta = min_delta.min(m.start + m.delta - cur);
                }
                _ => (),
            }
        }
        if input.seed_ranges.binary_search_by(|r| r.cmp_value(cur)).is_ok() {
            return location;
        }
        location += min_delta;
    }
}

#[derive(Copy, Clone, Eq)]
struct Mapping {
    start: isize,
    end: isize,
    delta: isize,
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: isize,
    len: isize,
}

impl Range {
    fn new(start: isize, len: isize) -> Self {
        Self { start, len }
    }

    fn end(&self) -> isize {
        self.start + self.len
    }

    fn cmp_value(&self, val: isize) -> Ordering {
        match val.cmp(&self.start) {
            Ordering::Less => Ordering::Greater,
            _ => match val.cmp(&self.end()) {
                Ordering::Less => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{} ({})",
            numfmt(self.start),
            numfmt(self.end()),
            numfmt(self.len)
        )
    }
}

impl Mapping {
    pub fn new(start: isize, end: isize, delta: isize) -> Self {
        Self { start, end, delta }
    }

    fn cmp_source(&self, val: isize) -> Ordering {
        match val.cmp(&self.start) {
            Ordering::Less => Ordering::Greater,
            _ => match val.cmp(&self.end) {
                Ordering::Less => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }

    fn cmp_dest(&self, val: isize) -> Ordering {
        match val.cmp(&(self.start + self.delta)) {
            Ordering::Less => Ordering::Greater,
            _ => match val.cmp(&(self.end + self.delta)) {
                Ordering::Less => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
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
    let x_str = x.abs().to_string().into_bytes();
    let mut s = String::with_capacity((x_str.len() + 2) / 3 * 4 + 1);
    if x < 0 {
        s.push('-');
    }
    for chunk in x_str.rchunks(3).rev() {
        if s.len() > 1 {
            s.push(' ');
        }
        s.push_str(std::str::from_utf8(chunk).unwrap());
    }
    s
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Input is empty")]
    EmptyInput,
    #[cfg(debug_assertions)]
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
    #[cfg(debug_assertions)]
    ExtraneousValues,
}

impl<'a> TryFrom<&'a [u8]> for Mapping {
    type Error = ParseError;

    fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        let mut it = s.split(|&ch| ch == b' ');
        let destination_start: isize = it.next().ok_or(ParseError::MissingDestination)?.parse()?;
        let source_start: isize = it.next().ok_or(ParseError::MissingSource)?.parse()?;
        let len: isize = it.next().ok_or(ParseError::MissingLen)?.parse()?;
        #[cfg(debug_assertions)]
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
pub struct Input {
    seeds: Vec<isize>,
    seed_ranges: Vec<Range>,
    mappings: Vec<Vec<Mapping>>,
    mappings2: Vec<Vec<Mapping>>,
}

fn parse_input(text: &str) -> Result<Input, ParseError> {
    let mut lines = text.as_bytes().lines();
    let first_line = lines.next().ok_or(ParseError::EmptyInput)?;
    #[cfg(debug_assertions)]
    if &first_line[..7] != b"seeds: " {
        return Err(ParseError::SeedSuffix);
    }
    let mut seeds = Vec::with_capacity(20);
    for num_str in first_line[7..].split(|&ch| ch == b' ') {
        seeds.push(num_str.parse()?);
    }
    let mut seed_ranges = Vec::with_capacity(seeds.len()/2);
    for &[a,b] in seeds.array_chunks() {
        seed_ranges.push(Range::new(a,b));
    }
    seed_ranges.sort_unstable();

    let mut mappings = Vec::with_capacity(10);
    let mut current: Vec<Mapping> = Vec::with_capacity(50);
    let mut is_header = true;

    for line in lines {
        if line.is_empty() {
            is_header = true;
            continue;
        }

        if is_header {
            is_header = false;
            if !current.is_empty() {
                current.sort_unstable();
                let mut tmp = Vec::with_capacity(50);
                std::mem::swap(&mut current, &mut tmp);
                mappings.push(tmp);
            }
        } else {
            current.push(line.try_into()?);
        }
    }
    current.sort_unstable();
    mappings.push(current);

    let mut mappings2 = mappings.clone();
    for m in &mut mappings2 {
        m.sort_unstable_by_key(|r| r.start + r.delta);
    }

    Ok(Input {
        seeds,
        seed_ranges,
        mappings,
        mappings2,
    })
}
