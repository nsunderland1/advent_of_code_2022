use std::str::FromStr;

#[allow(unused)]
use crate::prelude::*;

#[derive(Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    fn score(them: Self, us: Self) -> usize {
        (match (them, us) {
            (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper)
            | (Self::Paper, Self::Rock) => 0,
            (Self::Scissors, Self::Rock)
            | (Self::Paper, Self::Scissors)
            | (Self::Rock, Self::Paper) => 6,
            _ => 3,
        }) + (us as usize)
    }

    fn lose(self) -> Self {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }

    fn win(self) -> Self {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    fn draw(self) -> Self {
        self
    }
}

impl FromStr for Play {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => return Err("Nope".to_string()),
        })
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let plays: Vec<(Play, Play)> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|play| play.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let result1 = {
        plays
            .iter()
            .copied()
            .map(|(them, us)| Play::score(them, us))
            .sum()
    };

    let result2 = {
        plays
            .into_iter()
            .map(|(them, result)| match result {
                Play::Rock => Play::score(them, Play::lose(them)),
                Play::Paper => Play::score(them, Play::draw(them)),
                Play::Scissors => Play::score(them, Play::win(them)),
            })
            .sum()
    };

    (result1, result2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 2;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
