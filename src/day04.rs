#[allow(unused)]
use crate::prelude::*;

fn parse_int(mut bytes: &[u8], delimiter: u8) -> (&[u8], u8) {
    let mut num = 0;
    while bytes[0] != delimiter {
        num *= 10;
        num += bytes[0] - b'0';
        bytes = &bytes[1..];
    }
    (&bytes[1..], num)
}

pub fn run(input: &str) -> (usize, usize) {
    let mut bytes = input.as_bytes();

    let mut result1 = 0;
    let mut result2 = 0;

    while !bytes.is_empty() {
        let s1;
        (bytes, s1) = parse_int(bytes, b'-');

        let e1;
        (bytes, e1) = parse_int(bytes, b',');

        let s2;
        (bytes, s2) = parse_int(bytes, b'-');

        let e2;
        (bytes, e2) = parse_int(bytes, b'\n');

        let contained = (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1);
        result1 += contained as usize;

        let overlapping = (s2 >= s1 && s2 <= e1) || (s1 >= s2 && s1 <= e2);
        result2 += overlapping as usize;
    }

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
