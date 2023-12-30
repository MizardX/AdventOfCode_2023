#![warn(clippy::pedantic)]

use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use smallvec::SmallVec;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if input is malformed.
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

#[must_use]
pub fn part_1(input: &WiringDiagram) -> usize {
    let mut karger = Karger::new(input);
    let (a, b) = karger.run_to_completion();
    a * b
}

/// Implementation of Karger's algoritm. <https://en.wikipedia.org/wiki/Karger%27s_algorithm>
/// Adapted from <https://github.com/kuviman/advent_of_code_2023/blob/d6838bfca881c38134b357636604bf98b69833dd/src/bin/day25/main.rs#L74>
struct Karger<'a> {
    diagram: &'a WiringDiagram<'a>,
    edges: Vec<[usize; 2]>,
    union_find: UnionFind,
    rng: ThreadRng,
}

impl<'a> Karger<'a> {
    fn new(diagram: &'a WiringDiagram) -> Self {
        Self {
            diagram,
            edges: Vec::with_capacity(diagram.edges.len()),
            union_find: UnionFind::new(diagram.components.len()),
            rng: thread_rng(),
        }
    }

    fn reset(&mut self) {
        self.edges.clear();
        self.edges.extend_from_slice(&self.diagram.edges);
        self.union_find.reset();
    }

    fn single_cycle(&mut self) -> usize {
        self.reset();

        while self.union_find.num_components > 2 {
            let [a, b] = self
                .edges
                .swap_remove(self.rng.gen_range(0..self.edges.len()));
            self.union_find.union(a, b);
        }

        self.edges
            .iter()
            .filter(|&&[a, b]| self.union_find.find_root(a) != self.union_find.find_root(b))
            .count()
    }

    fn run_to_completion(&mut self) -> (usize, usize) {
        loop {
            let edges_left = self.single_cycle();

            if edges_left == 3 {
                return self
                    .union_find
                    .get_two_components()
                    .expect("Two components left");
            }
        }
    }
}

struct UnionFindNode {
    parent_ix: usize,
    size: usize,
}

impl UnionFindNode {
    fn new(parent_ix: usize, size: usize) -> Self {
        Self { parent_ix, size }
    }
}

struct UnionFind {
    nodes: Vec<UnionFindNode>,
    num_components: usize,
}

impl UnionFind {
    pub fn new(num_nodes: usize) -> Self {
        let nodes = (0..num_nodes).map(|i| UnionFindNode::new(i, 1)).collect();
        Self {
            nodes,
            num_components: num_nodes,
        }
    }

    pub fn reset(&mut self) {
        for (ix, node) in self.nodes.iter_mut().enumerate() {
            node.parent_ix = ix;
            node.size = 1;
        }
        self.num_components = self.nodes.len();
    }

    pub fn get_two_components(&self) -> Option<(usize, usize)> {
        if self.num_components != 2 {
            return None;
        }
        let mut first = None;
        let mut second = None;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.parent_ix == i {
                if first.is_none() {
                    first = Some(node.size);
                } else if second.is_none() {
                    second = Some(node.size);
                } else {
                    unreachable!()
                }
            }
        }
        Some((first?, second?))
    }

    pub fn find_root(&mut self, mut ix: usize) -> usize {
        let mut parent_ix = self.nodes[ix].parent_ix;
        while parent_ix != ix {
            let parent_parent_ix = self.nodes[parent_ix].parent_ix;
            self.nodes[ix].parent_ix = parent_parent_ix;
            ix = parent_ix;
            parent_ix = parent_parent_ix;
        }
        ix
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find_root(x);
        y = self.find_root(y);
        if x == y {
            return false;
        }
        if self.nodes[x].size < self.nodes[y].size {
            std::mem::swap(&mut x, &mut y);
        }
        // self.size[x] >= self.size[y]
        self.nodes[y].parent_ix = x;
        self.nodes[x].size += self.nodes[y].size;
        self.num_components -= 1;
        true
    }
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
pub struct WiringDiagram<'a> {
    components: Vec<&'a str>,
    edges: Vec<[usize; 2]>,
}

impl<'a> TryFrom<&'a str> for WiringDiagram<'a> {
    type Error = ParseInputError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let mut name_lookup = HashMap::new();
        let mut components = Vec::new();
        let mut edges = Vec::new();
        for line in text.lines() {
            let (name, rest) = line
                .split_once(':')
                .ok_or(ParseInputError::ExpectedChar(':'))?;
            let node_ix = match name_lookup.entry(name) {
                Entry::Occupied(o) => *o.get(),
                Entry::Vacant(v) => {
                    let ix = components.len();
                    components.push(name);
                    v.insert(ix);
                    ix
                }
            };
            for neighbor in rest.trim_start().split(' ') {
                let neighbor_ix = match name_lookup.entry(neighbor) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(v) => {
                        let ix = components.len();
                        components.push(neighbor);
                        v.insert(ix);
                        ix
                    }
                };
                edges.push([node_ix, neighbor_ix]);
            }
        }
        Ok(Self { components, edges })
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Expected character: '{0}'")]
    ExpectedChar(char),
}

/// # Panics
///
/// Panics if input is malformed.

#[must_use]
pub fn parse_test_input() -> WiringDiagram<'static> {
    INPUT.try_into().expect("Parse input")
}

#[must_use]
pub fn run_cycles(input: &WiringDiagram, cycles: usize) -> usize {
    let mut kerger = Karger::new(input);
    let mut sum = 0;
    for _ in 0..cycles {
        sum += kerger.single_cycle();
    }
    sum
}
