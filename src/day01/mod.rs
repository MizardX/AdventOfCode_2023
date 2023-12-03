#![warn(clippy::pedantic)]

pub fn run() {
    println!(".Day 01");

    println!("++Example");
    let example1 = parse_input(include_str!("example1.txt"));
    println!("|+-Part 1: {} (expected 142)", part_1(&example1));
    let example2 = parse_input(include_str!("example2.txt"));
    println!("|'-Part 2: {} (expected 281)", part_2(&example2));

    println!("++Input");
    let input = parse_input(include_str!("input.txt"));
    println!("|+-Part 1: {} (expected 54927)", part_1(&input));
    println!("|'-Part 2: {} (expected 54581)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Input]) -> usize {
    let mut sum = 0;
    for item in input {
        let mut first = None;
        let mut last = None;
        for &ch in item.line.as_bytes() {
            if ch.is_ascii_digit() {
                if first.is_none() {
                    first = Some(ch);
                }
                last = Some(ch);
            }
        }
        if let (Some(first), Some(last)) = (first, last) {
            sum += 10 * (first - b'0') as usize + (last - b'0') as usize;
        }
    }
    sum
}

#[derive(Debug,Copy,Clone)]
enum State {
    Start,
    O,
    On, // + N
    // ONE + E
    T,
    Tw,
    // TWO + O
    Th,
    Thr,
    Thre, // + E
    // THREE + E
    F,
    Fo, // + O
    Fou,
    // FOUR,
    Fi,
    Fiv,
    // FIVE + E
    S,
    Si,
    // SIX
    Se, // + E
    Sev,
    Seve, // + E
    // SEVEN + N
    E,
    Ei,
    Eig,
    Eigh,
    // EIGHT + T
    N,
    Ni,
    Nin, // + N
         // NINE + E
}

#[allow(unused)]
fn part_2(input: &[Input]) -> usize {
    let mut sum = 0;
    for item in input {
        let mut first = None;
        let mut last = None;
        let mut line = item.line;
        // println!("LINE: {line}");
        let mut state = State::Start;
        for &ch in line.as_bytes() {
            let (dig, state1) = match (state, ch) {
                (_, b @ b'0'..=b'9') => (Some(b - b'0'), State::Start),
                (State::O | State::Fo, b'n') => (None, State::On),
                (State::On, b'e') => (Some(1), State::E),
                (State::On | State::N | State::Nin, b'i') => (None, State::Ni),
                (State::T, b'w') => (None, State::Tw),
                (State::Tw, b'o') => (Some(2), State::O),
                (State::T, b'h') => (None, State::Th),
                (State::Th, b'r') => (None, State::Thr),
                (State::Thr, b'e') => (None, State::Thre),
                (State::Thre, b'e') => (Some(3), State::E),
                (State::Thre | State::Se | State::Seve | State::E, b'i') => (None, State::Ei),
                (State::F, b'o') => (None, State::Fo),
                (State::Fo, b'u') => (None, State::Fou),
                (State::Fou, b'r') => (Some(4), State::Start),
                (State::F, b'i') => (None, State::Fi),
                (State::Fi, b'v') => (None, State::Fiv),
                (State::Fiv, b'e') => (Some(5), State::E),
                (State::S, b'i') => (None, State::Si),
                (State::Si, b'x') => (Some(6), State::Start),
                (State::S, b'e') => (None, State::Se),
                (State::Se, b'v') => (None, State::Sev),
                (State::Sev, b'e') => (None, State::Seve),
                (State::Seve, b'n') => (Some(7), State::N),
                (State::Ei, b'g') => (None, State::Eig),
                (State::Eig, b'h') => (None, State::Eigh),
                (State::Eigh, b't') => (Some(8), State::T),
                (State::Ni, b'n') => (None, State::Nin),
                (State::Nin, b'e') => (Some(9), State::E),
                (_, b'o') => (None, State::O),
                (_, b't') => (None, State::T),
                (_, b'f') => (None, State::F),
                (_, b's') => (None, State::S),
                (_, b'e') => (None, State::E),
                (_, b'n') => (None, State::N),                
                (_, _) => (None, State::Start),
            };
            // println!("  {state:?} + {} -> {dig:?} + {state1:?}", ch as char);
            state = state1;
            if let Some(dig) = dig {
                if first.is_none() {
                    first = Some(dig);
                }
                last = Some(dig);
            }
        }
        if let (Some(first), Some(last)) = (first, last) {
            // println!("{line} -> {}", 10*first + last);
            sum += (10 * first + last) as usize;
        }
    }
    sum
}

#[derive(Debug, Clone)]
struct Input<'a> {
    line: &'a str,
}

fn parse_input(text: &str) -> Vec<Input> {
    let mut res: Vec<Input> = Vec::new();
    for line in text.lines() {
        res.push(Input { line });
    }
    res
}
