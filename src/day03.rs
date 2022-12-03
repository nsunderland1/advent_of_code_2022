#[allow(unused)]
use crate::prelude::*;

fn byte_priority(byte: u8) -> usize {
    (match byte {
        b'a'..=b'z' => 1 + byte - b'a',
        b'A'..=b'Z' => 27 + byte - b'A',
        _ => unreachable!(),
    }) as usize
}

fn process_line(line: &[u8], seen: &mut [u8; 52]) -> usize {
    seen.fill(0);

    let (left, right) = line.split_at(line.len() / 2);

    for &byte in left {
        seen[byte_priority(byte) - 1] = 1;
    }

    let mut part1_score = 0;
    for &byte in right {
        let priority = byte_priority(byte);
        if seen[priority - 1] == 1 {
            part1_score = priority;
        }
        seen[priority - 1] = 2;
    }

    part1_score
}

const NUM_LETTERS: usize = 52;

pub fn run(input: &str) -> (usize, usize) {
    let mut seen = [[0u8; NUM_LETTERS]; 3];

    let mut part1_score = 0;
    let mut part2_score = 0;
    for three_lines in input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .chunks(3)
        .into_iter()
    {
        let (line_1, line_2, line_3) = three_lines.collect_tuple().unwrap();

        part1_score += process_line(line_1, &mut seen[0]);
        part1_score += process_line(line_2, &mut seen[1]);
        part1_score += process_line(line_3, &mut seen[2]);

        for i in 0..NUM_LETTERS {
            if seen[0][i] > 0 && seen[1][i] > 0 && seen[2][i] > 0 {
                part2_score += i + 1;
                break;
            }
        }
    }

    (part1_score, part2_score)
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
