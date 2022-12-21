#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (Solution, Solution) {
    let mut input = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap() * 811589153)
        .enumerate()
        .collect_vec();

    let result1 = {
        for _ in 0..10 {
            for i in 0..input.len() {
                let current_index = input
                    .iter()
                    .enumerate()
                    .find(|(_, (original_index, _))| *original_index == i)
                    .unwrap()
                    .0;

                let shift = input[current_index].1;
                let shift = (shift.abs() % (input.len() as isize - 1)) * shift.signum();

                let mut target_index = (current_index as isize) + shift;
                if target_index < 0 {
                    target_index += input.len() as isize - 1;
                }
                if target_index >= input.len() as isize {
                    target_index %= input.len() as isize - 1;
                }
                let target_index = target_index as usize;

                let start = std::cmp::min(current_index, target_index);
                let end = std::cmp::max(current_index, target_index);

                if current_index == start {
                    input[start..=end].rotate_left(1);
                } else {
                    input[start..=end].rotate_right(1);
                }
            }
        }

        let zero_pos = input
            .iter()
            .enumerate()
            .find(|(_, (_, value))| *value == 0)
            .unwrap()
            .0;

        input[(zero_pos + 1000) % input.len()].1
            + input[(zero_pos + 2000) % input.len()].1
            + input[(zero_pos + 3000) % input.len()].1
    } as usize;

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
        const DAY: u32 = 20;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
