#![warn(clippy::pedantic)]

use smallvec::SmallVec;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 25");

    println!("++Example");
    let example = EXAMPLE.try_into().expect("Parse example");
    println!("|+-Part 1: {} (expected 54)", part_1(&example));

    println!("++Input");
    let input = INPUT.try_into().expect("Parse input");
    println!("|+-Part 1: {} (expected 507626)", part_1(&input));
    println!("')");
}

fn part_1(input: &WiringDiagram) -> usize {
    let n = input.components.len();
    let mut heat = vec![0; n * n];
    let mut visited = vec![false; n];
    let mut queue = VecDeque::with_capacity(n);
    for source in 0..n {
        for v in &mut visited {
            *v = false;
        }
        queue.push_back((source, source));
        while let Some((came_from, node)) = queue.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            heat[came_from * n + node] += 1;
            heat[came_from + node * n] += 1;

            for &next in &input.components[node].edges {
                if visited[next] {
                    continue;
                }
                queue.push_back((node, next));
            }
        }
    }
    let mut hottest = BinaryHeap::with_capacity(7);
    for (i, h) in heat.into_iter().enumerate() {
        hottest.push((Reverse(h), i / n, i % n));
        if hottest.len() > 6 {
            hottest.pop();
        }
    }
    let mut critical: Vec<usize> = Vec::with_capacity(6);
    while let Some((_, n1, n2)) = hottest.pop() {
        if !critical.contains(&n1) {
            critical.push(n1);
        }
        if !critical.contains(&n2) {
            critical.push(n2);
        }
    }
    for v in &mut visited {
        *v = false;
    }
    let mut num_visited = 0;
    queue.push_back((0, 0));
    while let Some((_, node)) = queue.pop_front() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        num_visited += 1;
        for &next in &input.components[node].edges {
            if visited[next] {
                continue;
            }
            if critical.contains(&node) && critical.contains(&next) {
                continue;
            }
            queue.push_back((next, next));
        }
    }
    num_visited * (n - num_visited)
}

#[derive(Clone)]
struct Nodes<'a> {
    name: &'a str,
    edges: SmallVec<[usize; 9]>,
}

impl<'a> Debug for Nodes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.name)
            .field(&self.edges)
            .finish()
    }
}

#[derive(Debug, Clone)]
struct WiringDiagram<'a> {
    components: Vec<Nodes<'a>>,
}

impl<'a> TryFrom<&'a str> for WiringDiagram<'a> {
    type Error = ParseInputError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let mut component_builders: Vec<ComponentBuilder<'a>> = Vec::new();
        let mut name_lookup = HashMap::new();
        for line in text.lines() {
            let builder: ComponentBuilder<'a> = line.try_into()?;
            let ix = component_builders.len();
            name_lookup.insert(builder.name, ix);
            component_builders.push(builder);
        }
        for ix in 0..component_builders.len() {
            for j in 0..component_builders[ix].connected_to.len() {
                let name = component_builders[ix].connected_to[j];
                let ix2 = match name_lookup.entry(name) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(v) => {
                        let ix = component_builders.len();
                        let cmp = ComponentBuilder::new(name, SmallVec::new());
                        component_builders.push(cmp);
                        v.insert(ix);
                        ix
                    }
                };
                component_builders[ix].connected_to_ixs.push(ix2);
                component_builders[ix2].connected_from_ixs.push(ix);
            }
        }
        let components = component_builders.into_iter().map(|b| b.build()).collect();
        Ok(Self { components })
    }
}

#[derive(Clone)]
struct ComponentBuilder<'a> {
    name: &'a str,
    connected_to: SmallVec<[&'a str; 6]>,
    connected_to_ixs: SmallVec<[usize; 6]>,
    connected_from_ixs: SmallVec<[usize; 6]>,
}

impl<'a> ComponentBuilder<'a> {
    pub fn new(name: &'a str, connected_to: SmallVec<[&'a str; 6]>) -> Self {
        Self {
            name,
            connected_to,
            connected_to_ixs: SmallVec::new(),
            connected_from_ixs: SmallVec::new(),
        }
    }

    pub fn build(&self) -> Nodes<'a> {
        let mut connected_to = SmallVec::new();
        connected_to.extend_from_slice(&self.connected_to_ixs);
        connected_to.extend_from_slice(&self.connected_from_ixs);
        Nodes {
            name: self.name,
            edges: connected_to,
        }
    }
}

impl<'a> TryFrom<&'a str> for ComponentBuilder<'a> {
    type Error = ParseInputError;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let (name, rest) = line
            .split_once(": ")
            .ok_or(ParseInputError::ExpectedChar(':'))?;
        let mut connected_to = SmallVec::new();
        for ref_name in rest.split(' ') {
            connected_to.push(ref_name);
        }
        Ok(Self::new(name, connected_to))
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Expected character: '{0}'")]
    ExpectedChar(char),
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(WiringDiagram::try_from(INPUT).expect("Parse input")));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = INPUT.try_into().expect("Parse input");
        b.iter(|| black_box(part_1(&input)));
    }
}
