#[allow(unused)]
use itertools::Itertools;

fn parse_line(s: &str) -> u32 {
    s.parse().unwrap()
}

pub fn run(input: &str) {
    #[allow(unused)]
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let result1 = {
        // Part 1
        0
    };

    println!("Part 1: {}", result1);

    let result2 = {
        // Part 2
        0
    };

    println!("Part 2: {}", result2);
}
