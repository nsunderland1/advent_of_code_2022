use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::many1,
    IResult,
};

#[allow(unused)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Square {
    Open,
    Wall,
    Void,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_char = match self {
            Square::Open => '.',
            Square::Wall => '#',
            Square::Void => ' ',
        };

        write!(f, "{as_char}")
    }
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    Right,
    Advance(usize),
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        value(Move::Left, tag("L")),
        value(Move::Right, tag("R")),
        map(parse_usize, Move::Advance),
    )))(input)
}

pub fn run(input: &str) -> (Solution, Solution) {
    let (input_map, moves) = input.split_once("\n\n").unwrap();

    let moves = parse_moves(moves).unwrap().1;

    let input_map: Vec<Vec<Square>> = input_map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    ' ' => Square::Void,
                    '.' => Square::Open,
                    '#' => Square::Wall,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let width = input_map.iter().map(|row| row.len()).max().unwrap();
    let height = input_map.len();

    let mut game_map = grid![Square::Void; width, height];

    for (y, row) in input_map.iter().enumerate() {
        for (x, &square) in row.iter().enumerate() {
            game_map[(x, y)] = square;
        }
    }

    let mut row_ranges = Vec::with_capacity(game_map.height());
    for y in 0..game_map.height() {
        let start = (0..game_map.width())
            .find(|&x| game_map[(x, y)] != Square::Void)
            .unwrap();
        let end = (0..game_map.width())
            .rev()
            .find(|&x| game_map[(x, y)] != Square::Void)
            .unwrap();

        row_ranges.push((start as isize)..=(end as isize));
    }

    // We're gonna hardcode all this stuff cause who has time to do it programmatically
    let mut row_start_wraps = Vec::with_capacity(game_map.height());
    let mut row_end_wraps = Vec::with_capacity(game_map.height());

    for y in 0..50 {
        row_start_wraps.push((RIGHT, (0, 150 - y - 1)));
        row_end_wraps.push((LEFT, (99, 150 - y - 1)));
    }
    for y in 50..100 {
        row_start_wraps.push((DOWN, (y - 50, 100)));
        row_end_wraps.push((UP, (y - 50 + 100, 49)));
    }
    for y in 100..150 {
        row_start_wraps.push((RIGHT, (50, 150 - y - 1)));
        row_end_wraps.push((LEFT, (149, 150 - y - 1)));
    }
    for y in 150..200 {
        row_start_wraps.push((DOWN, (y - 150 + 50, 0)));
        row_end_wraps.push((UP, (y - 150 + 50, 149)));
    }

    let mut column_start_wraps = Vec::with_capacity(game_map.width());
    let mut column_end_wraps = Vec::with_capacity(game_map.width());

    for x in 0..50 {
        column_start_wraps.push((RIGHT, (50, (x + 50))));
        column_end_wraps.push((DOWN, (x + 100, 0)));
    }
    for x in 50..100 {
        column_start_wraps.push((RIGHT, (0, x - 50 + 150)));
        column_end_wraps.push((LEFT, (50, x - 50 + 150)));
    }
    for x in 100..150 {
        column_start_wraps.push((UP, (x - 100, 199)));
        column_end_wraps.push((LEFT, (99, x - 100 + 50)));
    }

    let mut column_ranges = Vec::with_capacity(game_map.width());
    for x in 0..game_map.width() {
        let start = (0..game_map.height())
            .find(|&y| game_map[(x, y)] != Square::Void)
            .unwrap();
        let end = (0..game_map.height())
            .rev()
            .find(|&y| game_map[(x, y)] != Square::Void)
            .unwrap();

        column_ranges.push((start as isize)..=(end as isize));
    }

    const LEFT: (isize, isize) = (-1, 0);
    const RIGHT: (isize, isize) = (1, 0);
    const UP: (isize, isize) = (0, -1);
    const DOWN: (isize, isize) = (0, 1);

    let result1 = {
        let mut orientation = RIGHT;

        let start_x = row_ranges[0]
            .find(|&x| game_map[(x as usize, 0)] == Square::Open)
            .unwrap();

        let mut position = (start_x as isize, 0);

        for next_move in moves.iter().copied() {
            match next_move {
                Move::Left => orientation = (orientation.1, -orientation.0),
                Move::Right => orientation = (-orientation.1, orientation.0),
                Move::Advance(n) => {
                    for _ in 0..n {
                        let mut next_position =
                            (position.0 + orientation.0, position.1 + orientation.1);

                        let current_row_range = row_ranges[position.1 as usize].clone();
                        let current_column_range = column_ranges[position.0 as usize].clone();

                        if orientation == LEFT && position.0 == *current_row_range.start() {
                            next_position.0 = *current_row_range.end();
                        } else if orientation == RIGHT && position.0 == *current_row_range.end() {
                            next_position.0 = *current_row_range.start();
                        } else if orientation == UP && position.1 == *current_column_range.start() {
                            next_position.1 = *current_column_range.end();
                        } else if orientation == DOWN && position.1 == *current_column_range.end() {
                            next_position.1 = *current_column_range.start();
                        }

                        if game_map[(next_position.0 as usize, next_position.1 as usize)]
                            == Square::Open
                        {
                            position = next_position;
                        }
                    }
                }
            }
        }

        let facing = match orientation {
            RIGHT => 0,
            DOWN => 1,
            LEFT => 2,
            UP => 3,
            _ => unreachable!(),
        };

        (1000 * (position.1 + 1) + 4 * (position.0 + 1) + facing) as usize
    };

    let result2 = {
        let mut orientation = RIGHT;

        let start_x = row_ranges[0]
            .find(|&x| game_map[(x as usize, 0)] == Square::Open)
            .unwrap();

        let mut position = (start_x as isize, 0);

        for next_move in moves.iter().copied() {
            match next_move {
                Move::Left => orientation = (orientation.1, -orientation.0),
                Move::Right => orientation = (-orientation.1, orientation.0),
                Move::Advance(n) => {
                    for _ in 0..n {
                        let mut next_position =
                            (position.0 + orientation.0, position.1 + orientation.1);
                        let mut next_orientation = orientation;

                        let current_row_range = row_ranges[position.1 as usize].clone();
                        let current_column_range = column_ranges[position.0 as usize].clone();

                        if orientation == LEFT && position.0 == *current_row_range.start() {
                            (next_orientation, next_position) =
                                row_start_wraps[position.1 as usize];
                        } else if orientation == RIGHT && position.0 == *current_row_range.end() {
                            (next_orientation, next_position) = row_end_wraps[position.1 as usize];
                        } else if orientation == UP && position.1 == *current_column_range.start() {
                            (next_orientation, next_position) =
                                column_start_wraps[position.0 as usize];
                        } else if orientation == DOWN && position.1 == *current_column_range.end() {
                            (next_orientation, next_position) =
                                column_end_wraps[position.0 as usize];
                        }

                        if game_map[(next_position.0 as usize, next_position.1 as usize)]
                            == Square::Open
                        {
                            orientation = next_orientation;
                            position = next_position;
                        }
                    }
                }
            }
        }

        let facing = match orientation {
            RIGHT => 0,
            DOWN => 1,
            LEFT => 2,
            UP => 3,
            _ => unreachable!(),
        };

        (1000 * (position.1 + 1) + 4 * (position.0 + 1) + facing) as usize
    };

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 22;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
