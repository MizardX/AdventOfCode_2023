#![warn(clippy::pedantic)]

pub fn run() {
    println!(".Day XX");

    println!("++Example");
    let example = parse_input(include_str!("example.txt"));
    println!("|+-Part 1: {} (expected XXX)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt"));
    println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Input]) -> usize {
    0
}

#[allow(unused)]
fn part_2(input: &[Input]) -> usize {
    0
}

#[derive(Debug, Clone)]
struct Input(i32);

fn parse_input(text: &str) -> Vec<Input> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
        res.push(Input(line.trim().parse::<i32>().unwrap()));
    }
    res
}
