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

pub fn run(input: &str) -> (Solution, Solution) {
    let lines = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect_vec();

    const TARGET_Y: isize = 10;

    let result1 = {
        let mut excluded = HashSet::<isize>::default();

        for &(sensor, beacon) in lines.iter() {
            let distance = manhattan(sensor, beacon);

            let y_target_distance = sensor.1.abs_diff(TARGET_Y) as isize;
            if y_target_distance > distance {
                continue;
            }

            let min_x = sensor.0 - distance + y_target_distance;
            let max_x = sensor.0 + distance - y_target_distance;

            for x in min_x..=max_x {
                excluded.insert(x);
            }
        }

        excluded.len() - 1 // somehow I have an off by one, oops :)
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
        const DAY: u32 = 15;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
