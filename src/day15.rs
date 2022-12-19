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

const MAX_COORD: isize = 4_000_000;

fn intersect(
    up_start: (isize, isize),
    up_end: (isize, isize),
    down_start: (isize, isize),
    down_end: (isize, isize),
) -> Option<(isize, isize)> {
    // up: x + a
    // down: -x + b
    // x + a == -x + b
    // 2x = b - a
    let a = up_start.1 - up_start.0;
    let b = down_start.1 + down_start.0;

    if (b - a) % 2 != 0 {
        return None;
    }

    let intersection_x = (b - a) / 2;

    if (up_start.0..=up_end.0).contains(&intersection_x)
        && (down_start.0..=down_end.0).contains(&intersection_x)
    {
        Some((intersection_x, up_start.1 + (intersection_x - up_start.0)))
    } else {
        None
    }
}

pub fn run(input: &str) -> (Solution, Solution) {
    let lines = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .map(|(sensor, beacon)| (sensor, beacon, manhattan(sensor, beacon)))
        .collect_vec();

    const TARGET_Y: isize = 2_000_000;

    let result1 = {
        let mut excluded_ranges: Vec<RangeInclusive<isize>> = Vec::with_capacity(lines.len());

        for &(sensor, _, distance) in lines.iter() {
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
                    start_index.unwrap_or_else(convert::identity),
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
        let diamonds: Vec<[(isize, isize); 4]> = lines
            .iter()
            .map(|&(sensor, _, distance)| {
                [
                    (sensor.0 - distance - 1, sensor.1),
                    (sensor.0, sensor.1 + distance + 1),
                    (sensor.0 + distance + 1, sensor.1),
                    (sensor.0, sensor.1 - distance - 1),
                ]
            })
            .collect();

        let result = diamonds
            .iter()
            .enumerate()
            .cartesian_product(diamonds.iter().enumerate())
            .find_map(|((i, a), (j, b))| {
                if i == j {
                    return None;
                }

                let a_up = [(a[0], a[1]), (a[3], a[2])];
                let a_down = [(a[0], a[3]), (a[1], a[2])];

                let b_up = [(b[0], b[1]), (b[3], b[2])];
                let b_down = [(b[0], b[3]), (b[1], b[2])];

                Iterator::chain(
                    a_up.into_iter().cartesian_product(b_down),
                    b_up.into_iter().cartesian_product(a_down),
                )
                .find_map(|(up, down)| {
                    let intersection = intersect(up.0, up.1, down.0, down.1)?;
                    if !(0..=MAX_COORD).contains(&intersection.0)
                        || !(0..=MAX_COORD).contains(&intersection.1)
                    {
                        return None;
                    }

                    for &(sensor, _, distance) in lines.iter() {
                        if manhattan(sensor, intersection) <= distance {
                            return None;
                        }
                    }

                    Some(intersection)
                })
            })
            .unwrap();

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
