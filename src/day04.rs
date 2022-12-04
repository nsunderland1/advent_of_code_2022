#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (usize, usize) {
    let result1 = input
        .lines()
        .filter(|line| {
            let (r1, r2) = line.split_once(',').unwrap();
            let (s1, e1) = r1.split_once('-').unwrap();
            let (s2, e2) = r2.split_once('-').unwrap();
            let s1 = s1.parse::<usize>().unwrap();
            let s2 = s2.parse::<usize>().unwrap();
            let e1 = e1.parse::<usize>().unwrap();
            let e2 = e2.parse::<usize>().unwrap();

            (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1)
        })
        .count();
    let result2 = input
        .lines()
        .filter(|line| {
            let (r1, r2) = line.split_once(',').unwrap();
            let (s1, e1) = r1.split_once('-').unwrap();
            let (s2, e2) = r2.split_once('-').unwrap();
            let s1 = s1.parse::<usize>().unwrap();
            let s2 = s2.parse::<usize>().unwrap();
            let e1 = e1.parse::<usize>().unwrap();
            let e2 = e2.parse::<usize>().unwrap();

            (s2 >= s1 && s2 <= e1) || (s1 >= s2 && s1 <= e2)
        })
        .count();

    (result1, result2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 4;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
