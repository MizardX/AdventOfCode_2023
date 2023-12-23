#![warn(clippy::pedantic)]

use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::{CommonErrors, Dir, Grid, Pos};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 23");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 94)", part_1(&example));
    println!("|'-Part 2: {} (expected 154)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 2402)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

fn part_1(map: &Map) -> usize {
    dfs_longest(map, true)
}

fn part_2(map: &Map) -> usize {
    dfs_longest(map, false)
}

#[allow(clippy::cast_possible_wrap)]
fn dfs_longest(map: &Map, paths_one_way: bool) -> usize {
    let start = Pos::new(0, 1);
    let goal = Pos::new(
        map.grid.height() as isize - 1,
        map.grid.width() as isize - 2,
    );
    let mut visited = Grid::new(map.grid.width(), map.grid.height());
    let mut max_dist = 0;

    let mut pending = Vec::with_capacity(map.grid.width() * map.grid.height());
    pending.push((start, 0, true));
    while let Some((pos, dist, entering)) = pending.pop() {
        let Some(tile) = map.grid.get(pos) else {
            continue;
        };
        if matches!(tile, Tile::Blocked) {
            continue;
        }
        if entering {
            if pos == goal {
                max_dist = max_dist.max(dist);
                continue;
            }
            match visited.get_mut(pos).unwrap() {
                true => {
                    continue;
                }
                vis => *vis = true,
            };
            pending.push((pos, dist, false)); // exiting
            for step in [Dir::N, Dir::E, Dir::S, Dir::W] {
                if paths_one_way && matches!(tile, Tile::Slope(d) if d != step) {
                    continue;
                }
                let next = pos + step;
                if matches!(map.grid.get(next), Some(Tile::Blocked)) {
                    continue;
                }
                if matches!(visited.get(next), Some(true) | None) {
                    continue;
                }
                pending.push((next, dist + 1, true));
            }
        } else {
            // exiting
            match visited.get_mut(pos).unwrap() {
                false => unreachable!("Should never be unvisited when exiting"),
                vis => *vis = false,
            };
        }
    }
    max_dist
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Blocked,
    Slope(Dir),
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
struct Map {
    grid: Grid<Tile>,
}

impl FromStr for Map {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: text.parse()?,
        })
    }
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl CommonErrors for ParseInputError {
    fn empty_input() -> Self {
        Self::EmptyInput
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(INPUT.parse::<Map>().expect("Parse input")));
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
