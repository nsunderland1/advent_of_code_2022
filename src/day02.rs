#[allow(unused)]
use crate::prelude::*;
use crate::Solution;

const fn index(string: &[u8]) -> usize {
    (((string[0] - b'A') * 3) + (string[2] - b'X')) as usize
}

const fn value(play: u8) -> usize {
    match play {
        b'X' => 1,
        b'Y' => 2,
        b'Z' => 3,
        _ => panic!("Unknown play"),
    }
}

const fn score_round(them: u8, us: u8) -> usize {
    (match (them, us) {
        (b'A', b'Z') | (b'C', b'Y') | (b'B', b'X') => 0,
        (b'C', b'X') | (b'B', b'Z') | (b'A', b'Y') => 6,
        _ => 3,
    }) + value(us)
}

const fn part1_score(round: &[u8]) -> usize {
    let them = round[0];
    let us = round[2];
    score_round(them, us)
}

const fn lose(them: u8) -> u8 {
    match them {
        b'A' => b'Z',
        b'B' => b'X',
        b'C' => b'Y',
        _ => panic!("Invalid play"),
    }
}

const fn win(them: u8) -> u8 {
    match them {
        b'A' => b'Y',
        b'B' => b'Z',
        b'C' => b'X',
        _ => panic!("Invalid play"),
    }
}

const fn draw(them: u8) -> u8 {
    match them {
        b'A' => b'X',
        b'B' => b'Y',
        b'C' => b'Z',
        _ => panic!("Invalid play"),
    }
}

const fn part2_score(round: &[u8]) -> usize {
    let them = round[0];
    let us = round[2];

    score_round(
        them,
        match us {
            b'X' => lose(them),
            b'Y' => draw(them),
            b'Z' => win(them),
            _ => panic!("Invalid play"),
        },
    )
}

pub fn run(input: &str) -> (Solution, Solution) {
    const LOOKUP_TABLE: [(usize, usize); 9] = {
        let cases = [
            b"A X", b"A Y", b"A Z", b"B X", b"B Y", b"B Z", b"C X", b"C Y", b"C Z",
        ];

        let mut scores = [(0, 0); 9];

        let mut i = 0;

        loop {
            if i >= cases.len() {
                break;
            };
            assert!(i == index(cases[i]));

            scores[i].0 = part1_score(cases[i]);
            scores[i].1 = part2_score(cases[i]);

            i += 1;
        }

        scores
    };

    let mut part1 = 0;
    let mut part2 = 0;

    let input = input.as_bytes();
    for line in input.chunks(4) {
        let (result1, result2) = LOOKUP_TABLE[index(line)];
        part1 += result1;
        part2 += result2;
    }

    (part1.into(), part2.into())
}

// #[derive(Clone, Copy)]
// enum Play {
//     Rock = 1,
//     Paper = 2,
//     Scissors = 3,
// }

// impl Play {
//     fn score(them: Self, us: Self) -> usize {
//         (match (them, us) {
//             (Self::Rock, Self::Scissors)
//             | (Self::Scissors, Self::Paper)
//             | (Self::Paper, Self::Rock) => 0,
//             (Self::Scissors, Self::Rock)
//             | (Self::Paper, Self::Scissors)
//             | (Self::Rock, Self::Paper) => 6,
//             _ => 3,
//         }) + (us as usize)
//     }

//     fn lose(self) -> Self {
//         match self {
//             Play::Rock => Play::Scissors,
//             Play::Paper => Play::Rock,
//             Play::Scissors => Play::Paper,
//         }
//     }

//     fn win(self) -> Self {
//         match self {
//             Play::Rock => Play::Paper,
//             Play::Paper => Play::Scissors,
//             Play::Scissors => Play::Rock,
//         }
//     }

//     fn draw(self) -> Self {
//         self
//     }
// }

// impl FromStr for Play {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(match s {
//             "A" | "X" => Self::Rock,
//             "B" | "Y" => Self::Paper,
//             "C" | "Z" => Self::Scissors,
//             _ => return Err("Nope".to_string()),
//         })
//     }
// }

// pub fn run(input: &str) -> (usize, usize) {
//     let plays = input.lines().map(|line| {
//         let (them, us) = line.split_once(' ').unwrap();
//         (them.parse::<Play>().unwrap(), us.parse::<Play>().unwrap())
//     });

//     let mut result1 = 0;
//     let mut result2 = 0;

//     for (them, us) in plays {
//         result1 += Play::score(them, us);
//         let func = match us {
//             Play::Rock => Play::lose,
//             Play::Paper => Play::draw,
//             Play::Scissors => Play::win,
//         };
//         result2 += Play::score(them, func(them));
//     }

//     (result1, result2)
// }

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
