use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use itertools::Itertools;
use nom::{combinator::map, IResult};

#[macro_use]
mod grid;

#[macro_use]
pub mod prelude;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

const DAY_TABLE: &[fn(&str) -> (Solution, Solution)] = &[
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
    day08::run,
    day09::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    day15::run,
];

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Int(usize),
    String(String),
}

impl From<usize> for Solution {
    fn from(num: usize) -> Self {
        Self::Int(num)
    }
}

impl From<String> for Solution {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<&str> for Solution {
    fn from(string: &str) -> Self {
        Self::String(string.into())
    }
}

impl FromStr for Solution {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<usize>()
            .map(Self::from)
            .unwrap_or_else(|_| Self::from(s)))
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(num) => write!(f, "{num}"),
            Self::String(string) => write!(f, "{string}"),
        }
    }
}

/// Get the path to the input file for a given day
pub fn get_input(day: u32) -> String {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input_file = crate_root.join("input").join(format!("{day}.input"));
    std::fs::read_to_string(input_file)
        .expect("Could not read from input file")
        .to_owned()
}

/// Run a specific day with the given input as a string
pub fn run_day(day: u32, input: &str) -> (Solution, Solution) {
    DAY_TABLE[day as usize - 1](input)
}

pub fn output_file_path(day: u32) -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("output").join(format!("{day}.output"))
}

pub fn get_expected_output(day: u32) -> (Solution, Solution) {
    let output_file = output_file_path(day);
    let file_contents =
        std::fs::read_to_string(output_file).expect("Could not read from output file");

    file_contents
        .split(' ')
        .map(|result| result.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |n| n as usize)(input)
}
