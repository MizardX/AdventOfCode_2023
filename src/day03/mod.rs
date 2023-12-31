use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Add, Sub};

use smallvec::SmallVec;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 03");

    println!("++Example");
    let example = parse_input(EXAMPLE);
    println!("|+-Part 1: {} (expected 4361)", part_1(&example));
    println!("|'-Part 2: {} (expected 467835)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT);
    println!("|+-Part 1: {} (expected 532428)", part_1(&input));
    println!("|'-Part 2: {} (expected 84051670)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Input {
    parse_input(INPUT)
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut sum: usize = 0;
    let mut symbol_lookups = [(); 3].map(|()| BTreeSet::new());
    for sym in &input.symbols[0] {
        symbol_lookups[2].insert(sym.pos.col);
    }
    for (r, lblrow) in input.labels.iter().enumerate() {
        symbol_lookups.rotate_left(1);
        symbol_lookups[2].clear();
        if r + 1 < input.symbols.len() {
            for sym in &input.symbols[r + 1] {
                symbol_lookups[2].insert(sym.pos.col);
            }
        }
        for lbl in lblrow {
            let rng = (lbl.pos.col - 1)..=(lbl.pos.col + lbl.len);
            let mut count = 0;
            for l in &symbol_lookups {
                let matched = l.range(rng.clone()).collect::<SmallVec<[_; 3]>>();
                count += matched.len();
            }
            if count > 0 {
                sum += lbl.name as usize;
            }
        }
    }

    sum
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut sum: usize = 0;
    let mut num_lookups = [(); 3].map(|()| BTreeMap::new());
    for lbl in &input.labels[0] {
        num_lookups[2].insert(lbl.pos.col, lbl);
    }
    for r in 0..input.labels.len() {
        num_lookups.rotate_left(1);
        num_lookups[2].clear();
        if r + 1 < input.labels.len() {
            for lbl in &input.labels[r + 1] {
                num_lookups[2].insert(lbl.pos.col, lbl);
            }
        }
        'symbol: for sym in &input.symbols[r] {
            if sym.name != b'*' {
                continue;
            }
            let rng = (sym.pos.col - 3)..(sym.pos.col + 2);
            let mut prod = 1;
            let mut count = 0;
            for lookup in &num_lookups {
                for (&c, &lbl) in lookup.range(rng.clone()) {
                    if c + lbl.len >= sym.pos.col {
                        if count >= 2 {
                            continue 'symbol;
                        }
                        prod *= lbl.name as usize;
                        count += 1;
                    }
                }
            }
            if count == 2 {
                sum += prod;
            }
        }
    }
    sum
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Point {
    row: i16,
    col: i16,
}
impl Point {
    fn new(row: i16, col: i16) -> Self {
        Self { row, col }
    }
}
impl Add<(i16, i16)> for Point {
    type Output = Self;

    fn add(self, rhs: (i16, i16)) -> Self::Output {
        Self {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}
impl Sub<(i16, i16)> for Point {
    type Output = Self;

    fn sub(self, rhs: (i16, i16)) -> Self::Output {
        Self {
            row: self.row - rhs.0,
            col: self.col - rhs.1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    symbols: Vec<SmallVec<[Symbol; 9]>>,
    labels: Vec<SmallVec<[Label; 16]>>,
}
impl Input {
    fn new() -> Self {
        Self {
            symbols: Vec::with_capacity(140),
            labels: Vec::with_capacity(140),
        }
    }
}

#[derive(Debug, Clone)]
struct Label {
    pos: Point,
    len: i16,
    name: u16,
}

#[derive(Debug, Clone)]
struct Symbol {
    pos: Point,
    name: u8,
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn parse_input(text: &str) -> Input {
    let mut res: Input = Input::new();
    for (r, line) in text.lines().enumerate() {
        res.labels.push(SmallVec::new());
        res.symbols.push(SmallVec::new());
        let mut number: Option<Label> = None;
        for (c, ch) in line.bytes().enumerate() {
            let c = i16::try_from(c).expect("col < 2^15");
            if ch.is_ascii_digit() {
                if let Some(num) = number.as_mut() {
                    num.len += 1;
                    num.name = 10 * num.name + u16::from(ch - b'0');
                } else {
                    number = Some(Label {
                        pos: Point::new(r as _, c as _),
                        len: 1,
                        name: u16::from(ch - b'0'),
                    });
                }
            } else if let Some(num) = number {
                res.labels[r].push(num);
                number = None;
            }
            if !ch.is_ascii_digit() && ch != b'.' {
                res.symbols[r].push(Symbol {
                    pos: Point::new(r as _, c as _),
                    name: ch,
                });
            }
        }
        if let Some(num) = number {
            res.labels[r].push(num);
        }
    }
    res
}

