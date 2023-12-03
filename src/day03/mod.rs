#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::ops::{Add, Sub};

pub fn run() {
    println!(".Day 03");

    println!("++Example");
    let example = parse_input(include_str!("example.txt"));
    println!("|+-Part 1: {} (expected 4361)", part_1(&example));
    println!("|'-Part 2: {} (expected 467835)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt"));
    println!("|+-Part 1: {} (expected 532428)", part_1(&input));
    println!("|'-Part 2: {} (expected 84051670)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let mut symbol_lookup = HashMap::with_capacity(input.symbols.len());
    for sym in &input.symbols {
        symbol_lookup.insert(sym.pos, sym.name);
    }
    let mut sum = 0;
    for lbl in &input.labels {
        let mut found = symbol_lookup.contains_key(&(lbl.pos - (0, 1)))
            || symbol_lookup.contains_key(&(lbl.pos + (0, lbl.len)));
        if !found {
            for c in -1..=lbl.len {
                if symbol_lookup.contains_key(&(lbl.pos + (-1, c)))
                    || symbol_lookup.contains_key(&(lbl.pos + (1, c)))
                {
                    found = true;
                    break;
                }
            }
        }
        if found {
            sum += lbl.name as usize;
        }
    }
    sum
}

fn part_2(input: &Input) -> u64 {
    let mut gears: HashMap<Point, Vec<u16>> = input
        .symbols
        .iter()
        .filter_map(|s| {
            if s.name == b'*' {
                Some((s.pos, vec![]))
            } else {
                None
            }
        })
        .collect();
    for lbl in &input.labels {
        if let Some(gear) = gears.get_mut(&(lbl.pos - (0, 1))) {
            gear.push(lbl.name);
        }
        if let Some(gear) = gears.get_mut(&(lbl.pos + (0, lbl.len))) {
            gear.push(lbl.name);
        }
        for c in -1..=lbl.len {
            if let Some(gear) = gears.get_mut(&(lbl.pos + (-1, c))) {
                gear.push(lbl.name);
            }
            if let Some(gear) = gears.get_mut(&(lbl.pos + (1, c))) {
                gear.push(lbl.name);
            }
        }
    }
    let mut sum = 0;
    for gear in gears.values() {
        if gear.len() == 2 {
            sum += u64::from(gear[0]) * u64::from(gear[1]);
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
struct Input {
    symbols: Vec<Symbol>,
    labels: Vec<Label>,
}
impl Input {
    fn new() -> Self {
        Self::default()
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

fn parse_input(text: &str) -> Input {
    let mut res: Input = Input::new();
    for (r, line) in text.lines().enumerate() {
        let r = i16::try_from(r).expect("row < 2^15");
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
                res.labels.push(num);
                number = None;
            }
            if !ch.is_ascii_digit() && ch != b'.' {
                res.symbols.push(Symbol {
                    pos: Point::new(r as _, c as _),
                    name: ch,
                });
            }
        }
        if let Some(num) = number {
            res.labels.push(num);
        }
    }
    res
}
