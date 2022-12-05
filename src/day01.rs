#[allow(unused)]
use crate::prelude::*;
use crate::Solution;

fn parse_int_faster(ascii_bytes: &[u8]) -> usize {
    let mut total: usize = 0;
    for byte in ascii_bytes {
        total *= 10;
        total += (byte - b'0') as usize;
    }
    total
}

pub fn run(input: &str) -> (Solution, Solution) {
    // Invariant: always sorted in ascending order
    let mut heapish = [0usize; 4];

    let mut elf = 0usize;
    let input = input.as_bytes();
    for line in input.split(|&byte| byte == b'\n') {
        if line.is_empty() {
            heapish[0] = elf;
            heapish.sort();
            elf = 0;
            continue;
        }
        elf += parse_int_faster(line)
    }

    (
        heapish[3].into(),
        heapish.into_iter().skip(1).sum::<usize>().into(),
    )
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
