use std::{
    fs,
    time::{Duration, Instant},
};

use advent_of_code_2022::{get_input_file, run_day};
use chrono::{Datelike, FixedOffset, Utc};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Options {
    All,
    Today,
    Day { day: u32 },
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
            Options::Day { day } => {
                assert!(*day <= today.unwrap_or(25), "You can't run a future day!");
                vec![*day]
            }
        }
    }
}

fn main() {
    let options = Options::from_args();
    let days = options.days();

    let mut total = Duration::ZERO;
    for day in days {
        let input = fs::read_to_string(get_input_file(day)).expect("Failed to read input file");
        let input = input.trim();
        println!("Day {}", day);
        let time = Instant::now();
        run_day(day, &input);
        let runtime = time.elapsed();
        println!("Ran in {:?}", runtime);
        total += runtime;
    }
    println!("Total time: {:?}", total);
}
