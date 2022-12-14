use std::{
    cell::RefCell,
    ops::{Add, Mul},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[allow(unused)]
use crate::prelude::*;

#[derive(Clone, Copy)]
enum Operand {
    Old,
    Value(usize),
}

impl Operand {
    fn value(self, old: usize) -> usize {
        match self {
            Self::Old => old,
            Self::Value(value) => value,
        }
    }
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn apply(self, left: usize, right: usize) -> usize {
        let op = match self {
            Self::Add => Add::add,
            Self::Mul => Mul::mul,
        };

        op(left, right)
    }
}

#[derive(Clone, Copy)]
struct Operation {
    operator: Operator,
    left: Operand,
    right: Operand,
}

impl Operation {
    fn apply(self, old: usize) -> usize {
        self.operator
            .apply(self.left.value(old), self.right.value(old))
    }
}

#[derive(Clone, Copy)]
struct Test {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn select_destination(self, worry_level: usize) -> usize {
        if worry_level % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Clone)]
struct Monkey {
    _id: usize,
    items: RefCell<Vec<usize>>,
    operation: Operation,
    test: Test,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |n| n as usize)(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), parse_usize),
    )(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        value(Operand::Old, tag("old")),
        map(parse_usize, Operand::Value),
    ))(input)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    delimited(
        tag(" "),
        alt((
            value(Operator::Add, tag("+")),
            value(Operator::Mul, tag("*")),
        )),
        tag(" "),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    preceded(
        tag("  Operation: new = "),
        map(
            tuple((parse_operand, parse_operator, parse_operand)),
            |(left, operator, right)| Operation {
                operator,
                left,
                right,
            },
        ),
    )(input)
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    map(
        tuple((
            delimited(tag("  Test: divisible by "), parse_usize, newline),
            delimited(tag("    If true: throw to monkey "), parse_usize, newline),
            preceded(tag("    If false: throw to monkey "), parse_usize),
        )),
        |(divisible_by, if_true, if_false)| Test {
            divisible_by,
            if_true,
            if_false,
        },
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            delimited(tag("Monkey "), parse_usize, tag(":\n")),
            terminated(parse_items, newline),
            terminated(parse_operation, newline),
            terminated(parse_test, newline),
        )),
        |(id, items, operation, test)| Monkey {
            _id: id,
            items: RefCell::new(items),
            operation,
            test,
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_monkey)(input)
}

fn solve(monkeys: Vec<Monkey>, num_rounds: usize, reduce: impl Fn(usize) -> usize) -> usize {
    let mut inspected_counts = vec![0usize; monkeys.len()];

    for _ in 0..num_rounds {
        for (monkey, inspected_count) in Iterator::zip(monkeys.iter(), inspected_counts.iter_mut())
        {
            for mut worry_level in monkey.items.borrow_mut().drain(..) {
                *inspected_count += 1;
                worry_level = monkey.operation.apply(worry_level);
                worry_level = reduce(worry_level);
                let destination = monkey.test.select_destination(worry_level);
                monkeys[destination].items.borrow_mut().push(worry_level);
            }
        }
    }

    inspected_counts.sort();

    inspected_counts
        .into_iter()
        .rev()
        .take(2)
        .product::<usize>()
}

pub fn run(input: &str) -> (Solution, Solution) {
    let (_, monkeys) = parse_input(input).unwrap();

    let result1 = { solve(monkeys.clone(), 20, |worry_level| worry_level / 3) };

    let result2 = {
        let product_of_divisors = monkeys
            .iter()
            .map(|monkey| monkey.test.divisible_by)
            .product::<usize>();

        solve(monkeys, 10_000, |worry_level| {
            worry_level % product_of_divisors
        })
    };

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 11;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
