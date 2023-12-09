#![warn(clippy::pedantic)]

pub fn run() {
    println!(".Day 04");

    println!("++Example");
    let example = parse_input(include_str!("example.txt"));
    println!("|+-Part 1: {} (expected 13)", part_1(&example));
    println!("|'-Part 2: {} (expected 30)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt"));
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
    let mut res: Vec<Card> = Vec::new();
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