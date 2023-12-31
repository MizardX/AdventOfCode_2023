use smallvec::{smallvec, SmallVec};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Dir;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 16");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 46)", part_1(&example));
    println!("|'-Part 2: {} (expected 51)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6605)", part_1(&input));
    println!("|'-Part 2: {} (expected 6766)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> MirrorGraph {
    INPUT.parse().expect("Parse input")
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(graph: &MirrorGraph) -> usize {
    let mut shooter = LaserShooter::new(graph);
    shooter.shoot_laser(0, 0, Dir::E)
}

#[must_use]
pub fn part_2(graph: &MirrorGraph) -> usize {
    let mut shooter = LaserShooter::new(graph);
    let mut max = 0;
    for r in 0..graph.height {
        max = max.max(shooter.shoot_laser(r, 0, Dir::E));
        max = max.max(shooter.shoot_laser(r, graph.width - 1, Dir::W));
    }
    for c in 0..graph.width {
        max = max.max(shooter.shoot_laser(0, c, Dir::S));
        max = max.max(shooter.shoot_laser(graph.height - 1, c, Dir::N));
    }
    max
}

#[derive(Clone, Copy, Default)]
struct DirMap<T>([T; 4]);

impl Debug for DirMap<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last = None;
        let mut multiple = false;
        for d in [Dir::N, Dir::E, Dir::S, Dir::W] {
            if self[d] {
                if last.is_none() {
                    last = Some(d);
                } else {
                    multiple = true;
                }
            }
        }
        if multiple {
            write!(f, "+")
        } else if let Some(d) = last {
            write!(f, "{d:?}")
        } else {
            write!(f, ".")
        }
    }
}

impl<T: Debug + Copy> Debug for DirMap<Option<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = false;
        for d in [Dir::N, Dir::E, Dir::S, Dir::W] {
            if let Some(v) = self[d] {
                if first {
                    write!(f, "[{d:?}: {v:?}")?;
                    first = false;
                } else {
                    write!(f, ", {d:?}: {v:?}")?;
                }
            }
        }
        if first {
            write!(f, "[-]")
        } else {
            write!(f, "]")
        }
    }
}

impl<T> Index<Dir> for DirMap<T> {
    type Output = T;

    fn index(&self, index: Dir) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Dir> for DirMap<T> {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

struct LaserShooter<'a> {
    graph: &'a MirrorGraph,
    visited_tiles: Vec<u128>,
    visited_nodes: Vec<DirMap<bool>>,
    pending: Vec<(usize, Dir)>,
}

impl<'a> LaserShooter<'a> {
    pub fn new(graph: &'a MirrorGraph) -> Self {
        let visited_tiles: Vec<u128> = vec![0; graph.height];
        let visited_nodes: Vec<DirMap<bool>> = vec![DirMap::default(); graph.nodes.len()];
        let pending: Vec<(usize, Dir)> = Vec::new();
        Self {
            graph,
            visited_tiles,
            visited_nodes,
            pending,
        }
    }

    pub fn shoot_laser(&mut self, row: usize, col: usize, dir: Dir) -> usize {
        for vis in &mut self.visited_tiles {
            *vis = 0;
        }
        for dirmap in &mut self.visited_nodes {
            *dirmap = DirMap::default();
        }
        self.pending.clear();
        let start = match self.incoming_beam(row, col, dir) {
            Ok(start) => start,
            Err(result) => return result,
        };
        self.pending.push(start);
        while let Some((ix, dir)) = self.pending.pop() {
            if self.visited_nodes[ix][dir] {
                continue;
            }
            self.visited_nodes[ix][dir] = true;
            let node = &self.graph.nodes[ix];
            for next_dir in node.tile.reflect(dir) {
                if let Some(next_ix) = node.exits[next_dir] {
                    if self.visited_nodes[next_ix][next_dir] {
                        continue; // next_dir
                    }
                    let next_node = &self.graph.nodes[next_ix];
                    self.between_beam(node, next_node);
                    self.pending.push((next_ix, next_dir));
                } else {
                    self.outgoing_beam(node, next_dir);
                }
            }
        }
        let mut sum = 0;
        for &vis in &self.visited_tiles {
            sum += vis.count_ones();
        }
        sum as usize
    }

    #[allow(clippy::cast_sign_loss)]
    fn incoming_beam(&mut self, row: usize, col: usize, dir: Dir) -> Result<(usize, Dir), usize> {
        let bitfield = Bitfield::new(self.graph.width);
        let start_node_ix = match dir {
            Dir::N => {
                let bit = bitfield.bit(col);
                let Some(node_ix) = self.graph.from_south[col] else {
                    return Err(self.graph.height);
                };
                let row = self.graph.nodes[node_ix].row;
                for vis in &mut self.visited_tiles[row..] {
                    *vis |= bit;
                }
                node_ix
            }
            Dir::E => {
                let Some(node_ix) = self.graph.from_west[row] else {
                    return Err(self.graph.width);
                };
                let col = self.graph.nodes[node_ix].col;
                self.visited_tiles[row] |= bitfield.left_of(col);
                node_ix
            }
            Dir::S => {
                let bit = bitfield.bit(col);
                let Some(node_ix) = self.graph.from_north[col] else {
                    return Err(self.graph.height);
                };
                let row = self.graph.nodes[node_ix].row;
                for vis in &mut self.visited_tiles[..=row] {
                    *vis |= bit;
                }
                node_ix
            }
            Dir::W => {
                let Some(node_ix) = self.graph.from_east[row] else {
                    return Err(self.graph.width);
                };
                let col = self.graph.nodes[node_ix].col;
                self.visited_tiles[row] |= bitfield.right_of(col);
                node_ix
            }
        };
        Ok((start_node_ix, dir))
    }

    fn outgoing_beam(&mut self, node: &Node, dir: Dir) {
        let bitfield = Bitfield::new(self.graph.width);
        match dir {
            Dir::N => {
                let bit = bitfield.bit(node.col);
                for vis in &mut self.visited_tiles[..=node.row] {
                    *vis |= bit;
                }
            }
            Dir::E => {
                self.visited_tiles[node.row] |= bitfield.right_of(node.col);
            }
            Dir::S => {
                let bit = bitfield.bit(node.col);
                for vis in &mut self.visited_tiles[node.row..] {
                    *vis |= bit;
                }
            }
            Dir::W => {
                self.visited_tiles[node.row] |= bitfield.left_of(node.col);
            }
        }
    }

    fn between_beam(&mut self, node1: &Node, node2: &Node) {
        let bitfield = Bitfield::new(self.graph.width);
        if node2.row == node1.row {
            // Horizontal
            self.visited_tiles[node1.row] |= bitfield.between(node1.col, node2.col);
        } else {
            // Vertical
            let row1 = node1.row.min(node2.row);
            let row2 = node1.row.max(node2.row);
            let bit = bitfield.bit(node1.col);
            for vis in &mut self.visited_tiles[row1..=row2] {
                *vis |= bit;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Bitfield {
    width: usize,
}

impl Bitfield {
    fn new(width: usize) -> Self {
        Self { width }
    }
}
impl Bitfield {
    // col = 0 => bit = width - 1 (MSB) -- "leftmost"
    // col = width - 1 => bit = 0 (LSB) -- "rightmost"
    pub fn left_of(self, col: usize) -> u128 {
        // inclusive
        // 11110000 <-- LSB
        // '------' width = 8
        // '--' col = 3
        !(!0_u128 << (col + 1)) << (self.width - col - 1)
    }
    pub fn right_of(self, col: usize) -> u128 {
        // inclusive
        // 00011111 <-- LSB
        // '------' width = 8
        // '--' col = 3
        !(!0_u128 << (self.width - col))
    }
    pub fn between(self, col1: usize, col2: usize) -> u128 {
        // inclusive
        // 00111000 <-- LSB
        // '------' width = 8
        // '---' col2 = 4
        // '-' col1 = 2
        // 00111111 right_of(2)
        // 00001111 right_of(4)
        // 00110000 ^
        // 00101000 bit(2) | bit(4)
        // 00111000
        (self.right_of(col1) ^ self.right_of(col2)) | self.bit(col1) | self.bit(col2)
    }
    pub fn bit(self, col: usize) -> u128 {
        let _ = self;
        // 00010000 <-- LSB
        // '------' width = 8
        // '--' col = 3
        1_u128 << (self.width - col - 1)
    }
}

#[derive(Debug, Clone)]
struct Node {
    row: usize,
    col: usize,
    tile: Tile,
    exits: DirMap<Option<usize>>,
}

impl Node {
    fn new(row: usize, col: usize, tile: Tile) -> Self {
        Self {
            row,
            col,
            tile,
            exits: DirMap::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MirrorGraph {
    width: usize,
    height: usize,
    nodes: Vec<Node>,
    from_north: Vec<Option<usize>>,
    from_east: Vec<Option<usize>>,
    from_south: Vec<Option<usize>>,
    from_west: Vec<Option<usize>>,
}

impl FromStr for MirrorGraph {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let mut nodes: Vec<Node> = Vec::new();
        let mut from_north = vec![None; width];
        let mut from_south = from_north.clone();
        let mut from_west = Vec::new();
        let mut from_east = Vec::new();
        for (r, line) in s.lines().enumerate() {
            let mut from_west_r: Option<usize> = None;
            let mut from_east_r: Option<usize> = None;
            for (c, ch) in line.bytes().enumerate() {
                let tile: Tile = ch.try_into()?;
                if tile.is_empty() {
                    continue;
                }

                let mut new_node = Node::new(r, c, tile);
                let new_node_ix = nodes.len();
                if let Some(left_ix) = from_east_r {
                    new_node.exits[Dir::W] = Some(left_ix);
                    nodes[left_ix].exits[Dir::E] = Some(new_node_ix);
                }
                if let Some(above_ix) = from_south[c] {
                    new_node.exits[Dir::N] = Some(above_ix);
                    nodes[above_ix].exits[Dir::S] = Some(new_node_ix);
                }
                nodes.push(new_node);
                from_east_r = Some(new_node_ix);
                from_west_r = from_west_r.or(from_east_r);
                from_south[c] = from_east_r;
                from_north[c] = from_north[c].or(from_east_r);
            }
            from_west.push(from_west_r);
            from_east.push(from_east_r);
        }
        Ok(Self {
            width,
            height: from_west.len(),
            nodes,
            from_north,
            from_east,
            from_south,
            from_west,
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Empty = b'.',
    MirrorTrbl = b'/',
    MirrorTlbr = b'\\',
    SplitterTb = b'|',
    SplitterLr = b'-',
}

impl Tile {
    #[must_use]
    pub fn reflect(self, dir: Dir) -> SmallVec<[Dir; 2]> {
        match (self, dir) {
            (Tile::MirrorTlbr, Dir::W)
            | (Tile::MirrorTrbl, Dir::E)
            | (Tile::Empty | Tile::SplitterTb, Dir::N) => smallvec![Dir::N],
            (Tile::MirrorTlbr, Dir::S)
            | (Tile::MirrorTrbl, Dir::N)
            | (Tile::Empty | Tile::SplitterLr, Dir::E) => smallvec![Dir::E],
            (Tile::MirrorTlbr, Dir::E)
            | (Tile::MirrorTrbl, Dir::W)
            | (Tile::Empty | Tile::SplitterTb, Dir::S) => smallvec![Dir::S],
            (Tile::MirrorTlbr, Dir::N)
            | (Tile::MirrorTrbl, Dir::S)
            | (Tile::Empty | Tile::SplitterLr, Dir::W) => smallvec![Dir::W],
            (Tile::SplitterTb, Dir::E | Dir::W) => smallvec![Dir::N, Dir::S],
            (Tile::SplitterLr, Dir::N | Dir::S) => smallvec![Dir::W, Dir::E],
        }
    }

    #[must_use]
    fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Tile::Empty,
            b'/' => Tile::MirrorTrbl,
            b'\\' => Tile::MirrorTlbr,
            b'|' => Tile::SplitterTb,
            b'-' => Tile::SplitterLr,
            b => return Err(ParseInputError::InvalidChar(b as char)),
        })
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}
