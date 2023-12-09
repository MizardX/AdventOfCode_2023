#![warn(clippy::pedantic)]

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 04");

    println!("++Example");
    let example = parse_input(EXAMPLE);
    println!("|+-Part 1: {} (expected 13)", part_1(&example));
    println!("|'-Part 2: {} (expected 30)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT);
    println!("|+-Part 1: {} (expected 23235)", part_1(&input));
    println!("|'-Part 2: {} (expected 5920640)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Card]) -> usize {
    let mut sum = 0;
    for card in input {
        sum += card.score();
    }
    sum
}

fn part_2(input: &[Card]) -> usize {
    let mut counts = vec![1; input.len()];
    let mut sum = 0;
    for (i, c) in input.iter().enumerate() {
        let c_count = counts[i];
        for cnt in &mut counts[i + 1..=i + c.matches()] {
            *cnt += c_count;
        }
        sum += c_count;
    }
    sum
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct Card {
    winning: u128,
    have: u128,
}

impl Card {
    pub fn matches(&self) -> usize {
        (self.winning & self.have).count_ones() as _
    }

    pub fn score(&self) -> usize {
        match self.matches() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

fn parse_input(text: &str) -> Vec<Card> {
    let mut res: Vec<Card> = Vec::with_capacity(256);
    for line in text.lines() {
        let line = line.strip_prefix("Card ").expect("Card prefix");
        let line = line.trim_start();
        let (_, line) = line.split_once(':').expect("':' separator");
        let (winning_str, have_str) = line.split_once(" | ").expect("'|' separator");
        let winning = winning_str
            .split_ascii_whitespace()
            .flat_map(str::parse)
            .fold(0_u128, |s, n: u8| s | (1_u128 << n));
        let have = have_str
            .split_ascii_whitespace()
            .flat_map(str::parse)
            .fold(0_u128, |s, n: u8| s | (1_u128 << n));
        res.push(Card { winning, have });
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
