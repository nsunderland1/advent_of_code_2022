#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (usize, usize) {
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
