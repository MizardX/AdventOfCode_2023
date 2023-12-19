#![warn(clippy::pedantic)]

use smallvec::SmallVec;
use std::collections::HashMap;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 19");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 19 114)", part_1(&example));
    println!(
        "|'-Part 2: {} (expected 167 409 079 868 000)",
        part_2(&example)
    );

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 382 440)", part_1(&input));
    println!(
        "|'-Part 2: {} (expected 136 394 217 540 123)",
        part_2(&input)
    );
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let mut sum = 0;
    'parts: for &part in &input.parts {
        let mut action = input.workflow_start;
        'workflow: loop {
            action = match action {
                Action::Forward(next) => input.workflows[next].process(part),
                Action::Accept => break 'workflow,
                Action::Reject => continue 'parts,
            }
        }
        sum += part.x as usize + part.m as usize + part.a as usize + part.s as usize;
    }
    sum
}

fn part_2(input: &Input) -> u64 {
    let mut pending = Vec::new();
    pending.push((PartRange::default(), input.workflow_start));
    let mut sum_accepted = 0;
    while let Some((part_range, action)) = pending.pop() {
        match action {
            Action::Accept => sum_accepted += part_range.count(),
            Action::Reject => (),
            Action::Forward(next) => input.workflows[next].split(part_range, &mut pending),
        }
    }
    sum_accepted
}

type Value = u16;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Field {
    X = b'x',
    M = b'm',
    A = b'a',
    S = b's',
}

impl TryFrom<u8> for Field {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'x' => Self::X,
            b'm' => Self::M,
            b'a' => Self::A,
            b's' => Self::S,
            ch => return Err(ParseInputError::InvalidChar(ch as char)),
        })
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Condition {
    Less = b'<',
    Greater = b'>',
}

impl TryFrom<u8> for Condition {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'<' => Self::Less,
            b'>' => Self::Greater,
            ch => return Err(ParseInputError::InvalidChar(ch as char)),
        })
    }
}

#[derive(Debug, Copy, Clone, Default)]
enum Action {
    #[default]
    Accept,
    Reject,
    Forward(usize),
}

#[derive(Copy, Clone)]
struct Rule {
    field: Field,
    condition: Condition,
    value: Value,
    action: Action,
}

impl Rule {
    fn process(self, part: Part) -> Option<Action> {
        match self.condition {
            Condition::Less if part[self.field] < self.value => Some(self.action),
            Condition::Greater if part[self.field] > self.value => Some(self.action),
            _ => None,
        }
    }

    fn split(self, part_range: &PartRange) -> Option<(PartRange, Action, PartRange)> {
        match self.condition {
            Condition::Less => {
                let (low, high) = part_range.split(self.field, self.value)?;
                Some((low, self.action, high))
            }
            Condition::Greater => {
                let (low, high) = part_range.split(self.field, self.value + 1)?;
                Some((high, self.action, low))
            }
        }
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            field,
            condition,
            value,
            action,
        } = self;
        write!(
            f,
            "{field:?} {} {value} => {action:?}",
            *condition as u8 as char
        )
    }
}

#[derive(Debug, Clone, Default)]
struct Workflow {
    rules: SmallVec<[Rule; 4]>,
    fallback: Action,
}

impl Workflow {
    fn process(&self, part: Part) -> Action {
        for rule in &self.rules {
            if let Some(action) = rule.process(part) {
                return action;
            }
        }
        self.fallback
    }

    fn split(&self, mut part_range: PartRange, pending: &mut Vec<(PartRange, Action)>) {
        for &rule in &self.rules {
            if let Some((low, action, high)) = rule.split(&part_range) {
                pending.push((low, action));
                part_range = high;
            }
        }
        pending.push((part_range, self.fallback));
    }
}

#[derive(Debug, Copy, Clone)]
struct Part {
    x: Value,
    m: Value,
    a: Value,
    s: Value,
}

impl Index<Field> for Part {
    type Output = Value;

    fn index(&self, index: Field) -> &Self::Output {
        match index {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }
}

impl IndexMut<Field> for Part {
    fn index_mut(&mut self, index: Field) -> &mut Self::Output {
        match index {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct PartRange {
    x: ValueRange,
    m: ValueRange,
    a: ValueRange,
    s: ValueRange,
}

impl PartRange {
    pub fn split(&self, field: Field, value: Value) -> Option<(PartRange, PartRange)> {
        let (low, high) = self[field].split(value)?;
        let mut res1 = self.clone();
        let mut res2 = self.clone();
        res1[field] = low;
        res2[field] = high;
        Some((res1, res2))
    }

    pub fn count(&self) -> u64 {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }
}

impl Index<Field> for PartRange {
    type Output = ValueRange;

    fn index(&self, index: Field) -> &Self::Output {
        match index {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }
}

impl IndexMut<Field> for PartRange {
    fn index_mut(&mut self, index: Field) -> &mut Self::Output {
        match index {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }
}

#[derive(Copy, Clone)]
struct ValueRange {
    start: Value,
    end: Value,
}

impl ValueRange {
    fn new(start: Value, end: Value) -> Self {
        Self { start, end }
    }

    pub fn contains(self, value: Value) -> bool {
        self.start <= value && value < self.end
    }

    pub fn split(self, value: Value) -> Option<(ValueRange, ValueRange)> {
        self.contains(value).then_some((
            ValueRange::new(self.start, value),
            ValueRange::new(value, self.end),
        ))
    }

    pub fn count(self) -> u64 {
        u64::from(self.end - self.start)
    }
}

impl Default for ValueRange {
    fn default() -> Self {
        Self {
            start: 1,
            end: 4001,
        }
    }
}

impl Debug for ValueRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { start, end } = self;
        write!(f, "{start}..{end}")
    }
}

impl FromStr for Part {
    type Err = ParseInputError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let rest = line
            .strip_prefix("{x=")
            .ok_or(ParseInputError::ExpectedChar('{'))?;
        let (x_str, rest) = rest
            .split_once(",m=")
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let x: Value = x_str.parse()?;
        let (m_str, rest) = rest
            .split_once(",a=")
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let m: Value = m_str.parse()?;
        let (a_str, rest) = rest
            .split_once(",s=")
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let a: Value = a_str.parse()?;
        let s_str = rest
            .strip_suffix('}')
            .ok_or(ParseInputError::ExpectedChar('}'))?;
        let s: Value = s_str.parse()?;
        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Clone)]
struct Input {
    workflows: Vec<Workflow>,
    workflow_start: Action,
    parts: Vec<Part>,
}

#[derive(Debug, Error)]
enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("Did not find expected char: '{0}'")]
    ExpectedChar(char),
    #[error("Not a number: {0:?}")]
    NotANumber(#[from] ParseIntError),
    #[error("Invalid rule name")]
    InvalidRuleName,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut name_lookup = HashMap::new();
        for line in text.lines() {
            if line.is_empty() {
                break;
            }
            let (name, _) = line
                .split_once('{')
                .ok_or(ParseInputError::ExpectedChar('{'))?;
            let index = name_lookup.len();
            name_lookup.insert(name, index);
        }
        let mut workflows: Vec<Workflow> = Vec::new();
        let mut lines = text.lines();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let mut rules = SmallVec::new();
            let (_, mut rest) = line.split_once('{').unwrap(); // already checked
            while let Some((rule_str, tail)) = rest.split_once(',') {
                let rule_b = rule_str.as_bytes();
                let field: Field = rule_b[0].try_into()?;
                let condition: Condition = rule_b[1].try_into()?;
                let (value_str, action_str) = rule_str[2..]
                    .split_once(':')
                    .ok_or(ParseInputError::ExpectedChar(':'))?;
                let value: Value = value_str.parse()?;
                let action = match action_str {
                    "A" => Action::Accept,
                    "R" => Action::Reject,
                    ref_name => Action::Forward(
                        *name_lookup
                            .get(ref_name)
                            .ok_or(ParseInputError::InvalidRuleName)?,
                    ),
                };
                rules.push(Rule {
                    field,
                    condition,
                    value,
                    action,
                });
                rest = tail;
            }
            let fallback = match rest
                .strip_suffix('}')
                .ok_or(ParseInputError::ExpectedChar('}'))?
            {
                "A" => Action::Accept,
                "R" => Action::Reject,
                ref_name => Action::Forward(
                    *name_lookup
                        .get(ref_name)
                        .ok_or(ParseInputError::InvalidRuleName)?,
                ),
            };
            workflows.push(Workflow { rules, fallback });
        }
        let workflow_start = Action::Forward(
            *name_lookup
                .get("in")
                .ok_or(ParseInputError::InvalidRuleName)?,
        );
        let mut parts = Vec::new();
        for line in lines {
            parts.push(line.parse()?);
        }
        Ok(Self {
            workflows,
            workflow_start,
            parts,
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
        b.iter(|| black_box(INPUT.parse::<Input>().expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = INPUT.parse().expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
