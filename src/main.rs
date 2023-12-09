#![feature(test)]
#![feature(array_chunks)]

extern crate test;

use std::env;
use std::time::SystemTime;

macro_rules! days {
    ($($val:literal => $mod: ident),* $(,)?) => {
        $(
            mod $mod;
        )*
        fn main() {
            let start = SystemTime::now();
            let day = env::args().nth(1).and_then(|s| s.parse::<usize>().ok());
            $(
                #[allow(clippy::zero_prefixed_literal)]
                if let None | Some($val) = day {
                    $mod::run()
                }
            )*
            let d = SystemTime::now().duration_since(start).unwrap();
            println!("Duration: {}:{:02}:{:02}.{:06}", d.as_secs()/3600, d.as_secs()/60%60, d.as_secs()%60, d.subsec_micros());
        }
    };
}

days! {
    01 => day01,
    02 => day02,
    03 => day03,
    04 => day04,
    05 => day05,
    06 => day06,
    07 => day07,
    08 => day08,
    09 => day09,
    // 10 => day10,
    // 11 => day11,
    // 12 => day12,
    // 13 => day13,
    // 14 => day14,
    // 15 => day15,
    // 16 => day16,
    // 17 => day17,
    // 18 => day18,
    // 19 => day19,
    // 20 => day20,
    // 21 => day21,
    // 22 => day22,
    // 23 => day23,
    // 24 => day24,
    // 25 => day25
}
