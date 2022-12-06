use std::collections::VecDeque;

#[allow(unused)]
use crate::prelude::*;

fn solve(input: &str, n: usize) -> usize {
    let mut last_n = VecDeque::with_capacity(n);

    for (i, char) in input.chars().enumerate() {
        if last_n.len() == n {
            last_n.pop_front();
        }
        last_n.push_back(char);

        if last_n.iter().collect::<HashSet<_>>().len() == n {
            return i + 1;
        }
    }

    unreachable!()
}

pub fn run(input: &str) -> (Solution, Solution) {
    let result1 = solve(input, 4);
    let result2 = solve(input, 14);

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 6;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
