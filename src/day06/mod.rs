#![warn(clippy::pedantic)]

use std::num::ParseIntError;

pub fn run() {
    println!(".Day 06");

    println!("++Example");
    let example = parse_input(include_str!("example.txt"));
    println!("|+-Part 1: {} (expected 288)", part_1(&example));
    println!("|'-Part 2: {} (expected 71503)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt"));
    println!("|+-Part 1: {} (expected 4403592)", part_1(&input));
    println!("|'-Part 2: {} (expected 38017587)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &Input) -> i64 {
    let mut product = 1;
    for race in &input.races {
        product *= race.score();
    }
    product
}

#[allow(unused)]
fn part_2(input: &Input) -> i64 {
    let mut combined = Race::default();
    let scales = [1, 10, 100, 1_000, 10_000];
    for race in &input.races {
        combined.time = combined.time * scales[race.time_len] + race.time;
        combined.distance = combined.distance * scales[race.distance_len] + race.distance;
    }
    combined.score()
}

#[derive(Debug, Clone)]
struct Input {
    races: Vec<Race>,
}

#[derive(Debug, Clone, Default)]
struct Race {
    time: i64,
    time_len: usize,
    distance: i64,
    distance_len: usize,
}

impl Race {
    pub fn from_str(time_str: &str, distance_str: &str) -> Result<Race, ParseIntError> {
        Ok(Self {
            time: time_str.parse()?,
            time_len: time_str.len(),
            distance: distance_str.parse()?,
            distance_len: distance_str.len(),
        })
    }

    pub fn score(&self) -> i64 {
        #[allow(clippy::cast_possible_truncation,clippy::cast_precision_loss)]
        let s = ((self.time * self.time - 4 * self.distance - 4) as f64).sqrt() as i64;
        let low = (self.time - s + 1) / 2;
        let high = (self.time + s) / 2;
        high - low + 1
    }
}

fn parse_input(text: &str) -> Input {
    let mut lines = text.lines();

    let time_it = lines
        .next()
        .expect("two lines")
        .strip_prefix("Time:")
        .expect("time prefix")
        .split_ascii_whitespace();

    let distance_it = lines
        .next()
        .expect("two lines")
        .strip_prefix("Distance:")
        .expect("distance prefix")
        .split_ascii_whitespace();

    let races = time_it
        .zip(distance_it)
        .map(|(t, d)| Race::from_str(t, d).expect("integers"))
        .collect::<Vec<_>>();
    
    Input { races }
}
