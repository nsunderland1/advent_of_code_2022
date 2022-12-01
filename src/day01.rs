#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) {
    // Invariant: always sorted in ascending order
    let mut heapish = [0usize; 4];

    for elf in input.split("\n\n").map(|elf| {
        elf.split("\n")
            .map(|i| i.parse::<usize>().unwrap())
            .sum::<usize>()
    }) {
        // Top 3 will always be at indices 1, 2, 3, so insert at 0 and sort
        heapish[0] = elf;
        heapish.sort();
    }

    debugln!("Part 1: {}", heapish[3]);
    debugln!("Part 2: {}", heapish.into_iter().skip(1).sum::<usize>())
}

// Original solution:
// fn solve(input: &str, count: usize) -> usize {
//     input
//         .split("\n\n")
//         .map(|elf| {
//             elf.split("\n")
//                 .map(|i| i.parse::<usize>().unwrap())
//                 .sum::<usize>()
//         })
//         .sorted()
//         .rev()
//         .take(count)
//         .sum()
// }

// pub fn run(input: &str) {
//     let result1 = solve(input, 1);
//     debugln!("Part 1: {result1}");

//     let result2 = solve(input, 3);
//     debugln!("Part 2: {result2}");
// }
