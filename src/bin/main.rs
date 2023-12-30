use std::env;

pub fn main() {
    let day = env::args().nth(1).and_then(|s| s.parse::<usize>().ok());
    aoc_rust_2023::run(day);
}
