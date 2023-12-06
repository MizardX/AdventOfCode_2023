#![warn(clippy::pedantic)]

use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub fn run() {
    println!(".Day 05");

    println!("++Example");
    let example = parse_input(include_str!("example.txt")).expect("Example input parsed");
    println!("|+-Part 1: {} (expected 35)", part_1(&example));
    println!("|'-Part 2: {} (expected 46)", part_2(&example));

    println!("++Input");
    let input = parse_input(include_str!("input.txt")).expect("Real input parsed");
    println!("|+-Part 1: {} (expected 174137457)", part_1(&input));
    println!("|'-Part 2: {} (expected 1493866)", part_2(&input));

    println!("')");
}

#[allow(unused)]
fn part_1(input: &Input) -> isize {
    let mut min = isize::MAX;
    for &(mut seed) in &input.seeds {
        for mapping in [
            &input.seed_to_soil,
            &input.soil_to_fertilizer,
            &input.fertilizer_to_water,
            &input.water_to_light,
            &input.light_to_temperature,
            &input.temperature_to_humidity,
            &input.humidity_to_location,
        ] {
            seed = mapping
                .iter()
                .find_map(|r| (r.start <= seed && seed < r.end).then_some(seed + r.delta))
                .unwrap_or(seed);
        }
        min = min.min(seed);
    }
    min
}

#[allow(unused)]
fn part_2(input: &Input) -> isize {
    let mut min = isize::MAX;
    for pair in input.seeds.array_chunks::<2>() {
        for mut seed in pair[0]..pair[0] + pair[1] {
            for mapping in [
                &input.seed_to_soil,
                &input.soil_to_fertilizer,
                &input.fertilizer_to_water,
                &input.water_to_light,
                &input.light_to_temperature,
                &input.temperature_to_humidity,
                &input.humidity_to_location,
            ] {
                seed = mapping
                    .iter()
                    .find_map(|r| (r.start <= seed && seed < r.end).then_some(seed + r.delta))
                    .unwrap_or(seed);
            }
            min = min.min(seed);
        }
    }
    min
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    start: isize,
    end: isize,
    delta: isize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range(isize, isize);

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{} ({})",
            numfmt(self.0),
            numfmt(self.1),
            numfmt(self.1 - self.0)
        )
    }
}

impl Mapping {
    pub fn new(start: isize, end: isize, delta: isize) -> Self {
        Self { start, end, delta }
    }
}

impl Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}..{} -> {}..{} ({:+})>",
            numfmt(self.start),
            numfmt(self.end),
            numfmt(self.start + self.delta),
            numfmt(self.end + self.delta),
            self.delta
        )
    }
}

fn numfmt(x: isize) -> String {
    x.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(" ")
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("First line should start with 'Seeds:'")]
    SeedSuffix,
    #[error("The first number, start, is missing.")]
    MissingSource,
    #[error("The second number, destination_start, is missing.")]
    MissingDestination,
    #[error("The third number, len, is missing.")]
    MissingLen,
    #[error("One of the numbers could not be parsed as an integer: {0}")]
    NotInteger(#[from] ParseIntError),
    #[error("The line contains more values than expected")]
    ExtraneousValues,
    #[error("Expected more headers")]
    Incomplete,
}

impl FromStr for Mapping {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let destination_start: isize = it.next().ok_or(ParseError::MissingDestination)?.parse()?;
        let source_start: isize = it.next().ok_or(ParseError::MissingSource)?.parse()?;
        let len: isize = it.next().ok_or(ParseError::MissingLen)?.parse()?;
        if it.next().is_some() {
            return Err(ParseError::ExtraneousValues);
        }
        Ok(Self::new(
            source_start,
            source_start + len,
            destination_start - source_start,
        ))
    }
}

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<isize>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[allow(unused)]
fn parse_input(text: &str) -> Result<Input, ParseError> {
    #[derive(Debug, Copy, Clone)]
    enum State {
        Seeds,
        SeedToSoil,
        SoilToFertilizer,
        FertilizerToWater,
        WaterToLight,
        LightToTemperature,
        TemperatureToHumidity,
        HumidityToLocation,
    }

    let mut seeds: Vec<isize> = Vec::new();
    let mut seed_to_soil: Vec<Mapping> = Vec::new();
    let mut soil_to_fertilizer: Vec<Mapping> = Vec::new();
    let mut fertilizer_to_water: Vec<Mapping> = Vec::new();
    let mut water_to_light: Vec<Mapping> = Vec::new();
    let mut light_to_temperature: Vec<Mapping> = Vec::new();
    let mut temperature_to_humidity: Vec<Mapping> = Vec::new();
    let mut humidity_to_location: Vec<Mapping> = Vec::new();

    let mut state = State::Seeds;
    for line in text.lines() {
        match (state, line) {
            (_, "") => (),
            (State::Seeds, "seed-to-soil map:") => state = State::SeedToSoil,
            (State::SeedToSoil, "soil-to-fertilizer map:") => state = State::SoilToFertilizer,
            (State::SoilToFertilizer, "fertilizer-to-water map:") => {
                state = State::FertilizerToWater;
            }
            (State::FertilizerToWater, "water-to-light map:") => state = State::WaterToLight,
            (State::WaterToLight, "light-to-temperature map:") => state = State::LightToTemperature,
            (State::LightToTemperature, "temperature-to-humidity map:") => {
                state = State::TemperatureToHumidity;
            }
            (State::TemperatureToHumidity, "humidity-to-location map:") => {
                state = State::HumidityToLocation;
            }
            (State::Seeds, line) => {
                seeds = line
                    .strip_prefix("seeds: ")
                    .ok_or(ParseError::SeedSuffix)?
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?;
            }
            (State::SeedToSoil, line) => seed_to_soil.push(Mapping::from_str(line)?),
            (State::SoilToFertilizer, line) => soil_to_fertilizer.push(Mapping::from_str(line)?),
            (State::FertilizerToWater, line) => fertilizer_to_water.push(Mapping::from_str(line)?),
            (State::WaterToLight, line) => water_to_light.push(Mapping::from_str(line)?),
            (State::LightToTemperature, line) => {
                light_to_temperature.push(Mapping::from_str(line)?);
            }
            (State::TemperatureToHumidity, line) => {
                temperature_to_humidity.push(Mapping::from_str(line)?);
            }
            (State::HumidityToLocation, line) => {
                humidity_to_location.push(Mapping::from_str(line)?);
            }
        }
    }
    if let State::HumidityToLocation = &state {
        seed_to_soil.sort_unstable();
        soil_to_fertilizer.sort_unstable();
        fertilizer_to_water.sort_unstable();
        water_to_light.sort_unstable();
        light_to_temperature.sort_unstable();
        temperature_to_humidity.sort_unstable();
        humidity_to_location.sort_unstable();
        Ok(Input {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    } else {
        Err(ParseError::Incomplete)
    }
}
