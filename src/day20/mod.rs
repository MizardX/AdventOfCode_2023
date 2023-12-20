#![warn(clippy::pedantic)]

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
    let example = Input::try_from(EXAMPLE1).expect("Parse example 1");
    println!("|+-Part 1: {} (expected 32000000)", part_1(&example));

    println!("++Example 2");
    let example = Input::try_from(EXAMPLE2).expect("Parse example 2");
    println!("|+-Part 1: {} (expected 11687500)", part_1(&example));

    println!("++Input");
    let input = Input::try_from(INPUT).expect("Parse input");
    println!("|+-Part 1: {} (expected 899848294)", part_1(&input));
    println!("|'-Part 2: {} (expected 247454898168563)", part_2(&input));
    println!("')");
}

fn part_1(input: &Input) -> usize {
    let mut low_count = 0;
    let mut high_count = 0;
    button_mash(input, &mut |button_press, _, _, signal| {
        if button_press == 1001 {
            return Some(low_count * high_count);
        }

        if signal {
            high_count += 1;
        } else {
            low_count += 1;
        }

        None
    })
}

fn part_2(input: &Input) -> usize {
    let rx_parent = try { input.modules[input.rx_index?].sources[0] };
    let mut rx_parent_cycle = [None; 4];
    button_mash(input, &mut |button_press, source, activation, signal| {
        if Some(activation) == rx_parent && signal {
            let source_index = input.modules[activation]
                .sources
                .iter()
                .position(|&i| i == source)
                .unwrap();
            rx_parent_cycle[source_index] = Some(button_press);
            if rx_parent_cycle.iter().all(Option::is_some) {
                return Some(rx_parent_cycle.into_iter().flatten().reduce(lcm).unwrap());
            }
        }
        None
    })
}

fn button_mash(
    input: &Input,
    body: &mut impl FnMut(usize, usize, usize, bool) -> Option<usize>,
) -> usize {
    let mut state: u64 = 0;
    let mut pending = VecDeque::new();
    let broadcast_index = input.modules[input.button_index].destinations[0];
    for button_press in 1.. {
        pending.push_back((input.button_index, broadcast_index, false));
        while let Some((source, activation, signal)) = pending.pop_front() {
            let module = &input.modules[activation];
            let sent_signal = match module.subtype {
                ModuleType::Button => Some(false),
                ModuleType::Broadcaster => Some(signal),
                ModuleType::Sink => None,
                ModuleType::FlipFlop => {
                    if signal {
                        None
                    } else {
                        let bit = 1u64 << activation;
                        state ^= bit;
                        Some((state & bit) != 0)
                    }
                }
                ModuleType::Conjunction => {
                    let bit = 1u64 << source;
                    state = if signal { state | bit } else { state & !bit };
                    Some((state & module.source_mask) != module.source_mask)
                }
            };

            if let Some(res) = body(button_press, source, activation, signal) {
                return res;
            }
            if let Some(new_signal) = sent_signal {
                for &dest in &module.destinations {
                    pending.push_back((activation, dest, new_signal));
                }
            }
        }
    }
    unreachable!()
}

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    Button,
    Broadcaster,
    Sink,
    FlipFlop,
    Conjunction,
}

struct ModuleBuilder<'a> {
    name: &'a str,
    module_type: ModuleType,
    destinations: SmallVec<[&'a str; 6]>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(
        name: &'a str,
        module_type: ModuleType,
        destinations: SmallVec<[&'a str; 6]>,
    ) -> Self {
        Self {
            name,
            module_type,
            destinations,
        }
    }

    pub fn build(
        self,
        name_lookup: &HashMap<&'a str, usize>,
    ) -> Result<Module<'a>, ParseInputError<'a>> {
        let destinations = self
            .destinations
            .iter()
            .map(|n| Ok(*name_lookup.get(n).ok_or(ParseInputError::InvalidName(n))?))
            .collect::<Result<_, _>>()?;

        Ok(Module::new(self.name, self.module_type, destinations))
    }
}

impl<'a> TryFrom<&'a str> for ModuleBuilder<'a> {
    type Error = ParseInputError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }
        let (mut module_type, rest) = if let Some(rest) = value.strip_prefix('%') {
            (ModuleType::FlipFlop, rest)
        } else if let Some(rest) = value.strip_prefix('&') {
            (ModuleType::Conjunction, rest)
        } else {
            (ModuleType::Broadcaster, value)
        };
        let (name, rest) = rest
            .split_once(" -> ")
            .ok_or(ParseInputError::ExpectedArrow)?;
        let mut destinations = SmallVec::new();
        for piece in rest.split(", ") {
            destinations.push(piece);
        }

        if matches!(module_type, ModuleType::Broadcaster) && name != "broadcaster" {
            module_type = ModuleType::Sink;
        }

        Ok(Self::new(name, module_type, destinations))
    }
}

#[derive(Debug, Clone)]
struct Module<'a> {
    _name: &'a str,
    subtype: ModuleType,
    destinations: SmallVec<[usize; 6]>,
    sources: SmallVec<[usize; 10]>,
    source_mask: u64,
}

impl<'a> Module<'a> {
    fn new(name: &'a str, subtype: ModuleType, destinations: SmallVec<[usize; 6]>) -> Self {
        Self {
            _name: name,
            subtype,
            destinations,
            sources: SmallVec::new(),
            source_mask: 0,
        }
    }
}

#[derive(Debug, Error)]
enum ParseInputError<'a> {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Expected arrow")]
    ExpectedArrow,
    #[error("Invalid name")]
    InvalidName(&'a str),
}

#[derive(Debug, Clone)]
struct Input<'a> {
    modules: Vec<Module<'a>>,
    button_index: usize,
    rx_index: Option<usize>,
}

impl<'a> Input<'a> {
    fn new(modules: Vec<Module<'a>>, button_index: usize, rx_index: Option<usize>) -> Self {
        Self {
            modules,
            button_index,
            rx_index,
        }
    }
}

impl<'a> TryFrom<&'a str> for Input<'a> {
    type Error = ParseInputError<'a>;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let mut module_builders: Vec<ModuleBuilder<'a>> = Vec::new();
        let mut name_lookup = HashMap::new();
        let button = ModuleBuilder::new("button", ModuleType::Button, smallvec!["broadcaster"]);
        name_lookup.insert(button.name, 0);
        module_builders.push(button);
        for line in text.lines() {
            let builder: ModuleBuilder<'a> = line.try_into()?;
            let index = module_builders.len();
            name_lookup.insert(builder.name, index);
            module_builders.push(builder);
        }
        for index in 0..module_builders.len() {
            for j in 0..module_builders[index].destinations.len() {
                let dest_name = module_builders[index].destinations[j];
                if let Entry::Vacant(v) = name_lookup.entry(dest_name) {
                    let index = module_builders.len();
                    v.insert(index);
                    module_builders.push(ModuleBuilder::new(
                        dest_name,
                        ModuleType::Sink,
                        SmallVec::new(),
                    ));
                }
            }
        }
        let mut modules: Vec<Module<'a>> = module_builders
            .into_iter()
            .map(|b: ModuleBuilder<'a>| b.build(&name_lookup))
            .try_collect()?;
        for index in 0..modules.len() {
            for j in 0..modules[index].destinations.len() {
                let dest_index = modules[index].destinations[j];
                modules[dest_index].sources.push(index);
                modules[dest_index].source_mask |= 1u64 << index;
            }
        }
        let rx_index = name_lookup.get("rx").copied();
        Ok(Input::new(modules, 0, rx_index))
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(Input::try_from(INPUT).expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = Input::try_from(INPUT).expect("Parse input");
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = Input::try_from(INPUT).expect("Parse input");
        b.iter(|| black_box(part_2(&input)));
    }
}
