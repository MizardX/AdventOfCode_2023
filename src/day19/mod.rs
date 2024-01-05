use bstr::ByteSlice;
use bstr_parse::{BStrParse, ParseIntError};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::fmt::Debug;
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

#[must_use]
pub fn parse_test_input() -> Input {
    INPUT.parse().expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> u64 {
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
        sum += part.total_value();
    }
    sum
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    let mut pending: PendingVec = PendingVec::new();
    pending.push((PartRange::default(), input.workflow_start));
    let mut sum_accepted = 0;
    while let Some((part_range, action)) = pending.pop() {
        match action {
            Action::Accept => {
                sum_accepted += part_range.count();
            }
            Action::Reject => (),
            Action::Forward(next) => input.workflows[next].process_range(part_range, &mut pending),
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
    fn new(field: Field, condition: Condition, value: Value, action: Action) -> Self {
        Self {
            field,
            condition,
            value,
            action,
        }
    }

    fn process(self, part: Part) -> Option<Action> {
        match self.condition {
            Condition::Less if part[self.field] < self.value => Some(self.action),
            Condition::Greater if part[self.field] > self.value => Some(self.action),
            _ => None,
        }
    }

    fn process_range(
        self,
        part_range: &PartRange,
    ) -> (Option<(PartRange, Action)>, Option<PartRange>) {
        // Workflow::process_range() -> Rule::process_range() -> PartRange::split() -> ValueRange::split()
        // Rule::process_range(parts_range) -> (matched, unmatched)
        match self.condition {
            Condition::Less => {
                let (low, high) = part_range.split(self.field, self.value);
                (low.map(|r| (r, self.action)), high)
            }
            Condition::Greater => {
                let (low, high) = part_range.split(self.field, self.value + 1);
                (high.map(|r| (r, self.action)), low)
            }
        }
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {} => {:?}",
            self.field, self.condition as u8 as char, self.value, self.action
        )
    }
}

#[derive(Debug, Clone, Default)]
struct Workflow {
    rules: SmallVec<[Rule; 4]>,
    fallback: Action,
}

type PendingVec = SmallVec<[(PartRange, Action); 16]>;

impl Workflow {
    fn new(rules: SmallVec<[Rule; 4]>, fallback: Action) -> Self {
        Self { rules, fallback }
    }

    fn process(&self, part: Part) -> Action {
        for rule in &self.rules {
            if let Some(action) = rule.process(part) {
                return action;
            }
        }
        self.fallback
    }

    fn process_range(&self, mut part_range: PartRange, pending: &mut PendingVec) {
        // Workflow::process_range() -> Rule::process_range() -> PartRange::split() -> ValueRange::split()
        // Workflow::process_range(parts_range, out matched)
        for &rule in &self.rules {
            let (matched, unmatched) = rule.process_range(&part_range);
            if let Some((matched, action)) = matched {
                pending.push((matched, action));
            }
            if let Some(unmatched) = unmatched {
                part_range = unmatched;
            } else {
                // Nothing remains of the original PartRange
                return;
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

impl Part {
    fn new(x: Value, m: Value, a: Value, s: Value) -> Self {
        Self { x, m, a, s }
    }

    pub fn total_value(self) -> u64 {
        u64::from(self.x) + u64::from(self.m) + u64::from(self.a) + u64::from(self.s)
    }
}

impl<'a> TryFrom<&'a [u8]> for Part {
    type Error = ParseInputError;

    fn try_from(line: &'a [u8]) -> Result<Self, Self::Error> {
        // {x=1858,m=638,a=1227,s=370}
        #[cfg(debug_assertions)]
        if !matches!(&line[..3], b"{x=") {
            return Err(ParseInputError::ExpectedChar('{'));
        }
        let comma = line[3..]
            .find_byte(b',')
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let (x_str, line) = line[3..].split_at(comma);
        let x: Value = x_str.parse()?;

        #[cfg(debug_assertions)]
        if !matches!(&line[..3], b",m=") {
            return Err(ParseInputError::ExpectedChar(','));
        }
        let comma = line[3..]
            .find_byte(b',')
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let (m_str, line) = line[3..].split_at(comma);
        let m: Value = m_str.parse()?;

        #[cfg(debug_assertions)]
        if !matches!(&line[..3], b",a=") {
            return Err(ParseInputError::ExpectedChar(','));
        }
        let comma = line[3..]
            .find_byte(b',')
            .ok_or(ParseInputError::ExpectedChar(','))?;
        let (a_str, line) = line[3..].split_at(comma);
        let a: Value = a_str.parse()?;

        #[cfg(debug_assertions)]
        if !matches!(&line[..3], b",s=") {
            return Err(ParseInputError::ExpectedChar(','));
        }
        let close = line[3..]
            .find_byte(b'}')
            .ok_or(ParseInputError::ExpectedChar('}'))?;
        let (s_str, _line) = line[3..].split_at(close);
        let s: Value = s_str.parse()?;

        #[cfg(debug_assertions)]
        #[allow(clippy::used_underscore_binding)]
        if !matches!(_line, b"}") {
            return Err(ParseInputError::ExpectedChar('}'));
        }

        Ok(Self::new(x, m, a, s))
    }
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

#[derive(Debug, Copy, Clone, Default)]
struct PartRange {
    x: ValueRange,
    m: ValueRange,
    a: ValueRange,
    s: ValueRange,
}

impl PartRange {
    pub fn split(&self, field: Field, value: Value) -> (Option<PartRange>, Option<PartRange>) {
        // Workflow::process_range() -> Rule::process_range() -> PartRange::split() -> ValueRange::split()
        // PartRange::split(field, value) -> (below, above)
        let (low, high) = self[field].split(value);

        (
            low.map(|value_range| self.with(field, value_range)),
            high.map(|value_range| self.with(field, value_range)),
        )
    }

    pub fn with(mut self, field: Field, value_range: ValueRange) -> Self {
        self[field] = value_range;
        self
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

    pub fn split(self, value: Value) -> (Option<ValueRange>, Option<ValueRange>) {
        // Workflow::process_range() -> Rule::process_range() -> PartRange::split() -> ValueRange::split()
        // ValueRange::split(value) -> (below, above)
        if value <= self.start {
            (Some(self), None)
        } else if value >= self.end {
            (None, Some(self))
        } else {
            (
                Some(ValueRange::new(self.start, value)),
                Some(ValueRange::new(value, self.end)),
            )
        }
    }

    pub fn count(self) -> u64 {
        u64::from(self.end).saturating_sub(u64::from(self.start))
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

#[derive(Debug, Clone)]
struct RuleBuilder<'a> {
    field: Field,
    condition: Condition,
    value: Value,
    action_str: &'a [u8],
}

impl<'a> RuleBuilder<'a> {
    pub fn build(&self, name_lookup: &HashMap<&[u8], usize>) -> Result<Rule, ParseInputError> {
        let action = match self.action_str {
            b"A" => Action::Accept,
            b"R" => Action::Reject,
            ref_name => Action::Forward(
                *name_lookup
                    .get(ref_name)
                    .ok_or(ParseInputError::InvalidRuleName)?,
            ),
        };
        Ok(Rule::new(self.field, self.condition, self.value, action))
    }
}

impl<'a> TryFrom<&'a [u8]> for RuleBuilder<'a> {
    type Error = ParseInputError;

    fn try_from(rule_str: &'a [u8]) -> Result<Self, Self::Error> {
        // a>1858:kd
        // s<173:A
        let field: Field = rule_str[0].try_into()?;
        let condition: Condition = rule_str[1].try_into()?;
        let (value_str, action_str) = rule_str[2..]
            .split_once(|&ch| ch == b':')
            .ok_or(ParseInputError::ExpectedChar(':'))?;
        let value: Value = value_str.parse()?;
        Ok(Self {
            field,
            condition,
            value,
            action_str,
        })
    }
}

#[derive(Debug, Clone)]
struct WorkflowBuilder<'a> {
    name: &'a [u8],
    rules: SmallVec<[RuleBuilder<'a>; 4]>,
    fallback_str: &'a [u8],
}

impl<'a> WorkflowBuilder<'a> {
    fn new(name: &'a [u8], rules: SmallVec<[RuleBuilder<'a>; 4]>, fallback_str: &'a [u8]) -> Self {
        Self {
            name,
            rules,
            fallback_str,
        }
    }

    pub fn build(&self, name_lookup: &HashMap<&[u8], usize>) -> Result<Workflow, ParseInputError> {
        let fallback = match self.fallback_str {
            b"A" => Action::Accept,
            b"R" => Action::Reject,
            ref_name => Action::Forward(
                *name_lookup
                    .get(ref_name)
                    .ok_or(ParseInputError::InvalidRuleName)?,
            ),
        };
        let rules = self
            .rules
            .iter()
            .map(|r| r.build(name_lookup))
            .collect::<Result<_, _>>()?;
        Ok(Workflow::new(rules, fallback))
    }
}

impl<'a> TryFrom<&'a [u8]> for WorkflowBuilder<'a> {
    type Error = ParseInputError;

    fn try_from(line: &'a [u8]) -> Result<Self, Self::Error> {
        // tj{x<2412:qh,s<173:A,x>2448:R,R}
        let (name, mut rest) = line
            .split_once(|&ch| ch == b'{')
            .ok_or(ParseInputError::ExpectedChar('{'))?;

        let mut rules = SmallVec::new();
        while let Some((rule_str, tail)) = rest.split_once(|&ch| ch == b',') {
            rules.push(rule_str.try_into()?);
            rest = tail;
        }

        let fallback_str = rest
            .split_once(|&ch| ch == b'}')
            .ok_or(ParseInputError::ExpectedChar('}'))?
            .0;

        Ok(Self::new(name, rules, fallback_str))
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("Did not find expected char: '{0}'")]
    ExpectedChar(char),
    #[error("Not a number: {0:?}")]
    NotANumber(#[from] ParseIntError),
    #[error("Invalid rule name")]
    InvalidRuleName,
}

#[derive(Debug, Clone)]
pub struct Input {
    workflows: Vec<Workflow>,
    workflow_start: Action,
    parts: Vec<Part>,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut name_lookup = HashMap::with_capacity(7 << 7);
        let mut workflow_builders = Vec::with_capacity(539);
        let mut lines = text.as_bytes().lines();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let workflow: WorkflowBuilder = line.try_into()?;
            let index = workflow_builders.len();
            name_lookup.insert(workflow.name, index);
            workflow_builders.push(workflow);
        }
        let mut workflows = Vec::with_capacity(539);
        for builder in &workflow_builders {
            workflows.push(builder.build(&name_lookup)?);
        }
        let workflow_start = Action::Forward(
            *name_lookup
                .get(b"in" as &[u8])
                .ok_or(ParseInputError::InvalidRuleName)?,
        );
        let mut parts = Vec::with_capacity(200);
        for line in lines {
            parts.push(line.try_into()?);
        }
        Ok(Self {
            workflows,
            workflow_start,
            parts,
        })
    }
}
