#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::str::FromStr;

use num_traits::Num;

use thiserror::Error;

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

#[allow(unused)]
fn part_1(input: &Input) -> usize {
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

#[allow(unused)]
fn part_2(input: &Input) -> u64 {
    let mut res = 1;
    for &start_ix in &input.start_ixs {
        let dist = distance_to_end(input, start_ix);
        res = lcm(res, dist as u64);
    }
    res
}

fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + Num,
{
    let zero = T::zero();
    while a != zero {
        let r = b % a;
        b = a;
        a = r;
    }
    b
}

fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Num,
{
    a * b / gcd(a, b)
}

fn distance_to_end(input: &Input, start_ix: usize) -> usize {
    let mut ix = start_ix;
    for (i, mov) in input.instructions.iter().copied().cycle().enumerate() {
        let node = &input.nodes[ix];
        if node.is_end {
            return i;
        }
        ix = match mov {
            Dir::Left => node.left_ix,
            Dir::Right => node.right_ix,
        };
    }
    unreachable!()
}

#[derive(Debug, Error, Clone)]
enum ParseError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Invalid instruction: '{0}'")]
    InvalidInstruction(char),
    #[error("Missing blank separator line")]
    MissingSeparatorLine,
    #[error("Node line does not match 'NAME = (NAME, NAME)'")]
    NodeSyntaxError,
    #[error("Node not found")]
    NodeNotFound,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl TryFrom<u8> for Dir {
    type Error = ParseError;

    fn try_from(ch: u8) -> Result<Self, Self::Error> {
        Ok(match ch {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => return Err(ParseError::InvalidInstruction(ch as char)),
        })
    }
}

#[derive(Debug, Clone, Default)]
struct Node {
    name: String,
    left_ix: usize,
    right_ix: usize,
    is_end: bool,
}

#[derive(Debug, Clone)]
struct Input {
    instructions: Vec<Dir>,
    nodes: Vec<Node>,
    start_ix: usize,
    end_ix: usize,
    start_ixs: Vec<usize>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .ok_or(ParseError::EmptyInput)?
            .bytes()
            .map(Dir::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        match lines.next() {
            Some("") => (),
            _ => return Err(ParseError::MissingSeparatorLine),
        };

        let mut lookup = HashMap::new();
        let mut nodes = Vec::new();
        for line in lines.clone() {
            let (name, line) = line.split_once(" = (").ok_or(ParseError::NodeSyntaxError)?;
            let (left, line) = line.split_once(", ").ok_or(ParseError::NodeSyntaxError)?;
            let right = line.strip_suffix(')').ok_or(ParseError::NodeSyntaxError)?;
            let mut node = Node::default();
            let ix = nodes.len();
            node.name = name.to_string();
            nodes.push(node);
            lookup.insert(name, (ix, left, right));
        }
        let mut start_ix = usize::MAX;
        let mut end_ix = usize::MAX;
        let mut start_ixs = Vec::new();
        for (ix, node) in nodes.iter_mut().enumerate() {
            let &(_, left, right) = lookup
                .get(node.name.as_str())
                .ok_or(ParseError::NodeNotFound)?;
            node.left_ix = lookup.get(left).ok_or(ParseError::NodeNotFound)?.0;
            node.right_ix = lookup.get(right).ok_or(ParseError::NodeNotFound)?.0;
            if node.name.ends_with('A') {
                if node.name.eq("AAA") {
                    start_ix = ix;
                }
                start_ixs.push(ix);
            }
            if node.name.ends_with('Z') {
                if node.name.eq("ZZZ") {
                    end_ix = ix;
                }
                node.is_end = true;
            }
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
        let input = INPUT.parse::<Input>().expect("Parase input");
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = INPUT.parse::<Input>().expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
