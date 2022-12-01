#[allow(unused)]
use crate::prelude::*;

fn parse_line(s: &str) -> u32 {
    s.parse().unwrap()
}

pub fn run(input: &str) -> (usize, usize) {
    #[allow(unused)]
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let result1 = {
        // Part 1
        0
    };

    let result2 = {
        // Part 2
        0
    };

    (result1, result2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = todo!();
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
