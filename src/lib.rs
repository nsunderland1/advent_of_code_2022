use std::path::{Path, PathBuf};

use itertools::Itertools;

#[macro_use]
mod grid;

#[macro_use]
pub mod prelude;

mod day01;
mod day02;
mod day03;
mod day04;

const DAY_TABLE: &[fn(&str) -> (usize, usize)] = &[day01::run, day02::run, day03::run, day04::run];

/// Get the path to the input file for a given day
pub fn get_input(day: u32) -> String {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input_file = crate_root.join("input").join(format!("{day}.input"));
    std::fs::read_to_string(input_file)
        .expect("Could not read from input file")
        .to_owned()
}

/// Run a specific day with the given input as a string
pub fn run_day(day: u32, input: &str) -> (usize, usize) {
    DAY_TABLE[day as usize - 1](input)
}

pub fn output_file_path(day: u32) -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("output").join(format!("{day}.output"))
}

pub fn get_expected_output(day: u32) -> (usize, usize) {
    let output_file = output_file_path(day);
    let file_contents =
        std::fs::read_to_string(output_file).expect("Could not read from output file");

    file_contents
        .split(' ')
        .map(|result| result.parse().unwrap())
        .collect_tuple()
        .unwrap()
}
