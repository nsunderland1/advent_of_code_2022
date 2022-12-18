use std::{cmp::Ordering, convert, ops::RangeInclusive};

use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

#[allow(unused)]
use crate::prelude::*;

fn parse_line(input: &str) -> IResult<&str, ((isize, isize), (isize, isize))> {
    map(
        tuple((
            tag("Sensor at x="),
            parse_isize,
            tag(", y="),
            parse_isize,
            tag(": closest beacon is at x="),
            parse_isize,
            tag(", y="),
            parse_isize,
        )),
        |(_, sx, _, sy, _, bx, _, by)| ((sx, sy), (bx, by)),
    )(input)
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map(nom::character::complete::i64, |n| n as isize)(input)
}

fn manhattan(start: (isize, isize), end: (isize, isize)) -> isize {
    (start.0.abs_diff(end.0) + start.1.abs_diff(end.1)) as isize
}

const MAX_COORD: isize = 4000000;

fn points_at_distance(
    position: (isize, isize),
    distance: isize,
) -> impl Iterator<Item = (isize, isize)> {
    let left = ((position.0 - distance)..=position.0)
        .enumerate()
        .flat_map(move |(i, x)| [(x, position.1 + i as isize), (x, position.1 - i as isize)]);

    let right = ((position.0 + 1)..=(position.0 + distance))
        .rev()
        .enumerate()
        .flat_map(move |(i, x)| [(x, position.1 + i as isize), (x, position.1 - i as isize)]);

    left.chain(right)
        .filter(|(x, y)| (0..=MAX_COORD).contains(&x) && (0..=MAX_COORD).contains(&y))
}

pub fn run(input: &str) -> (Solution, Solution) {
    let lines = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect_vec();

    const TARGET_Y: isize = 2_000_000;

    let result1 = {
        let mut excluded_ranges: Vec<RangeInclusive<isize>> = Vec::with_capacity(lines.len());

        for &(sensor, beacon) in lines.iter() {
            let distance = manhattan(sensor, beacon);

            let y_target_distance = sensor.1.abs_diff(TARGET_Y) as isize;
            if y_target_distance > distance {
                continue;
            }

            let min_x = sensor.0 - distance + y_target_distance;
            let max_x = sensor.0 + distance - y_target_distance;

            let start_index = excluded_ranges.binary_search_by(|range| {
                if min_x < *range.start() {
                    Ordering::Greater
                } else if min_x > *range.end() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });

            let end_index = excluded_ranges.binary_search_by(|range| {
                if max_x < *range.start() {
                    Ordering::Greater
                } else if max_x > *range.end() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });

            let (start, end) = if start_index == end_index {
                if let (Err(start), Err(_)) = (start_index, end_index) {
                    excluded_ranges.insert(start, min_x..=max_x);
                }

                (
                    start_index.unwrap_or_else(convert::identity),
                    end_index.unwrap_or_else(convert::identity),
                )
            } else {
                (
                    start_index.unwrap_or_else(|start| start),
                    end_index.unwrap_or_else(|end| end - 1),
                )
            };

            excluded_ranges[start] =
                min_x.min(*excluded_ranges[start].start())..=max_x.max(*excluded_ranges[end].end());

            for _ in excluded_ranges.drain((start + 1)..=end) {}
        }

        excluded_ranges
            .into_iter()
            .map(|range| range.end() - range.start() + 1)
            .sum::<isize>() as usize
            - 1
    };

    let result2 = {
        let mut result = None;

        'outer: for &(sensor, beacon) in lines.iter() {
            let distance = manhattan(sensor, beacon);

            for point in points_at_distance(sensor, distance + 1) {
                let mut found = false;

                for &(sensor, beacon) in lines.iter() {
                    if manhattan(sensor, point) <= manhattan(sensor, beacon) {
                        found = true;
                        break;
                    }
                }

                if !found {
                    result = Some(point);
                    break 'outer;
                }
            }
        }

        let result = result.unwrap();

        result.0 * MAX_COORD + result.1
    };

    (result1.into(), (result2 as usize).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 15;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
