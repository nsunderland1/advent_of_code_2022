use std::path::{Path, PathBuf};

#[macro_use]
mod grid;

pub mod prelude;

mod day01;

const DAY_TABLE: &[fn(&str)] = &[day01::run];

/// Get the path to the input file for a given day
pub fn get_input_file(day: u32) -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input").join(format!("{}.input", day))
}

/// Run a specific day with the given input as a string
pub fn run_day(day: u32, input: &str) {
    DAY_TABLE[day as usize - 1](input)
}
