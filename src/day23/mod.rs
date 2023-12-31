use smallvec::{smallvec, SmallVec};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{BitAnd, Deref};
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonParseError, Dir, Grid, Pos};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 23");

    println!("++Example");
    let example = Graph::from(&EXAMPLE.parse::<Map>().expect("Parse example"));
    println!("|+-Part 1: {} (expected 94)", part_1(&example));
    println!("|'-Part 2: {} (expected 154)", part_2(&example));

    println!("++Input");
    let input = Graph::from(&INPUT.parse::<Map>().expect("Parse input"));
    println!("|+-Part 1: {} (expected 2402)", part_1(&input));
    println!("|'-Part 2: {} (expected 6450)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input() -> Map {
    INPUT.parse::<Map>().expect("Parse input")
}

#[must_use]
pub fn transform_test_input(input: &Map) -> Graph {
    Graph::from(input)
}

pub fn profile() {
    use std::hint::black_box;
    let input = transform_test_input(&parse_test_input());
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(graph: &Graph) -> usize {
    graph.longest_path::<true>()
}

#[must_use]
pub fn part_2(graph: &Graph) -> usize {
    graph.longest_path::<false>()
}

pub struct Graph {
    nodes: Vec<Node>,
    start_ix: usize,
    goal_ix: usize,
}

impl Graph {
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn longest_path<const SLOPES_ONE_WAY: bool>(&self) -> usize {
        let mut max_dist = 0;

        let mut pending = Vec::with_capacity(self.len());
        pending.push((self.start_ix, 0, 0_u64));

        while let Some((node_ix, dist, visited)) = pending.pop() {
            if node_ix == self.goal_ix {
                max_dist = max_dist.max(dist);
                continue;
            }

            let node_bit = 1_u64 << node_ix;
            if (visited & node_bit) != 0 {
                continue;
            }

            for &edge in &self.nodes[node_ix].neighbors {
                if (visited & (1_u64 << edge.dest_ix)) != 0 {
                    continue;
                }
                if SLOPES_ONE_WAY && !edge.direction.is_outgoing() {
                    continue;
                }

                pending.push((edge.dest_ix, dist + edge.dist, visited | node_bit));
            }
        }
        max_dist
    }
}

impl From<&Map> for Graph {
    fn from(map: &Map) -> Self {
        let mut builder = GraphBuiler::new();
        builder.parse_map(map);
        builder.build()
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph G {{")?;
        for (i, node) in self.nodes.iter().enumerate() {
            write!(f, "n{i}; ")?;
            for e in &node.neighbors {
                if e.dest_ix < i {
                    continue;
                }
                write!(
                    f,
                    "n{i} -> n{} [ dir={} label=\"{}\" ]; ",
                    e.dest_ix,
                    match e.direction {
                        EdgeDirection::Untraversible => "none",
                        EdgeDirection::Incoming => "back",
                        EdgeDirection::Outgoing => "forward",
                        EdgeDirection::TwoWay => "both",
                    },
                    e.dist
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Debug)]
struct GraphBuiler {
    nodes: Vec<Node>,
    node_lookup: HashMap<Pos, usize>,
    start_ix: Option<usize>,
    goal_ix: Option<usize>,
}

impl GraphBuiler {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_lookup: HashMap::new(),
            start_ix: None,
            goal_ix: None,
        }
    }

    pub fn build(self) -> Graph {
        Graph {
            nodes: self.nodes,
            start_ix: self.start_ix.unwrap(),
            goal_ix: self.goal_ix.unwrap(),
        }
    }

    fn add_node(&mut self, pos: Pos) -> usize {
        let node = Node::new();
        let ix = self.nodes.len();
        self.nodes.push(node);
        self.node_lookup.insert(pos, ix);
        ix
    }

    fn connect(
        &mut self,
        source_node_ix: usize,
        dest_node_ix: usize,
        dist: usize,
        edge_type: EdgeDirection,
    ) {
        self.nodes[source_node_ix].connect(dest_node_ix, dist, edge_type);
        self.nodes[dest_node_ix].connect(source_node_ix, dist, edge_type.reverse());
    }

    fn parse_map(&mut self, map: &Map) {
        let start_node_ix = self.add_node(map.start);
        self.start_ix = Some(start_node_ix);

        let mut visited = HashSet::new();

        let mut pending = Vec::new();
        pending.push((start_node_ix, map.start, Dir::S, 0, EdgeDirection::TwoWay));

        while let Some((source_node_ix, pos, dir, dist, edge_type)) = pending.pop() {
            let Some(tile) = map.grid.get(pos) else {
                continue;
            };
            if matches!(tile, Tile::Blocked) {
                continue;
            }

            if !visited.insert((source_node_ix, pos, dir)) {
                continue;
            }

            if let Some(&existing_ix) = self.node_lookup.get(&pos) {
                if pos != map.start {
                    self.connect(source_node_ix, existing_ix, dist, edge_type);
                    continue;
                }
            }

            if pos == map.goal {
                let goal_ix = self.add_node(pos);
                self.goal_ix = Some(goal_ix);
                self.connect(source_node_ix, goal_ix, dist, edge_type);
                continue;
            }

            let neighbors = map.neighbors(pos);
            let num_exists = neighbors
                .iter()
                .filter(|&&(d, _, _)| d != dir.reverse())
                .count();
            match num_exists {
                0 => (),
                1 => {
                    for &(next_dir, next_pos, next_type) in neighbors.iter() {
                        if next_dir == dir.reverse() {
                            continue;
                        }
                        pending.push((
                            source_node_ix,
                            next_pos,
                            next_dir,
                            dist + 1,
                            edge_type & next_type,
                        ));
                    }
                }
                _ => {
                    // Intresection: Create new node
                    let new_node_ix = self.add_node(pos);
                    self.connect(source_node_ix, new_node_ix, dist, edge_type);
                    for &(next_dir, next_pos, next_type) in neighbors.iter() {
                        if next_dir == dir.reverse() {
                            continue;
                        }
                        pending.push((new_node_ix, next_pos, next_dir, 1, next_type));
                    }
                }
            }
        }
    }
}

#[derive(Clone, Default)]
struct Node {
    neighbors: SmallVec<[Edge; 4]>,
}

impl Node {
    fn new() -> Self {
        Self {
            neighbors: SmallVec::new(),
        }
    }

    fn connect(&mut self, other_ix: usize, dist: usize, edge_type: EdgeDirection) {
        let edge = Edge::new(other_ix, dist, edge_type);
        if !self.neighbors.contains(&edge) {
            self.neighbors.push(edge);
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Node").field(&self.neighbors).finish()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Edge {
    dest_ix: usize,
    dist: usize,
    direction: EdgeDirection,
}

impl Edge {
    fn new(dest_ix: usize, dist: usize, edge_type: EdgeDirection) -> Self {
        Self {
            dest_ix,
            dist,
            direction: edge_type,
        }
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.dest_ix, self.dist)
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Open,
    Blocked,
    Slope(Dir),
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "."),
            Self::Blocked => write!(f, "#"),
            Self::Slope(Dir::N) => write!(f, "^"),
            Self::Slope(Dir::E) => write!(f, ">"),
            Self::Slope(Dir::S) => write!(f, "v"),
            Self::Slope(Dir::W) => write!(f, "<"),
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Tile::Open,
            b'#' => Tile::Blocked,
            b'^' => Tile::Slope(Dir::N),
            b'>' => Tile::Slope(Dir::E),
            b'v' => Tile::Slope(Dir::S),
            b'<' => Tile::Slope(Dir::W),
            ch => return Err(ParseInputError::InvalidChar(ch as char)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    grid: Grid<Tile>,
    start: Pos,
    goal: Pos,
}

impl Map {
    fn neighbors(&self, pos: Pos) -> impl Deref<Target = [(Dir, Pos, EdgeDirection)]> {
        match self.grid.get(pos) {
            // Current is not a valid tile
            None | Some(Tile::Blocked) => smallvec![],
            Some(tile) => {
                let mut neighbors: SmallVec<[_; 4]> = SmallVec::new();
                for next_dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
                    let next_pos = pos + next_dir;
                    let cur_along = match tile {
                        // Current is slope in current direction
                        Tile::Slope(dir) => Some(dir == next_dir),
                        _ => None,
                    };

                    let next_against = match self.grid.get(next_pos) {
                        // Neighbor is not a valid tile
                        None | Some(Tile::Blocked) => continue,
                        // Neighbor is slope pointing back to here
                        Some(Tile::Slope(dir)) => Some(dir == next_dir.reverse()),
                        // Path or slope in other direction
                        _ => None,
                    };
                    let edge_type = match (cur_along, next_against) {
                        // Neither is a slope. ..
                        (None, None) => EdgeDirection::TwoWay,
                        // Current is a slope along, and neighbor is not a slope against. >., .> or >>
                        (None | Some(true), None | Some(false)) => EdgeDirection::Outgoing,
                        // Neighbor is a slope against, and current is not a slope along. <., .< or <<
                        (None | Some(false), None | Some(true)) => EdgeDirection::Incoming,
                        // Current and neighbor is either against, of away from, eachother. >< or <>
                        _ => EdgeDirection::Untraversible,
                    };
                    neighbors.push((next_dir, next_pos, edge_type));
                }
                neighbors
            }
        }
    }
}

impl FromStr for Map {
    type Err = ParseInputError;

    #[allow(clippy::cast_possible_wrap)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Tile> = text.parse()?;
        let start = Pos::new(0, 1);
        let goal = Pos::new(grid.height() as isize - 1, grid.width() as isize - 2);
        Ok(Self { grid, start, goal })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeDirection {
    Untraversible = 0b00,
    Incoming = 0b01,
    Outgoing = 0b10,
    TwoWay = 0b11,
}

impl EdgeDirection {
    #[must_use]
    fn is_incoming(self) -> bool {
        matches!(self, Self::Incoming | Self::TwoWay)
    }
    #[must_use]
    fn is_outgoing(self) -> bool {
        matches!(self, Self::Outgoing | Self::TwoWay)
    }

    fn reverse(self) -> Self {
        match self {
            Self::Incoming => Self::Outgoing,
            Self::Outgoing => Self::Incoming,
            e => e,
        }
    }
}

impl BitAnd for EdgeDirection {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let incoming = self.is_incoming() && other.is_incoming();
        let outgoing = self.is_outgoing() && other.is_outgoing();
        match (incoming, outgoing) {
            (false, false) => Self::Untraversible,
            (false, true) => Self::Outgoing,
            (true, false) => Self::Incoming,
            (true, true) => Self::TwoWay,
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("{0:?}")]
    CommonError(#[from] CommonParseError),
}
