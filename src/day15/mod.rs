#![warn(clippy::pedantic)]

use std::str::FromStr;

use smallvec::{smallvec, SmallVec};
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 15");

    println!("++Example");
    let example = Input::from_str(EXAMPLE).expect("Parse example");
    println!("|+-Part 1: {} (expected 1320)", part_1(&example));
    println!("|'-Part 2: {} (expected 145)", part_2(&example));

    println!("++Input");
    let input = Input::from_str(INPUT).expect("Parse input");
    println!("|+-Part 1: {} (expected 508498)", part_1(&input));
    println!("|'-Part 2: {} (expected 279116)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let mut sum = 0;
    for item in &input.steps {
        sum += item.init_hash as usize;
    }
    sum
}

fn part_2(input: &Input) -> usize {
    let mut lens_boxes: SmallVec<[LensBox; 256]> = smallvec![LensBox::default(); 256];

    for step in &input.steps {
        let lens_box = &mut lens_boxes[step.box_hash as usize];
        match step.operation {
            Operation::Remove => {
                lens_box
                    .lenses
                    .retain(|lens| lens.name_hash != step.name_hash);
            }
            Operation::Insert(focal_length) => {
                if let Some(lens) = lens_box
                    .lenses
                    .iter_mut()
                    .find(|a| a.name_hash == step.name_hash)
                {
                    lens.focal_length = focal_length;
                } else {
                    lens_box
                        .lenses
                        .push(Lens::new(step.name_hash, focal_length));
                }
            }
        }
    }

    let mut total_focusing_power = 0;
    for (box_index, lens_box) in lens_boxes.into_iter().enumerate() {
        for (lens_index, lens) in lens_box.lenses.into_iter().enumerate() {
            total_focusing_power +=
                (box_index + 1) * (lens_index + 1) * (lens.focal_length as usize);
        }
    }
    total_focusing_power
}

#[derive(Debug, Default, Clone, Copy)]
struct Lens {
    name_hash: u8,
    focal_length: u8,
}

impl Lens {
    fn new(name_hash: u8, focal_length: u8) -> Self {
        Self {
            name_hash,
            focal_length,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct LensBox {
    lenses: SmallVec<[Lens; 8]>,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Remove,
    Insert(u8),
}

#[derive(Debug, Clone)]
struct Step {
    init_hash: u8,
    box_hash: u8,
    name_hash: u8,
    operation: Operation,
}

#[allow(clippy::cast_lossless)]
fn hash<const F: u8>(b: &[u8]) -> u8 {
    let mut hash: u8 = 0;
    for &ch in b {
        hash = hash.wrapping_add(ch).wrapping_mul(F);
    }
    hash
}

impl TryFrom<&[u8]> for Step {
    type Error = ParseInputError;

    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        let (prefix, operation) = match b {
            [prefix @ .., b'-'] => (prefix, Operation::Remove),
            [prefix @ .., b'=', digit] => (prefix, Operation::Insert(digit - b'0')),
            _ => return Err(ParseInputError::MissingOperation),
        };
        let step = Self {
            init_hash: hash::<17>(b),
            box_hash: hash::<17>(prefix),
            // with ::<6>, all names within the same box get different names
            // ::<2> also gives the correct answer, even though thre are collissions
            name_hash: hash::<6>(prefix),
            operation,
        };
        Ok(step)
    }
}

#[derive(Debug, Clone)]
struct Input {
    steps: Vec<Step>,
}

impl Input {
    pub fn new(steps: Vec<Step>) -> Self {
        Self { steps }
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Missing operation indicator")]
    MissingOperation,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut steps = Vec::new();
        for description in s.as_bytes().split(|&ch| ch == b',') {
            steps.push(description.try_into()?);
        }
        Ok(Self::new(steps))
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(Input::from_str(INPUT))); //.expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = Input::from_str(INPUT).expect("Parse input");
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = Input::from_str(INPUT).expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
