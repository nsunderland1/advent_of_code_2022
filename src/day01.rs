#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (usize, usize) {
    // Invariant: always sorted in ascending order
    let mut heapish = [0usize; 4];

    let mut elf = 0usize;
    for line in input.lines() {
        if line.is_empty() {
            heapish[0] = elf;
            heapish.sort();
            elf = 0;
            continue;
        }
        elf += line.parse::<usize>().unwrap();
    }

    (heapish[3], heapish.into_iter().skip(1).sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 1;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}

// Original solution:
// fn solve(input: &str, count: usize) -> usize {
//     input
//         .split("\n\n")
//         .map(|elf| {
//             elf.split("\n")
//                 .map(|i| i.parse::<usize>().unwrap())
//                 .sum::<usize>()
//         })
//         .sorted()
//         .rev()
//         .take(count)
//         .sum()
// }

// pub fn run(input: &str) {
//     let result1 = solve(input, 1);
//     debugln!("Part 1: {result1}");

//     let result2 = solve(input, 3);
//     debugln!("Part 2: {result2}");
// }
