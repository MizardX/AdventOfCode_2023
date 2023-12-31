use std::str::FromStr;

use thiserror::Error;

use crate::aoclib::lcm;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const EXAMPLE3: &str = include_str!("example3.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 07");

    println!("++Example1");
    let example1 = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 2)", part_1(&example1));

    println!("++Example2");
    let example2 = EXAMPLE2.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 6)", part_1(&example2));

    println!("++Example3");
    let example3 = EXAMPLE3.parse().expect("Parse example 3");
    println!("|+-Part 2: {} (expected 6)", part_2(&example3));

    println!("++Input");
    let input = INPUT.parse().expect("Real input");
    println!("|+-Part 1: {} (expected 15517)", part_1(&input));
    println!("|'-Part 2: {} (expected 14935034899483)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Real input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut node = input.start_ix;
    for (i, mov) in input.instructions.iter().copied().cycle().enumerate() {
        if node == input.end_ix {
            return i;
        }
        node = match mov {
            Dir::Left => input.nodes[node].left_ix,
            Dir::Right => input.nodes[node].right_ix,
        };
    }
    unreachable!()
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    let mut res = 1;
    for &start_ix in &input.start_ixs {
        let dist = distance_to_end(input, start_ix);
        res = lcm(res, dist as u64);
    }
    res
}

fn distance_to_end(input: &Input, start_ix: usize) -> usize {
    let mut ix = start_ix;
    for (i, mov) in input.instructions.iter().copied().cycle().enumerate() {
        if input.nodes[ix].is_end {
            return i;
        }
        ix = match mov {
            Dir::Left => input.nodes[ix].left_ix,
            Dir::Right => input.nodes[ix].right_ix,
        };
    }
    unreachable!()
}

#[derive(Debug, Error, Clone)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Invalid instruction: '{0}'")]
    InvalidInstruction(char),
    #[error("Missing blank separator line")]
    MissingSeparatorLine,
    #[error("Node line does not match 'NAME = (NAME, NAME)'")]
    NodeSyntaxError,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl TryFrom<u8> for Dir {
    type Error = ParseInputError;

    fn try_from(ch: u8) -> Result<Self, Self::Error> {
        Ok(match ch {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => return Err(ParseInputError::InvalidInstruction(ch as char)),
        })
    }
}

#[derive(Debug, Clone, Default)]
struct Node {
    left_ix: usize,
    right_ix: usize,
    is_end: bool,
}

impl Node {
    pub fn new(left_ix: usize, right_ix: usize) -> Self {
        Self {
            left_ix,
            right_ix,
            is_end: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    instructions: Vec<Dir>,
    nodes: Vec<Node>,
    start_ix: usize,
    end_ix: usize,
    start_ixs: Vec<usize>,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn hash(s: &[u8]) -> usize {
            match s {
                &[b1, b2, b3] => {
                    (((b1 as usize) << 10) + ((b2 as usize) << 5) + (b3 as usize)) & 0x7FFF
                }
                _ => unreachable!(),
            }
        }

        let mut lines = s.as_bytes().split(|&ch| ch == b'\n');
        let instructions = lines
            .next()
            .ok_or(ParseInputError::EmptyInput)?
            .trim_ascii_end()
            .iter()
            .copied()
            .map(Dir::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        match lines.next() {
            Some([] | b"\r") => (),
            _ => return Err(ParseInputError::MissingSeparatorLine),
        };

        let mut nodes = vec![Node::default(); 0x8000];
        let mut start_ix = usize::MAX;
        let mut end_ix = usize::MAX;
        let mut start_ixs = Vec::new();
        for line in lines {
            let line = line.trim_ascii_end();
            // AAA = (BBB, CCC)
            if line.len() != 16 {
                return Err(ParseInputError::NodeSyntaxError);
            }
            let name = &line[0..3];
            #[cfg(debug_assertions)]
            if &line[3..7] != b" = (" {
                return Err(ParseInputError::NodeSyntaxError);
            }
            let left = &line[7..10];
            #[cfg(debug_assertions)]
            if &line[10..12] != b", " {
                return Err(ParseInputError::NodeSyntaxError);
            }
            let right = &line[12..15];
            #[cfg(debug_assertions)]
            if &line[15..] != b")" {
                return Err(ParseInputError::NodeSyntaxError);
            }

            let mut node = Node::new(hash(left), hash(right));
            let ix = hash(name);
            if name[2] == b'A' {
                if name == b"AAA" {
                    start_ix = ix;
                }
                start_ixs.push(ix);
            } else if name[2] == b'Z' {
                if name == b"ZZZ" {
                    end_ix = ix;
                }
                node.is_end = true;
            }
            nodes[ix] = node;
        }
        Ok(Self {
            instructions,
            nodes,
            start_ix,
            end_ix,
            start_ixs,
        })
    }
}
