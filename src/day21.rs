use std::ops::{Add, Div, Mul, Sub};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{map, value},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[allow(unused)]
use crate::prelude::*;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Div,
    Sub,
}

impl Op {
    fn func(self) -> fn(isize, isize) -> isize {
        match self {
            Op::Add => Add::add,
            Op::Mul => Mul::mul,
            Op::Div => Div::div,
            Op::Sub => Sub::sub,
        }
    }
}

#[derive(Clone)]
enum Monkey<'a> {
    Number(isize),
    BinOp(Op, &'a str, &'a str),
}

struct MonkeyAnnotated<'a> {
    name: &'a str,
    monkey: Monkey<'a>,
    contains_human: bool,
}

impl<'a> Monkey<'a> {
    fn eval(&self, monkeys: &HashMap<&str, Self>) -> isize {
        match self {
            Monkey::Number(num) => *num,
            Monkey::BinOp(op, l, r) => {
                let l = monkeys.get(l).unwrap().eval(monkeys);
                let r = monkeys.get(r).unwrap().eval(monkeys);
                op.func()(l, r)
            }
        }
    }
}

impl<'a> MonkeyAnnotated<'a> {
    fn solve_human(
        &self,
        target: isize,
        monkeys: &HashMap<&str, Monkey<'a>>,
        annotated_monkeys: &HashMap<&str, Self>,
    ) -> isize {
        match self.monkey {
            Monkey::Number(_) => target,
            Monkey::BinOp(op, l, r) => {
                let left_annotated = annotated_monkeys.get(l).unwrap();
                let right_annotated = annotated_monkeys.get(r).unwrap();

                match (op, left_annotated, right_annotated) {
                    (
                        Op::Add,
                        nonhuman @ MonkeyAnnotated {
                            contains_human: false,
                            ..
                        },
                        human,
                    )
                    | (
                        Op::Add,
                        human @ MonkeyAnnotated {
                            contains_human: true,
                            ..
                        },
                        nonhuman,
                    ) => {
                        let nonhuman_monkey = monkeys.get(nonhuman.name).unwrap();
                        let new_target = target - nonhuman_monkey.eval(monkeys);
                        human.solve_human(new_target, monkeys, annotated_monkeys)
                    }
                    (
                        Op::Mul,
                        nonhuman @ MonkeyAnnotated {
                            contains_human: false,
                            ..
                        },
                        human,
                    )
                    | (
                        Op::Mul,
                        human @ MonkeyAnnotated {
                            contains_human: true,
                            ..
                        },
                        nonhuman,
                    ) => {
                        let nonhuman_monkey = monkeys.get(nonhuman.name).unwrap();
                        let new_target = target / nonhuman_monkey.eval(monkeys);
                        human.solve_human(new_target, monkeys, annotated_monkeys)
                    }
                    (
                        Op::Div,
                        nonhuman @ MonkeyAnnotated {
                            contains_human: false,
                            ..
                        },
                        human,
                    ) => {
                        let nonhuman_monkey = monkeys.get(nonhuman.name).unwrap();
                        let new_target = nonhuman_monkey.eval(monkeys) / target;
                        human.solve_human(new_target, monkeys, annotated_monkeys)
                    }
                    (
                        Op::Div,
                        human @ MonkeyAnnotated {
                            contains_human: true,
                            ..
                        },
                        nonhuman,
                    ) => {
                        let nonhuman_monkey = monkeys.get(nonhuman.name).unwrap();
                        let new_target = target * nonhuman_monkey.eval(monkeys);
                        human.solve_human(new_target, monkeys, annotated_monkeys)
                    }
                    (
                        Op::Sub,
                        nonhuman @ MonkeyAnnotated {
                            contains_human: false,
                            ..
                        },
                        human,
                    ) => {
                        let nonhuman_monkey = monkeys.get(nonhuman.name).unwrap();
                        let new_target = nonhuman_monkey.eval(monkeys) - target;
                        human.solve_human(new_target, monkeys, annotated_monkeys)
                    }
                    (
                        Op::Sub,
                        human @ MonkeyAnnotated {
                            contains_human: true,
                            ..
                        },
                        nonhuman,
                    ) => {
                        let nonhuman_monkey = monkeys.get(nonhuman.name).unwrap();
                        let new_target = target + nonhuman_monkey.eval(monkeys);
                        human.solve_human(new_target, monkeys, annotated_monkeys)
                    }
                }
            }
        }
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map(nom::character::complete::i64, |n| n as isize)(input)
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    delimited(
        tag(" "),
        alt((
            value(Op::Add, tag("+")),
            value(Op::Mul, tag("*")),
            value(Op::Div, tag("/")),
            value(Op::Sub, tag("-")),
        )),
        tag(" "),
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey<'_>> {
    alt((
        map(parse_isize, Monkey::Number),
        map(tuple((alpha1, parse_op, alpha1)), |(l, op, r)| {
            Monkey::BinOp(op, l, r)
        }),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Monkey<'_>)> {
    separated_pair(alpha1, tag(": "), parse_monkey)(input)
}

fn build_annotations<'a>(
    monkey_name: &'a str,
    monkeys: &HashMap<&'a str, Monkey<'a>>,
    annotated_monkeys: &mut HashMap<&'a str, MonkeyAnnotated<'a>>,
) -> bool {
    let monkey = monkeys.get(monkey_name).unwrap().clone();
    if monkey_name == "humn" {
        annotated_monkeys.insert(
            monkey_name,
            MonkeyAnnotated {
                name: monkey_name,
                monkey,
                contains_human: true,
            },
        );
        return true;
    }

    let contains_human = match monkey {
        Monkey::Number(_) => false,
        Monkey::BinOp(_, l, r) => {
            let left_contains_human = build_annotations(l, monkeys, annotated_monkeys);
            let right_contains_human = build_annotations(r, monkeys, annotated_monkeys);

            left_contains_human || right_contains_human
        }
    };

    annotated_monkeys.insert(
        monkey_name,
        MonkeyAnnotated {
            name: monkey_name,
            monkey,
            contains_human,
        },
    );

    contains_human
}

pub fn run(input: &str) -> (Solution, Solution) {
    let monkeys: HashMap<&str, Monkey<'_>> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let result1 = { monkeys.get("root").unwrap().eval(&monkeys) } as usize;

    let result2 = {
        let mut annotated_monkeys = HashMap::default();
        assert!(build_annotations("root", &monkeys, &mut annotated_monkeys));

        let Monkey::BinOp(_, l, r) = monkeys.get("root").unwrap() else {
            unreachable!()
        };

        let l_annotated = annotated_monkeys.get(l).unwrap();
        let r_annotated = annotated_monkeys.get(r).unwrap();

        if l_annotated.contains_human {
            let r_monkey = monkeys.get(r).unwrap();
            l_annotated.solve_human(r_monkey.eval(&monkeys), &monkeys, &annotated_monkeys)
        } else {
            let l_monkey = monkeys.get(l).unwrap();
            r_annotated.solve_human(l_monkey.eval(&monkeys), &monkeys, &annotated_monkeys)
        }
    } as usize;

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 21;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
