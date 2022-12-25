use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};

#[allow(unused)]
use crate::prelude::*;

fn parse_line(input: &str) -> IResult<&str, Vec<i8>> {
    many1(alt((
        value(2, tag("2")),
        value(1, tag("1")),
        value(0, tag("0")),
        value(-1, tag("-")),
        value(-2, tag("=")),
    )))(input)
}

fn normalify(number: &[i8]) -> isize {
    let mut result = 0;
    for &digit in number.iter() {
        result *= 5;
        result += digit as isize;
    }
    result
}

fn weirdify(mut number: isize) -> Vec<char> {
    let mut out = Vec::new();
    while number > 0 {
        let (diff, c) = match number % 5 {
            0 => (0, '0'),
            1 => (0, '1'),
            2 => (0, '2'),
            3 => (-2, '='),
            4 => (-1, '-'),
            _ => unreachable!(),
        };

        number -= diff;
        number /= 5;
        out.push(c);
    }

    out.reverse();
    out
}

pub fn run(input: &str) -> (Solution, Solution) {
    let numbers = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect_vec();

    let normal_numbers = numbers.iter().map(|num| normalify(num)).collect_vec();

    let sum = normal_numbers.iter().copied().sum::<isize>();

    dbg!(&sum);
    let result1 = { weirdify(sum).into_iter().collect::<String>() };

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
        const DAY: u32 = 25;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
