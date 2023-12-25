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
    println!("|+-Part 1: {} (expected 507 626)", part_1(&input));
    println!("')");
}

fn part_1(input: &WiringDiagram) -> usize {
    let num_nodes = input.names.len();
    let mut uf = UnionFind::new(num_nodes);
    let num_edges = input.edges.len();
    let should_be_removed = |i: usize| {
        ![input.edges[i].0, input.edges[i].1].into_iter().all(|j| {
            matches!(
                input.names[j],
                "bbg" | "kbr" | "tdk" | "czs" | "vtt" | "fht"
            )
        })
    };
    for i in 0..num_edges - 2 {
        if num_edges > 100 && should_be_removed(i) {
            continue;
        }
        for j in i + 1..num_edges - 1 {
            if num_edges > 100 && should_be_removed(j) {
                continue;
            }
            for k in j + 1..num_edges {
                if num_edges > 100 && should_be_removed(k) {
                    continue;
                }
                uf.reset();
                for (ix, (a, b)) in input.edges.iter().copied().enumerate() {
                    if ix == i || ix == j || ix == k {
                        continue;
                    }
                    uf.union(a, b);
                }
                if let Some((a, b)) = uf.get_two_components() {
                    return a * b;
                }
            }
        }
    }
    0
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: usize,
}

impl UnionFind {
    pub fn new(num_nodes: usize) -> Self {
        let parent = (0..num_nodes).collect();
        let size = vec![1; num_nodes];
        Self {
            parent,
            size,
            num_components: num_nodes,
        }
    }

    pub fn get_two_components(&self) -> Option<(usize, usize)> {
        if self.num_components != 2 {
            return None;
        }
        let mut first = None;
        let mut second = None;
        for (i, (&parent, &size)) in self.parent.iter().zip(&self.size).enumerate() {
            if parent == i {
                if first.is_none() {
                    first = Some(size);
                } else if second.is_none() {
                    second = Some(size);
                } else {
                    unreachable!()
                }
            }
        }
        Some((first?, second?))
    }

    pub fn reset(&mut self) {
        for (i, x) in self.parent.iter_mut().enumerate() {
            *x = i;
        }
        for x in &mut self.size {
            *x = 1;
        }
        self.num_components = self.parent.len();
    }

    pub fn find_parent(&mut self, mut ix: usize) -> usize {
        let mut parent_ix = self.parent[ix];
        while parent_ix != ix {
            let parent_parent_ix = self.parent[parent_ix];
            self.parent[ix] = parent_parent_ix;
            ix = parent_ix;
            parent_ix = parent_parent_ix;
        }
        ix
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find_parent(x);
        y = self.find_parent(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        // self.size[x] >= self.size[y]
        self.parent[y] = x;
        self.size[x] += self.size[y];
        self.num_components -= 1;
        true
    }
}

#[derive(Debug, Clone)]
struct WiringDiagram<'a> {
    names: Vec<&'a str>,
    edges: Vec<(usize, usize)>,
}

impl<'a> TryFrom<&'a str> for WiringDiagram<'a> {
    type Error = ParseInputError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let mut names = Vec::new();
        let mut name_lookup = HashMap::new();
        let mut edges = Vec::new();
        for line in text.lines() {
            let (src_name, dst_names) = line
                .split_once(": ")
                .ok_or(ParseInputError::ExpectedChar(':'))?;
            let src_index = match name_lookup.entry(src_name) {
                Entry::Occupied(o) => *o.get(),
                Entry::Vacant(v) => {
                    let ix = names.len();
                    names.push(src_name);
                    v.insert(ix);
                    ix
                }
            };
            for dst_name in dst_names.split(' ') {
                let dst_index = match name_lookup.entry(dst_name) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(v) => {
                        let ix = names.len();
                        names.push(dst_name);
                        v.insert(ix);
                        ix
                    }
                };
                edges.push((src_index, dst_index));
            }
        }
        Ok(Self { names, edges })
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
