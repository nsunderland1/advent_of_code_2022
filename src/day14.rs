use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult};

use crate::grid;
#[allow(unused)]
use crate::prelude::*;

fn parse_point(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_usize, tag(","), parse_usize)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(tag(" -> "), parse_point)(input)
}

fn unordered_line_iterator(
    start: (usize, usize),
    end: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    // assumes start.x == end.x or start.y == end.y
    let x_range = std::cmp::min(start.0, end.0)..=std::cmp::max(start.0, end.0);
    let y_range = std::cmp::min(start.1, end.1)..=std::cmp::max(start.1, end.1);

    Itertools::cartesian_product(x_range, y_range)
}

fn count_sand(cave: &mut Grid<char>, position: (usize, usize)) -> (usize, bool) {
    if cave.get(position).is_none() {
        return (0, true);
    }

    if cave[position] != '.' {
        return (0, false);
    }

    let down = (position.0, position.1 + 1);
    let down_left = (position.0.saturating_sub(1), position.1 + 1);
    let down_right = (position.0 + 1, position.1 + 1);

    let mut total = 0;
    let moves = [down, down_left, down_right];

    for step in moves {
        let (count, leaked) = count_sand(cave, step);
        total += count;
        if leaked {
            return (total, true);
        }
    }

    cave[position] = 'o';
    total += 1;

    (total, false)
}

pub fn run(input: &str) -> (Solution, Solution) {
    let lines: Vec<_> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let width = 1 + lines
        .iter()
        .flat_map(|line| line.iter().map(|&(x, _)| x))
        .max()
        .unwrap();

    let height = 1 + lines
        .iter()
        .flat_map(|line| line.iter().map(|&(_, y)| y))
        .max()
        .unwrap();

    let mut cave = grid!['.'; width, height];
    for line in lines {
        for segment in line.windows(2) {
            for point in unordered_line_iterator(segment[0], segment[1]) {
                cave[point] = '#';
            }
        }
    }

    let starting_point = (500, 0);

    let result1 = {
        let mut cave = cave.clone();
        let (count, leaked) = count_sand(&mut cave, starting_point);
        assert!(leaked);
        count
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
        const DAY: u32 = 14;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
