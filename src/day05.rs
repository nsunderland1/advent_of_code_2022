use nom::{bytes::complete::tag, sequence::tuple, IResult};

#[allow(unused)]
use crate::prelude::*;

fn parse_line(string: &[u8]) -> IResult<&[u8], (usize, usize, usize)> {
    let (rem, (_, a, _, b, _, c)) = tuple((
        tag("move "),
        nom::character::complete::u64,
        tag(" from "),
        nom::character::complete::u64,
        tag(" to "),
        nom::character::complete::u64,
    ))(string)?;

    Ok((rem, (a as usize, b as usize, c as usize)))
}

pub fn run(input: &str) -> (usize, usize) {
    let mut stacks = Vec::new();

    let mut lines = input.lines();

    'outer: loop {
        let line = lines.next().unwrap();

        let stack_entries = line.as_bytes().chunks(4);
        for (stack_id, stack_entry) in stack_entries.into_iter().enumerate() {
            if stack_entry[1] == b'1' {
                break 'outer;
            }

            let stack_char = stack_entry[1];
            if stack_id >= stacks.len() {
                stacks.push(Vec::new());
            }

            if stack_entry[1] == b' ' {
                continue;
            }

            stacks[stack_id].push(stack_char as char);
        }
    }

    assert_eq!(lines.next().unwrap(), "");
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    for line in lines {
        let Ok((_, (num, from, to))) = parse_line(line.as_bytes()) else { break; };

        // for _ in 0..num {
        //     let val = stacks[from - 1].pop().unwrap();
        //     stacks[to - 1].push(val);
        // }

        let split_index = stacks[from - 1].len() - num;
        let to_move = stacks[from - 1].split_off(split_index);
        stacks[to - 1].extend_from_slice(&to_move);
    }

    for stack in stacks {
        print!("{}", *stack.last().unwrap());
    }

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
