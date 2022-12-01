use std::path::Path;

use itertools::Itertools;

#[macro_use]
mod grid;

#[macro_use]
pub mod prelude;

mod day01;

const DAY_TABLE: &[fn(&str) -> (usize, usize)] = &[day01::run];

/// Get the path to the input file for a given day
pub fn get_input(day: u32) -> String {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input_file = crate_root.join("input").join(format!("{day}.input"));
    std::fs::read_to_string(input_file)
        .expect("Could not read from input file")
        .trim()
        .to_owned()
}

/// Run a specific day with the given input as a string
pub fn run_day(day: u32, input: &str) -> (usize, usize) {
    DAY_TABLE[day as usize - 1](input)
}

pub fn get_expected_output(day: u32) -> (usize, usize) {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_file = crate_root.join("output").join(format!("{day}.output"));
    let file_contents =
        std::fs::read_to_string(output_file).expect("Could not read from output file");

    file_contents
        .split(' ')
        .map(|result| result.parse().unwrap())
        .collect_tuple()
        .unwrap()
}
