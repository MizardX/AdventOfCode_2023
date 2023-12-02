use std::ops::AddAssign;

pub fn run() {
    println!(".Day 02");

    println!("++Example");
    let example = parse_input(include_str!("example.txt"));
    println!("|+-Part 1: {} (expected 8)", part_1(&example));
    println!("|'-Part 2: {} (expected 2286)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt"));
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

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl AddAssign<(usize, Color)> for Round {
    fn add_assign(&mut self, rhs: (usize, Color)) {
        match rhs.1 {
            Color::Red => self.red += rhs.0,
            Color::Green => self.green += rhs.0,
            Color::Blue => self.blue += rhs.0,
        }
    }
}

fn parse_input(text: &str) -> Vec<Input> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
        let line = line.strip_prefix("Game ").expect("game prefix");
        let (id_str, line) = line.split_once(": ").expect("colon separator");
        let id = id_str.parse::<usize>().expect("numeric game id");
        let mut rounds = Vec::new();
        for round_str in line.split("; ") {
            let round = parse_round(round_str).expect("round");
            rounds.push(round);
        }
        res.push(Input { id, rounds });
    }
    res
}

fn parse_round(piece: &str) -> Option<Round> {
    let mut res = Round::new();
    for cube in piece.split(", ") {
        res += parse_cube(cube)?;
    }
    Some(res)
}

fn parse_cube(component: &str) -> Option<(usize, Color)> {
    let (num_str, color_str) = component.split_once(' ')?;
    let num = num_str.parse::<usize>().ok()?;
    let color = match color_str.as_bytes()[0] {
        b'r' => Color::Red,
        b'g' => Color::Green,
        b'b' => Color::Blue,
        _ => return None,
    };
    Some((num, color))
}
