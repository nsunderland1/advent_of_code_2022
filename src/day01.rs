use crate::prelude::*;

fn solve(input: &str, count: usize) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|i| i.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(count)
        .sum()
}

pub fn run(input: &str) {
    let result1 = solve(input, 1);
    println!("Part 1: {result1}");

    let result2 = solve(input, 3);
    println!("Part 2: {result2}");
}
