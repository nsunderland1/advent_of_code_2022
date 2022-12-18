use std::time::{Duration, Instant};

use advent_of_code_2022::{get_input, output_file_path, run_day};
use chrono::{Datelike, FixedOffset, Utc};
use clap::Parser;

#[derive(Parser)]
enum Options {
    All,
    Today,
    Day {
        day: u32,
        #[clap(short, long)]
        save: bool,
    },
}

fn current_day_december_2022() -> Option<u32> {
    let eastern_tz = FixedOffset::west_opt(5 * 3600).unwrap(); // Eastern Canada / US
    let date = Utc::now().with_timezone(&eastern_tz).date_naive();
    if date.year() == 2022 && date.month() == 12 && date.day() <= 25 {
        Some(date.day())
    } else {
        None
    }
}

impl Options {
    fn days(&self) -> Vec<u32> {
        let today = current_day_december_2022();
        match self {
            Options::All => (1..=today.unwrap_or(25)).collect(),
            Options::Today => {
                let today =
                    today.expect("This option only works from December 1st through 25th, 2022");
                vec![today]
            }
            Options::Day { day, save: _ } => {
                assert!(*day <= today.unwrap_or(25), "You can't run a future day!");
                vec![*day]
            }
        }
    }
}

fn main() {
    let options = Options::parse();
    let days = options.days();

    let mut total = Duration::ZERO;
    for day in days {
        let input = get_input(day);
        println!("Day {day}");
        let time = Instant::now();
        let (part1, part2) = run_day(day, &input);
        let runtime = time.elapsed();
        println!("Part 1: {part1}");
        println!("Part 2: {part2}");
        println!("Ran in {runtime:?}");
        total += runtime;

        if let Options::Day { save: true, .. } = options {
            let output_file = output_file_path(day);
            std::fs::write(&output_file, format!("{part1} {part2}"))
                .expect("Failed to write to output file");

            println!("Saved results to {}", output_file.display());
        }
    }
    println!("Total time: {total:?}");
}
