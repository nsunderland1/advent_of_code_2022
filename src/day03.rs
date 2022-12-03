#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (usize, usize) {
    let mut result1 = 0;
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left: HashSet<_> = left.iter().copied().collect();
            let right: HashSet<_> = right.iter().copied().collect();
            let intersection = left.intersection(&right);

            for byte in intersection {
                result1 += match byte {
                    b'a'..=b'z' => 1 + byte - b'a',
                    b'A'..=b'Z' => 27 + byte - b'A',
                    _ => unreachable!(),
                } as usize;
            }
        })
        .collect_vec();

    let total = input
        .lines()
        .map(|line| line.as_bytes())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let byte = chunk
                .map(|line| line.into_iter().copied().collect::<HashSet<u8>>())
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            (match byte {
                b'a'..=b'z' => 1 + byte - b'a',
                b'A'..=b'Z' => 27 + byte - b'A',
                _ => unreachable!(),
            }) as usize
        })
        .sum();

    (result1, total)
}

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
