use std::{cmp::Ordering, collections::VecDeque};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, tuple},
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

fn parse_instruction(string: &[u8]) -> IResult<&[u8], (usize, usize, usize)> {
    let (rem, (_, count, _, from, _, to)) = tuple((
        tag("move "),
        nom::character::complete::u64,
        tag(" from "),
        nom::character::complete::u64,
        tag(" to "),
        nom::character::complete::u64,
    ))(string)?;

    Ok((rem, (count as usize, from as usize - 1, to as usize - 1)))
}

pub fn run(input: &str) -> (usize, usize) {
    let mut stacks = Vec::new();

    let mut lines = input.lines();

    while let Ok(_) = parse_stack_row(lines.next().unwrap().as_bytes(), &mut stacks) {}

    assert_eq!(lines.next().unwrap(), "");

    let instructions: Vec<_> = lines
        .map(|line| parse_instruction(line.as_bytes()).unwrap().1)
        .collect();

    {
        let mut stacks = stacks.clone();

        for &(num, from, to) in &instructions {
            for _ in 0..num {
                let val = stacks[from].pop_front().unwrap();
                stacks[to].push_front(val);
            }
        }

        for stack in stacks {
            print!("{}", *stack.front().unwrap() as char);
        }
    }

    println!();

    {
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

        for stack in stacks {
            print!("{}", *stack.front().unwrap() as char);
        }
    }

    println!();

    // println!("Part 1:");
    (0, 0)
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
