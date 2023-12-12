#![warn(clippy::pedantic)]

use std::str::FromStr;

use num_traits::PrimInt;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 12");

    println!("++Example");
    let example = parse_input(EXAMPLE).expect("Parse example");
    println!("|+-Part 1: {} (expected 21)", part_1(&example));
    println!("|'-Part 2: {} (expected 525 152)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT).expect("Parse input");
    println!("|+-Part 1: {} (expected 7 694)", part_1(&input));
    println!("|'-Part 2: {} (expected 5 071 883 216 318)", part_2(&input));
    println!("')");
}

fn part_1(input: &[Input]) -> usize {
    let mut sum = 0;
    for spring in input {
        sum += spring.combinations();
    }
    sum
}

fn part_2(input: &[Input]) -> usize {
    let mut sum = 0;
    for spring in input {
        let spring2 = spring.expand::<u128>(5);
        let count = spring2.combinations();
        sum += count;
    }
    sum
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Invalid character: {0}")]
    InvalidInput(char),
}

#[derive(Debug, Clone)]
struct Input<T = u32> {
    len: usize,
    working: T,
    broken: T,
    counts: Vec<usize>,
}

impl<T> Input<T>
where
    T: PrimInt,
{
    fn combinations(&self) -> usize {
        Solver::new(self.len, self.working, self.broken, &self.counts).solve()
    }

    pub fn expand<T2>(&self, times: usize) -> Input<T2>
    where
        T2: PrimInt,
    {
        let mut len = self.len;
        let mut working = T2::from(self.working).unwrap();
        let mut broken = T2::from(self.broken).unwrap();
        let mut counts = Vec::with_capacity(times * len);
        counts.extend(&self.counts);

        for _ in 1..times {
            working = (working << (len + 1)) | working;
            broken = (broken << (len + 1)) | broken;
            counts.extend(&self.counts);
        }

        len = times * (len + 1) - 1;

        Input {
            len,
            working,
            broken,
            counts,
        }
    }
}

struct Solver<'a, T> {
    len: usize,
    working: T,
    broken: T,
    counts: &'a [usize],
    cache: Vec<Option<usize>>,
}

impl<'a, T> Solver<'a, T>
where
    T: PrimInt,
{
    pub fn new(len: usize, working: T, broken: T, counts: &'a [usize]) -> Self {
        let cache = vec![None; (len + 2) * counts.len()];
        Self {
            len,
            working,
            broken,
            counts,
            cache,
        }
    }

    pub fn solve(&mut self) -> usize {
        self.solve_inner(T::zero(), 0, 0)
    }

    fn solve_inner(&mut self, accum: T, ix: usize, offset: usize) -> usize {
        if ix == self.counts.len() {
            return usize::from((accum & (self.broken | self.working)) == self.broken);
        }
        let key = (self.len + 2) * ix + offset;
        if let Some(res) = self.cache[key] {
            return res;
        }
        let mut sum = 0;
        let size: usize = self.counts[self.counts.len() - 1 - ix];
        let group = (T::one() << size) - T::one();
        for i in offset..=self.len - size {
            let broken2 = accum | (group << i);
            let masksize = i + size;
            let mask = (T::one() << masksize).saturating_sub(T::one());
            if broken2 & (self.broken | self.working) & mask == self.broken & mask {
                sum += self.solve_inner(broken2, ix + 1, i + size + 1);
            }
        }
        self.cache[key] = Some(sum);
        sum
    }
}

impl<T> FromStr for Input<T>
where
    T: PrimInt,
{
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let zero = T::zero();
        let one = T::one();

        let mut bytes = s.bytes();
        let mut len = 0;
        let mut working = zero;
        let mut broken = zero;
        let mut counts = Vec::with_capacity(6);
        for b in &mut bytes {
            let (w, b) = match b {
                b'.' => (one, zero),
                b'#' => (zero, one),
                b'?' => (zero, zero),
                b' ' => break,
                b => return Err(ParseInputError::InvalidInput(b as char)),
            };
            working = (working << 1) | w;
            broken = (broken << 1) | b;
            len += 1;
        }
        let mut num = 0;
        for b in bytes {
            num = match b {
                b'0'..=b'9' => 10 * num + (b - b'0') as usize,
                b',' => {
                    counts.push(num);
                    0
                }
                b => return Err(ParseInputError::InvalidInput(b as char)),
            };
        }
        counts.push(num);
        Ok(Self {
            len,
            working,
            broken,
            counts,
        })
    }
}

fn parse_input(text: &str) -> Result<Vec<Input>, ParseInputError> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
        res.push(line.parse()?);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(parse_input(INPUT).expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = parse_input(INPUT).expect("Parse input");
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = parse_input(INPUT).expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
