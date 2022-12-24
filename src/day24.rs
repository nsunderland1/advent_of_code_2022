use std::fmt::Display;

#[allow(unused)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq)]
enum Square {
    Wall,
    Open(Vec<(isize, isize)>),
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_char = match self {
            Square::Wall => '#',
            Square::Open(blizzards) => match blizzards.as_slice() {
                [] => '.',
                [LEFT] => '<',
                [RIGHT] => '>',
                [UP] => '^',
                [DOWN] => 'v',
                _ => blizzards.len().to_string().chars().take(1).next().unwrap(),
            },
        };

        write!(f, "{as_char}")
    }
}

const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Open(Vec::new()),
            '>' => Self::Open(vec![RIGHT]),
            '<' => Self::Open(vec![LEFT]),
            '^' => Self::Open(vec![UP]),
            'v' => Self::Open(vec![DOWN]),
            _ => unreachable!(),
        }
    }
}

pub fn run(input: &str) -> (Solution, Solution) {
    let map: Grid<Square> = input
        .lines()
        .map(|line| line.chars().map(|c| Square::from(c)))
        .collect();

    let result1 = {
        let mut map = map.clone();

        let mut result = 0;

        let start = (1, 0);
        let end = (map.width() - 2, map.height() - 1);

        for (start, end) in [(start, end), (end, start), (start, end)] {
            let mut positions = HashSet::from_iter([start]);
            for num_moves in 1.. {
                let mut next_map = grid![Square::Open(Vec::new()); map.width(), map.height()];

                for y in 1..(map.height() - 1) {
                    for x in 1..(map.width() - 1) {
                        match &map[(x, y)] {
                            Square::Wall => (),
                            Square::Open(blizzards) => {
                                for blizzard in blizzards.iter().copied() {
                                    let mut next_position = (
                                        (x as isize + blizzard.0) as usize,
                                        (y as isize + blizzard.1) as usize,
                                    );

                                    if next_position.0 == 0 {
                                        next_position.0 = map.width() - 2;
                                    } else if next_position.0 == map.width() - 1 {
                                        next_position.0 = 1;
                                    } else if next_position.1 == 0 {
                                        next_position.1 = map.height() - 2;
                                    } else if next_position.1 == map.height() - 1 {
                                        next_position.1 = 1;
                                    }

                                    let Square::Open(next_blizzards) = &mut next_map[next_position] else {
                                        unreachable!();
                                    };
                                    next_blizzards.push(blizzard);
                                }
                            }
                        }
                    }
                }

                let mut next_positions = HashSet::default();

                for position in positions.into_iter() {
                    for option in map.neighbours_orthogonal(position).chain([position]) {
                        if map[option] != Square::Wall
                            && next_map[option] == Square::Open(Vec::new())
                        {
                            next_positions.insert(option);
                        }
                    }
                }

                positions = next_positions;

                for y in 1..(map.height() - 1) {
                    for x in 1..(map.width() - 1) {
                        map[(x, y)] = next_map[(x, y)].clone();
                    }
                }

                if positions.contains(&end) {
                    result += num_moves;
                    break;
                }
            }
        }

        result
    };

    let result2 = {
        // Part 2
        0
    };

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 24;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
