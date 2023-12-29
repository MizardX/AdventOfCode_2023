#![warn(clippy::pedantic)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
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
    let mut uf_outer = UnionFind::new(n);
    let mut best_cut = None;

    while uf_outer.num_components > 1 {
        let mut uf_inner = uf_outer.clone();
        let mut root = uf_inner.find_root(0);
        let mut second_last = root;
        while uf_inner.num_components > 2 {
            let mut tightest = None;
            for candidate in 0..n {
                let candidate_root = uf_inner.find_root(candidate);
                if candidate_root == root || candidate_root != candidate {
                    continue;
                }
                let mut weight = 0_usize;
                for &edge in &input.edges {
                    let a_root = uf_inner.find_root(edge.from);
                    let b_root = uf_inner.find_root(edge.to);
                    if (a_root, b_root) == (candidate_root, root)
                        || (a_root, b_root) == (root, candidate_root)
                    {
                        weight += 1;
                    }
                }
                tightest = match tightest {
                    Some((_, w)) if w >= weight => tightest,
                    _ => Some((candidate, weight)),
                };
            }
            let (tightest, cut) = tightest.expect("At least one unmerged node remaining");

            second_last = tightest;
            uf_inner.union(root, tightest);
            root = uf_inner.find_root(0);
        }

        let mut tightest = None;
        for last in 0..n {
            let last_root = uf_inner.find_root(last);
            if last_root == root || last_root != last {
                continue;
            }
            let mut weight = 0_usize;
            for &edge in &input.edges {
                let a_root = uf_inner.find_root(edge.from);
                let b_root = uf_inner.find_root(edge.to);
                if (a_root, b_root) == (last_root, root) || (a_root, b_root) == (root, last_root) {
                    weight += 1;
                }
            }
            tightest = match tightest {
                Some((_, w)) if w >= weight => tightest,
                _ => Some((last, weight)),
            };
        }
        let (last, cut) = tightest.expect("At least two unmerged nodes remaining");


        let grouped = uf_inner.group_size(root);
        let ungrouped = uf_inner.group_size(last);


        best_cut = match best_cut {
            Some((c, _, _)) if c <= cut => best_cut,
            _ => {
                Some((cut, grouped, ungrouped))
            },
        };

        uf_outer.union(second_last, last);
    }

    dbg!(best_cut);

    let best_cut = best_cut.expect("Minimum exists");
    best_cut.1 * best_cut.2
}

#[derive(Debug, Clone)]
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

    pub fn find_root(&mut self, mut ix: usize) -> usize {
        let mut parent_ix = self.nodes[ix].parent;
        while parent_ix != ix {
            let parent_parent_ix = self.nodes[parent_ix].parent;
            self.nodes[ix].parent = parent_parent_ix;
            ix = parent_ix;
            parent_ix = parent_parent_ix;
        }
        ix
    }

    pub fn group_size(&mut self, mut ix: usize) -> usize {
        ix = self.find_root(ix);
        self.nodes[ix].size
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
        self.nodes[y].parent = x;
        self.nodes[x].size += self.nodes[y].size;
        self.num_components -= 1;
        true
    }
}

#[derive(Debug, Clone, Copy)]
struct UnionFindNode {
    parent: usize,
    size: usize,
}

impl UnionFindNode {
    fn new(parent: usize, size: usize) -> Self {
        Self { parent, size }
    }
}

#[derive(Clone)]
struct Node<'a> {
    name: &'a str,
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Node").field(&self.name).finish()
    }
}

#[derive(Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.from).field(&self.to).finish()
    }
}

#[derive(Debug, Clone)]
struct WiringDiagram<'a> {
    components: Vec<Node<'a>>,
    edges: Vec<Edge>,
}

impl<'a> TryFrom<&'a str> for WiringDiagram<'a> {
    type Error = ParseInputError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let mut components: Vec<Node<'a>> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();
        let mut name_lookup = HashMap::new();
        for line in text.lines() {
            let (name, rest) = line
                .split_once(':')
                .ok_or(ParseInputError::ExpectedChar(':'))?;
            let from_ix = match name_lookup.entry(name) {
                Entry::Occupied(o) => *o.get(),
                Entry::Vacant(v) => {
                    let ix = components.len();
                    components.push(Node { name });
                    v.insert(ix);
                    ix
                }
            };
            for to in rest.trim_start().split(' ') {
                let to_ix = match name_lookup.entry(to) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(v) => {
                        let ix = components.len();
                        components.push(Node { name: to });
                        v.insert(ix);
                        ix
                    }
                };
                edges.push(Edge::new(from_ix, to_ix));
            }
        }
        Ok(Self { components, edges })
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
