use bstr::ByteSlice;
use smallvec::{smallvec, SmallVec};
use std::collections::{hash_map::Entry, HashMap, VecDeque};
use thiserror::Error;

use crate::aoclib::lcm;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 20");

    println!("++Example 1");
    let example = Circuit::try_from(EXAMPLE1).expect("Parse example 1");
    println!("|+-Part 1: {} (expected 32000000)", part_1(&example));

    println!("++Example 2");
    let example = Circuit::try_from(EXAMPLE2).expect("Parse example 2");
    println!("|+-Part 1: {} (expected 11687500)", part_1(&example));

    println!("++Input");
    let input = Circuit::try_from(INPUT).expect("Parse input");
    println!("|+-Part 1: {} (expected 899848294)", part_1(&input));
    println!("|'-Part 2: {} (expected 247454898168563)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Circuit<'static> {
    Circuit::try_from(INPUT).expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Circuit) -> usize {
    let mut simulator = CircuitSimulator::new(input);
    for _ in 0..1000 {
        simulator.press_button_once();
    }
    simulator.low_count * simulator.high_count
}

#[must_use]
pub fn part_2(input: &Circuit) -> usize {
    let mut simulator = CircuitSimulator::new(input);
    loop {
        simulator.press_button_once();
        if simulator.cycle_counts.iter().all(Option::is_some) {
            return simulator
                .cycle_counts
                .iter()
                .flatten()
                .copied()
                .reduce(lcm)
                .unwrap_or(0);
        }
    }
}

#[derive(Debug)]
struct CircuitSimulator<'a> {
    circuit: &'a Circuit<'a>,
    state: u64,
    pending: VecDeque<(usize, usize, bool)>,
    low_count: usize,
    high_count: usize,
    cycle_counts: SmallVec<[Option<usize>; 4]>,
    button_presses: usize,
}

impl<'a> CircuitSimulator<'a> {
    fn new(circuit: &'a Circuit<'a>) -> Self {
        Self {
            circuit,
            state: 0,
            pending: VecDeque::with_capacity(50),
            low_count: 0,
            high_count: 0,
            cycle_counts: smallvec![None; 4],
            button_presses: 0,
        }
    }

    fn press_button_once(&mut self) {
        self.button_presses += 1;
        self.pending.push_back((
            self.circuit.button_index,
            self.circuit.broadcast_index,
            false,
        ));
        while self.propagate_one_signal() {}
    }

    fn propagate_one_signal(&mut self) -> bool {
        let Some((source, destination, is_high)) = self.pending.pop_front() else {
            return false;
        };
        let module = &self.circuit.modules[destination];
        let sent_signal = match (module.subtype, is_high) {
            (GateType::Button, _) => Some(false),
            (GateType::Identity, is_high) => Some(is_high),
            (GateType::FlipFlop, true) => None,
            (GateType::FlipFlop, false) => {
                let bit = 1u64 << destination;
                self.state ^= bit;
                Some((self.state & bit) != 0)
            }
            (GateType::Conjunction, true) => {
                let bit = 1u64 << source;
                self.state |= bit;
                Some((self.state & module.source_mask) != module.source_mask)
            }
            (GateType::Conjunction, false) => {
                let bit = 1u64 << source;
                self.state &= !bit;
                Some((self.state & module.source_mask) != module.source_mask)
            }
        };

        if let Some(new_signal) = sent_signal {
            for &dest in &module.destinations {
                self.pending.push_back((destination, dest, new_signal));
            }
        }

        if is_high {
            self.high_count += 1;
        } else {
            self.low_count += 1;
        }

        if is_high && Some(destination) == self.circuit.rx_source_index {
            let source_index = self.circuit.modules[destination]
                .sources
                .iter()
                .position(|&i| i == source)
                .unwrap();
            self.cycle_counts[source_index] = Some(self.button_presses);
        }

        true
    }
}

#[derive(Debug, Clone, Copy)]
enum GateType {
    Button,
    Identity,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    _name: &'a [u8],
    subtype: GateType,
    destinations: SmallVec<[usize; 6]>,
    sources: SmallVec<[usize; 10]>,
    source_mask: u64,
}

impl<'a> Gate<'a> {
    fn new(name: &'a [u8], subtype: GateType, destinations: SmallVec<[usize; 6]>) -> Self {
        Self {
            _name: name,
            subtype,
            destinations,
            sources: SmallVec::new(),
            source_mask: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Circuit<'a> {
    modules: Vec<Gate<'a>>,
    button_index: usize,
    broadcast_index: usize,
    rx_source_index: Option<usize>,
}

impl<'a> Circuit<'a> {
    fn new(
        modules: Vec<Gate<'a>>,
        button_index: usize,
        broadcast_index: usize,
        rx_source_index: Option<usize>,
    ) -> Self {
        Self {
            modules,
            button_index,
            broadcast_index,
            rx_source_index,
        }
    }
}

impl<'a> TryFrom<&'a str> for Circuit<'a> {
    type Error = ParseInputError<'a>;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let mut gate_builders: Vec<GateBuilder<'a>> = Vec::with_capacity(64);
        let mut name_lookup = HashMap::with_capacity(7 << 4);

        let button = GateBuilder::new(
            b"button",
            GateType::Button,
            smallvec![b"broadcaster" as &[u8]],
        );
        name_lookup.insert(button.name, 0);
        gate_builders.push(button);

        for line in text.lines() {
            let builder: GateBuilder<'a> = line.try_into()?; // TryFrom<str> for GateBuilder
            let index = gate_builders.len();
            name_lookup.insert(builder.name, index);
            gate_builders.push(builder);
        }

        // Make sure all destinations exist
        for index in 0..gate_builders.len() {
            for j in 0..gate_builders[index].destinations.len() {
                let dest_name = gate_builders[index].destinations[j];
                if let Entry::Vacant(v) = name_lookup.entry(dest_name) {
                    let index = gate_builders.len();
                    v.insert(index);
                    gate_builders.push(GateBuilder::new(
                        dest_name,
                        GateType::Identity,
                        SmallVec::new(),
                    ));
                }
            }
        }

        let mut gates = Vec::with_capacity(64);
        for builder in gate_builders {
            gates.push(builder.build(&name_lookup)?);
        }

        for index in 0..gates.len() {
            for j in 0..gates[index].destinations.len() {
                let dest_index = gates[index].destinations[j];
                gates[dest_index].sources.push(index);
                gates[dest_index].source_mask |= 1u64 << index;
            }
        }

        let button_index = 0;
        let broadcast_index = gates[button_index].destinations[0];
        let rx_index = name_lookup.get(b"rx" as &[u8]).copied();
        let rx_source_index = try { gates[rx_index?].sources[0] };
        Ok(Circuit::new(
            gates,
            button_index,
            broadcast_index,
            rx_source_index,
        ))
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError<'a> {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Expected arrow")]
    ExpectedArrow,
    #[error("Invalid name")]
    InvalidName(&'a [u8]),
}

struct GateBuilder<'a> {
    name: &'a [u8],
    gate_type: GateType,
    destinations: SmallVec<[&'a [u8]; 6]>,
}

impl<'a> GateBuilder<'a> {
    pub fn new(name: &'a [u8], gate_type: GateType, destinations: SmallVec<[&'a [u8]; 6]>) -> Self {
        Self {
            name,
            gate_type,
            destinations,
        }
    }

    pub fn build(
        self,
        name_lookup: &HashMap<&'a [u8], usize>,
    ) -> Result<Gate<'a>, ParseInputError<'a>> {
        let destinations = self
            .destinations
            .iter()
            .map(|n| Ok(*name_lookup.get(n).ok_or(ParseInputError::InvalidName(n))?))
            .try_collect()?;

        Ok(Gate::new(self.name, self.gate_type, destinations))
    }
}

impl<'a> TryFrom<&'a str> for GateBuilder<'a> {
    type Error = ParseInputError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let value = value.as_bytes();
        if value.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }

        let (gate_type, rest) = match value {
            [b'%', rest @ ..] => (GateType::FlipFlop, rest),
            [b'&', rest @ ..] => (GateType::Conjunction, rest),
            rest => (GateType::Identity, rest),
        };

        let spc = rest.find_byte(b' ').ok_or(ParseInputError::ExpectedArrow)?;
        let (name, rest) = rest.split_at(spc);
        #[cfg(debug_assertions)]
        if !matches!(&rest[..4], b" -> ") {
            return Err(ParseInputError::ExpectedArrow);
        }
        let rest = &rest[4..];

        let mut destinations = SmallVec::new();
        for piece in rest.split(|&ch| ch == b',') {
            destinations.push(piece.trim_ascii());
        }

        Ok(Self::new(name, gate_type, destinations))
    }
}
