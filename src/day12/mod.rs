#![warn(clippy::pedantic)]

use std::str::FromStr;

use num_traits::PrimInt;
use smallvec::SmallVec;
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
    let mut cache = vec![None; (6 + 2) * 20];
    for spring in input {
        sum += spring.combinations(&mut cache);
    }
    sum
}

fn part_2(input: &[Input]) -> usize {
    let mut sum = 0;
    let mut cache = vec![None; (6 * 5 + 2) * (20 * 5 + 4)];
    for spring in input {
        let spring2 = spring.expand::<u128, 30>(5);
        let count = spring2.combinations(&mut cache);
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
struct Input<T = u32, const N: usize = 6> {
    len: usize,
    mask: T,
    broken: T,
    counts: SmallVec<[usize; N]>,
}

impl<T, const N: usize> Input<T, N>
where
    T: PrimInt,
{
    fn combinations(&self, cache: &mut [Option<usize>]) -> usize {
        Solver::new(self.len, self.mask, self.broken, &self.counts, cache).solve()
    }

    pub fn expand<T2, const N2: usize>(&self, times: usize) -> Input<T2, N2>
    where
        T2: PrimInt,
    {
        let mut len = self.len;
        let mut mask = T2::from(self.mask).unwrap();
        let mut broken = T2::from(self.broken).unwrap();
        let mut counts = SmallVec::new();
        counts.extend_from_slice(&self.counts);

        for _ in 1..times {
            mask = (mask << (len + 1)) | mask;
            broken = (broken << (len + 1)) | broken;
            counts.extend_from_slice(&self.counts);
        }

        len = times * (len + 1) - 1;

        Input {
            len,
            mask,
            broken,
            counts,
        }
    }
}

impl<T, const N: usize> FromStr for Input<T, N>
where
    T: PrimInt,
{
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let zero = T::zero();
        let one = T::one();

        let mut bytes = s.bytes();
        let mut len = 0;
        let mut mask = zero;
        let mut broken = zero;
        let mut counts = SmallVec::new();
        for b in &mut bytes {
            let (m, b) = match b {
                b'.' => (one, zero),
                b'#' => (one, one),
                b'?' => (zero, zero),
                b' ' => break,
                b => return Err(ParseInputError::InvalidInput(b as char)),
            };
            mask = (mask << 1) | m;
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
            mask,
            broken,
            counts,
        })
    }
}

struct Solver<'a, T> {
    len: usize,
    mask: T,
    broken: T,
    counts: &'a [usize],
    cache: &'a mut [Option<usize>],
}

impl<'a, T> Solver<'a, T>
where
    T: PrimInt,
{
    pub fn new(
        len: usize,
        mask: T,
        broken: T,
        counts: &'a [usize],
        cache: &'a mut [Option<usize>],
    ) -> Self {
        let n = (len + 2) * counts.len();
        debug_assert!(
            cache.len() >= n,
            "length {len}, count {} requires at least {n} size cache, but only got {}",
            counts.len(),
            cache.len()
        );
        for x in &mut cache[0..n] {
            *x = None;
        }
        Self {
            len,
            mask,
            broken,
            counts,
            cache,
        }
    }

    pub fn solve(&mut self) -> usize {
        self.solve_inner(0, 0)
    }

    fn get_cached(&self, ix: usize, offset: usize) -> Option<usize> {
        self.cache[(self.len + 2) * ix + offset]
    }

    fn set_cached(&mut self, ix: usize, offset: usize, value: usize) {
        self.cache[(self.len + 2) * ix + offset] = Some(value);
    }

    fn solve_inner(&mut self, ix: usize, offset: usize) -> usize {
        if ix == self.counts.len() {
            return usize::from((self.broken >> offset).is_zero());
        }
        if let Some(res) = self.get_cached(ix, offset) {
            return res;
        }
        let mut sum = 0;
        let group_size: usize = self.counts[self.counts.len() - 1 - ix];
        let group_bits = (T::one() << group_size) - T::one();
        for group_offset in offset..=self.len - group_size {
            let between_mask = (T::one() << group_offset) - (T::one() << offset);
            if !(self.broken & between_mask).is_zero() {
                break;
            }
            let shifted_group = group_bits << group_offset;
            let check_mask = (shifted_group << 1) | shifted_group;
            if self.broken & check_mask != shifted_group & self.mask {
                continue;
            }
            sum += self.solve_inner(ix + 1, group_offset + group_size + 1);
        }
        self.set_cached(ix, offset, sum);
        sum
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
