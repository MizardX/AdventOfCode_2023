#![warn(clippy::pedantic)]

use std::str::FromStr;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 02");

    println!("++Example");
    let example = parse_input(EXAMPLE);
    println!("|+-Part 1: {} (expected 8)", part_1(&example));
    println!("|'-Part 2: {} (expected 2286)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT);
    println!("|+-Part 1: {} (expected 2176)", part_1(&input));
    println!("|'-Part 2: {} (expected 63700)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Input]) -> usize {
    let mut sum = 0;
    'outer: for game in input {
        for &piece in &game.rounds {
            if !piece.is_possible() {
                continue 'outer;
            }
        }
        sum += game.id;
    }
    sum
}

#[allow(unused)]
fn part_2(input: &[Input]) -> usize {
    let mut sum = 0;
    for game in input {
        let mut target = Round::new();
        for &piece in &game.rounds {
            target.max_assign(piece);
        }
        sum += target.power();
    }
    sum
}

#[derive(Debug, Clone)]
struct Input {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn is_possible(self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    pub fn max_assign(&mut self, other: Self) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }

    pub fn power(self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(piece: &str) -> Result<Self, Self::Err> {
        let mut res = Round::new();
        for cube in piece.split(", ") {
            let (num_str, color_str) = cube.split_once(' ').expect("Space separator");
            let num = num_str.parse::<usize>().expect("Integer");
            match color_str.as_bytes()[0] {
                b'r' => res.red += num,
                b'g' => res.green += num,
                b'b' => res.blue += num,
                _ => (),
            };
        }
        Ok(res)
    }
}

fn parse_input(text: &str) -> Vec<Input> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
        let line = &line[5..]; //.strip_prefix("Game ").expect("game prefix");
        let (id_str, line) = line.split_once(": ").expect("colon separator");
        let id = id_str.parse::<usize>().expect("numeric game id");
        let mut rounds = Vec::with_capacity(10);
        for round_str in line.split("; ") {
            let round = round_str.parse().expect("round");
            rounds.push(round);
        }
        res.push(Input { id, rounds });
    }
    res
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;


    #[bench]
    fn run_parse_input(b: &mut Bencher) {
        b.iter(|| black_box(parse_input(INPUT)));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| black_box(part_2(&input)));
    }
}
