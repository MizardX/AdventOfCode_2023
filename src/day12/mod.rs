use bstr::ByteSlice;
use num_traits::PrimInt;
use smallvec::SmallVec;
use thiserror::Error;

use crate::aoclib::{parse_int, ParseIntError2};

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

#[must_use]
pub fn parse_test_input() -> Vec<Input> {
    parse_input(INPUT).expect("Real input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &[Input]) -> usize {
    let mut sum = 0;
    let mut cache = [None; (6 + 2) * 20]; // 160
    for spring in input {
        sum += spring.combinations(&mut cache);
    }
    sum
}

#[must_use]
pub fn part_2(input: &[Input]) -> usize {
    let mut sum = 0;
    let mut cache = [None; (6 * 5 + 2) * (20 * 5 + 4)]; // 3328
    for spring in input {
        let spring2 = spring.expand::<u128, 30>(5);
        let count = spring2.combinations(&mut cache);
        sum += count;
    }
    sum
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Invalid character: {0}")]
    InvalidInput(char),
    #[error("Expected character: {0}")]
    Expected(char),
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError2),
}

#[derive(Debug, Clone)]
pub struct Input<T = u32, const N: usize = 6> {
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

    fn expand<T2, const N2: usize>(&self, times: usize) -> Input<T2, N2>
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

impl<'a, T, const N: usize> TryFrom<&'a [u8]> for Input<T, N>
where
    T: PrimInt,
{
    type Error = ParseInputError;

    fn try_from(line: &'a [u8]) -> Result<Self, Self::Error> {
        let zero = T::zero();
        let one = T::one();

        let mut mask = zero;
        let mut broken = zero;
        let mut counts = SmallVec::new();
        let len = line.find_byte(b' ').ok_or(ParseInputError::Expected(' '))?;
        let mut shift = len - 1;
        for &b in &line[..len] {
            let (m, b) = match b {
                b'.' => (one, zero),
                b'#' => (one, one),
                b'?' => (zero, zero),
                b => return Err(ParseInputError::InvalidInput(b as char)),
            };
            mask = mask | (m << shift);
            broken = broken | (b << shift);
            shift = shift.saturating_sub(1);
        }

        let mut start = len + 1;
        while let Some(ix) = line[start..].find_byte(b',') {
            let num: usize = parse_int(&line[start..start + ix])?;
            counts.push(num);
            start += ix + 1;
        }

        let num: usize = parse_int(&line[start..])?;
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
    let mut res: Vec<Input> = Vec::with_capacity(1000);
    for line in text.as_bytes().lines() {
        res.push(line.try_into()?);
    }
    Ok(res)
}
