#[allow(unused)]
use crate::prelude::*;
use crate::Solution;

fn byte_bit(byte: u8) -> u64 {
    1 << (match byte {
        b'a'..=b'z' => byte - b'a',
        b'A'..=b'Z' => 26 + byte - b'A',
        _ => unreachable!(),
    }) as usize
}

fn process_line(line: &[u8]) -> (u32, u64) {
    let (left, right) = line.split_at(line.len() / 2);

    let left_set = left.iter().fold(0u64, |set, &byte| set | byte_bit(byte));
    let right_set = right.iter().fold(0u64, |set, &byte| set | byte_bit(byte));

    (
        (left_set & right_set).trailing_zeros() + 1,
        left_set | right_set,
    )
}

pub fn run(input: &str) -> (Solution, Solution) {
    let mut part1_score = 0;
    let mut part2_score = 0;

    let mut lines = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .filter(|line| !line.is_empty());

    while let Some(line) = lines.next() {
        let (score_1, set_1) = process_line(line);
        let (score_2, set_2) = process_line(lines.next().unwrap());
        let (score_3, set_3) = process_line(lines.next().unwrap());

        part1_score += score_1 + score_2 + score_3;
        part2_score += (set_1 & set_2 & set_3).trailing_zeros() + 1;
    }

    ((part1_score as usize).into(), (part2_score as usize).into())
}

// Original solution
// pub fn run(input: &str) -> (usize, usize) {
//     let mut result1 = 0;
//     input
//         .lines()
//         .map(|line| line.as_bytes())
//         .map(|line| {
//             let (left, right) = line.split_at(line.len() / 2);
//             let left: HashSet<_> = left.iter().copied().collect();
//             let right: HashSet<_> = right.iter().copied().collect();
//             let intersection = left.intersection(&right);

//             for byte in intersection {
//                 result1 += match byte {
//                     b'a'..=b'z' => 1 + byte - b'a',
//                     b'A'..=b'Z' => 27 + byte - b'A',
//                     _ => unreachable!(),
//                 } as usize;
//             }
//         })
//         .collect_vec();

//     let total = input
//         .lines()
//         .map(|line| line.as_bytes())
//         .chunks(3)
//         .into_iter()
//         .map(|chunk| {
//             let byte = chunk
//                 .map(|line| line.into_iter().copied().collect::<HashSet<u8>>())
//                 .reduce(|a, b| a.intersection(&b).copied().collect())
//                 .unwrap()
//                 .into_iter()
//                 .next()
//                 .unwrap();

//             (match byte {
//                 b'a'..=b'z' => 1 + byte - b'a',
//                 b'A'..=b'Z' => 27 + byte - b'A',
//                 _ => unreachable!(),
//             }) as usize
//         })
//         .sum();

//     (result1, total)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 3;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
