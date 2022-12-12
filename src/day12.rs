use std::collections::VecDeque;

#[allow(unused)]
use crate::prelude::*;

fn solve(
    heightmap: &Grid<char>,
    mut horizon: VecDeque<(usize, (usize, usize))>,
    end: (usize, usize),
) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::default();

    while let Some((cost, vertex)) = horizon.pop_front() {
        if vertex == end {
            return cost;
        }

        if visited.contains(&vertex) {
            continue;
        }
        visited.insert(vertex);

        for neighbour in heightmap.neighbours_orthogonal(vertex) {
            if (heightmap[neighbour] as u8).saturating_sub(heightmap[vertex] as u8) <= 1 {
                horizon.push_back(((cost + 1), neighbour));
            }
        }
    }

    unreachable!()
}

pub fn run(input: &str) -> (Solution, Solution) {
    let mut heightmap: Grid<char> = input.lines().map(|line| line.chars()).collect();

    let mut end = None;

    let mut part_1_start = VecDeque::new();
    let mut part_2_start = VecDeque::new();

    for y in 0..heightmap.height() {
        for x in 0..heightmap.width() {
            if heightmap[(x, y)] == 'S' {
                part_1_start.push_back((0, (x, y)));
                heightmap[(x, y)] = 'a';
            } else if heightmap[(x, y)] == 'E' {
                end = Some((x, y));
                heightmap[(x, y)] = 'z';
            }

            if heightmap[(x, y)] == 'a' {
                part_2_start.push_back((0, (x, y)));
            }
        }
    }

    let end = end.unwrap();

    let result1 = solve(&heightmap, part_1_start, end);
    let result2 = solve(&heightmap, part_2_start, end);

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 12;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
