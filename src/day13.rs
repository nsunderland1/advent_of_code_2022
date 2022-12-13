use std::cmp::Ordering;

use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

#[allow(unused)]
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::Int(_), _) => Packet::List(vec![self.clone()]).cmp(other),
            (_, Packet::Int(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::List(left), Packet::List(right)) => {
                let mut left = left.into_iter();
                let mut right = right.into_iter();

                loop {
                    match (left.next(), right.next()) {
                        (None, None) => return Ordering::Equal,
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(l), Some(r)) => {
                            let comparison = l.cmp(r);
                            if comparison != Ordering::Equal {
                                return comparison;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |n| n as usize)(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(parse_usize, Packet::Int),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            Packet::List,
        ),
    ))(input)
}

pub fn run(input: &str) -> (Solution, Solution) {
    let packets: Vec<_> = input
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            let left = parse_packet(left).unwrap().1;
            let right = parse_packet(right).unwrap().1;
            (left, right)
        })
        .collect();

    let result1 = packets
        .iter()
        .map(|(left, right)| left < right)
        .enumerate()
        .filter(|(_, correct)| *correct)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let padding = [
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];
    let mut packets: Vec<_> = packets
        .into_iter()
        .flat_map(|(l, r)| [l, r])
        .chain(padding.clone())
        .collect();

    packets.sort();

    let result2 = packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| *packet == padding[0] || *packet == padding[1])
        .map(|(i, _)| i + 1)
        .product::<usize>();

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 13;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
