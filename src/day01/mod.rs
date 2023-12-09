#![warn(clippy::pedantic)]

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 01");

    println!("++Example");
    let example1 = parse_input(EXAMPLE1);
    println!("|+-Part 1: {} (expected 142)", part_1(&example1));
    let example2 = parse_input(EXAMPLE2);
    println!("|'-Part 2: {} (expected 281)", part_2(&example2));

    println!("++Input");
    let input = parse_input(INPUT);
    println!("|+-Part 1: {} (expected 54927)", part_1(&input));
    println!("|'-Part 2: {} (expected 54581)", part_2(&input));
    println!("')");
}

#[allow(unused)]
fn part_1(input: &[Input]) -> usize {
    let mut sum = 0;
    for item in input {
        let first = item.line.bytes().find(u8::is_ascii_digit);
        let last = item.line.bytes().rev().find(u8::is_ascii_digit);
        if let (Some(first), Some(last)) = (first, last) {
            sum += 10 * (first - b'0') as usize + (last - b'0') as usize;
        }
    }
    sum
}

#[derive(Debug, Copy, Clone)]
enum State {
    Start,
    O,
    On, // + N
    // ONE 1 E
    T,
    Tw,
    // TWO 2 O
    Th,
    Thr,
    Thre, // E
    // THREE 3 E
    F,
    Fo, // O
    Fou,
    // FOUR 4 -
    Fi,
    Fiv,
    // FIVE 5 E
    S,
    Si,
    // SIX 6 -
    Se, // E
    Sev,
    Seve, // E
    // SEVEN 7 N
    E,
    Ei,
    Eig,
    Eigh,
    // EIGHT 8 T
    N,
    Ni,
    Nin, // N
         // NINE 9 E
}

#[derive(Debug, Copy, Clone)]
enum StateRev {
    Start,
    E,
    Ee,  // E
    Eer, // R
    Eerh,
    // EERHT 3 T

    //E
    En, // N
    Eni,
    // ENIN 9 N

    //E
    // EN
    // ENO 1 O

    //E
    Ev,
    Evi,
    // EVIF 5 -
    N,
    Ne,  // E
    Nev, // EV
    Neve,
    // NEVES 7 -
    O,
    Ow,
    // OWT 2 T
    R,
    Ru,
    Ruo, // O
    // RUOF 4 -
    T,
    Th,
    Thg,
    Thgi,
    // THGIE 8 E
    X,
    Xi,
    // XIS 6 -
}

#[allow(unused)]
fn part_2(input: &[Input]) -> usize {
    let mut sum = 0;
    for item in input {
        let mut line = item.line;
        let first = match_forward(line);
        let last = match_backward(line);
        if let (Some(first), Some(last)) = (first, last) {
            // println!("{line} -> {}", 10*first + last);
            sum += (10 * first + last) as usize;
        }
    }
    sum
}

fn match_forward(line: &str) -> Option<u8> {
    let mut state = State::Start;
    for &ch in line.as_bytes() {
        state = match (state, ch) {
            (_, b @ b'0'..=b'9') => return Some(b - b'0'),
            (State::O | State::Fo, b'n') => State::On,
            (State::On, b'e') => return Some(1),
            (State::T, b'w') => State::Tw,
            (State::T, b'h') => State::Th,
            (State::Tw, b'o') => return Some(2),
            (State::Th, b'r') => State::Thr,
            (State::Thr, b'e') => State::Thre,
            (State::Thre, b'e') => return Some(3),
            (State::F, b'o') => State::Fo,
            (State::F, b'i') => State::Fi,
            (State::Fo, b'u') => State::Fou,
            (State::Fou, b'r') => return Some(4),
            (State::Fi, b'v') => State::Fiv,
            (State::Fiv, b'e') => return Some(5),
            (State::S, b'i') => State::Si,
            (State::S, b'e') => State::Se,
            (State::Si, b'x') => return Some(6),
            (State::Se, b'v') => State::Sev,
            (State::Sev, b'e') => State::Seve,
            (State::Seve, b'n') => return Some(7),
            (State::E | State::Thre | State::Se | State::Seve, b'i') => State::Ei,
            (State::Ei, b'g') => State::Eig,
            (State::Eig, b'h') => State::Eigh,
            (State::Eigh, b't') => return Some(8),
            (State::N | State::On | State::Nin, b'i') => State::Ni,
            (State::Ni, b'n') => State::Nin,
            (State::Nin, b'e') => return Some(9),
            (_, b'o') => State::O,
            (_, b't') => State::T,
            (_, b'f') => State::F,
            (_, b's') => State::S,
            (_, b'e') => State::E,
            (_, b'n') => State::N,
            (_, _) => State::Start,
        };
    }
    None
}

fn match_backward(line: &str) -> Option<u8> {
    //println!("LINE: {line}");
    let mut state = StateRev::Start;
    for ch in line.bytes().rev() {
        //println!("  {state:?} + {}", ch as char);
        state = match (state, ch) {
            (_, b @ b'0'..=b'9') => return Some(b - b'0'),
            (StateRev::E | StateRev::Ee | StateRev::Ne | StateRev::Neve, b'e') => StateRev::Ee,
            (StateRev::Ee, b'r') => StateRev::Eer,
            (StateRev::Eer, b'h') => StateRev::Eerh,
            (StateRev::Eerh, b't') => return Some(3),
            (StateRev::E | StateRev::Ee | StateRev::Ne | StateRev::Neve, b'n') => StateRev::En,
            (StateRev::En, b'i') => StateRev::Eni,
            (StateRev::En, b'o') => return Some(1),
            (StateRev::Eni, b'n') => return Some(9),
            (StateRev::E | StateRev::Ee | StateRev::Neve, b'v') => StateRev::Ev,
            (StateRev::Ev | StateRev::Nev, b'i') => StateRev::Evi,
            (StateRev::Evi, b'f') => return Some(5),
            (StateRev::N | StateRev::En, b'e') => StateRev::Ne,
            (StateRev::Ne, b'v') => StateRev::Nev,
            (StateRev::Nev, b'e') => StateRev::Neve,
            (StateRev::Neve, b's') => return Some(7),
            (StateRev::O | StateRev::Ruo, b'w') => StateRev::Ow,
            (StateRev::Ow, b't') => return Some(2),
            (StateRev::R | StateRev::Eer, b'u') => StateRev::Ru,
            (StateRev::Ru, b'o') => StateRev::Ruo,
            (StateRev::Ruo, b'f') => return Some(4),
            (StateRev::T, b'h') => StateRev::Th,
            (StateRev::Th, b'g') => StateRev::Thg,
            (StateRev::Thg, b'i') => StateRev::Thgi,
            (StateRev::Thgi, b'e') => return Some(8),
            (StateRev::X, b'i') => StateRev::Xi,
            (StateRev::Xi, b's') => return Some(6),
            (_, b'e') => StateRev::E,
            (_, b'n') => StateRev::N,
            (_, b'o') => StateRev::O,
            (_, b'r') => StateRev::R,
            (_, b't') => StateRev::T,
            (_, b'x') => StateRev::X,
            _ => StateRev::Start,
        }
    }
    None
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    #[bench]
    fn parsing(b: &mut Bencher) {
        b.iter(|| parse_input(INPUT));
    }

    #[bench]
    fn run_part_1(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn run_part_2(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| part_2(&input));
    }
}
