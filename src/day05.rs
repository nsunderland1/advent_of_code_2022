use std::{cmp::Ordering, collections::VecDeque};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, value},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

#[allow(unused)]
use crate::prelude::*;

fn parse_stack_entry(input: &[u8]) -> IResult<&[u8], Option<u8>> {
    alt((
        // Empty
        value(None, tag(b"   ")),
        // Stack entry
        map(
            // Single char wrapped in square brackets
            delimited(tag(b"["), take(1usize), tag(b"]")),
            // Always safe to do byte[0] because we did take(1)
            |byte: &[u8]| Some(byte[0]),
        ),
    ))(input)
}

fn parse_stack_row<'a>(input: &'a [u8], stacks: &mut Vec<VecDeque<u8>>) -> IResult<&'a [u8], ()> {
    let (remaining, row) = separated_list1(tag(" "), parse_stack_entry)(input)?;

    stacks.resize(row.len(), VecDeque::new());

    for (i, item) in row.into_iter().enumerate() {
        if let Some(item) = item {
            stacks[i].push_back(item);
        }
    }

    Ok((remaining, ()))
}

fn parse_int(mut bytes: &[u8], delimiter: u8) -> (&[u8], usize) {
    let mut num = 0;
    while bytes[0] != delimiter {
        num *= 10;
        num += (bytes[0] - b'0') as usize;
        bytes = &bytes[1..];
    }
    (&bytes[1..], num)
}

fn parse_instruction(mut input: &[u8]) -> (&[u8], usize, usize, usize) {
    let (count, from, to);
    (input, count) = parse_int(&input[5..], b' ');
    (input, from) = parse_int(&input[5..], b' ');
    (input, to) = parse_int(&input[3..], b'\n');
    (input, count, from - 1, to - 1)
}

pub fn run(input: &str) -> (Solution, Solution) {
    let mut stacks = Vec::new();

    let (stack, instructions) = input.split_once("\n\n").unwrap();

    for line in stack.lines() {
        let _ = parse_stack_row(line.as_bytes(), &mut stacks);
    }

    let mut instructions_bytes = instructions.as_bytes();
    let mut instructions = Vec::new();
    while !instructions_bytes.is_empty() {
        let (count, from, to);
        (instructions_bytes, count, from, to) = parse_instruction(instructions_bytes);
        instructions.push((count, from, to));
    }

    let result1: String = {
        let mut stacks = stacks.clone();

        for &(num, from, to) in &instructions {
            for _ in 0..num {
                let val = stacks[from].pop_front().unwrap();
                stacks[to].push_front(val);
            }
        }

        stacks
            .into_iter()
            .map(|stack| *stack.front().unwrap() as char)
            .collect()
    };

    let result2: String = {
        for (num, from, to) in instructions {
            let (from_stack, to_stack) = match from.cmp(&to) {
                Ordering::Less => {
                    let (left, right) = stacks.split_at_mut(to);
                    (&mut left[from], &mut right[0])
                }
                Ordering::Equal => continue,
                Ordering::Greater => {
                    let (left, right) = stacks.split_at_mut(from);
                    (&mut right[0], &mut left[to])
                }
            };

            for item in from_stack.drain(0..num).rev() {
                to_stack.push_front(item);
            }
        }

        stacks
            .into_iter()
            .map(|stack| *stack.front().unwrap() as char)
            .collect()
    };

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 5;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
