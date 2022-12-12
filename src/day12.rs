use std::collections::VecDeque;

#[allow(unused)]
use crate::prelude::*;

fn solve(
    heightmap: &Grid<char>,
    start: (usize, usize),
    can_jump: impl Fn((usize, usize), (usize, usize)) -> bool,
    done: impl Fn((usize, usize)) -> bool,
) -> usize {
    let mut visited: Grid<bool> = grid![false; heightmap.width(), heightmap.height()];
    let mut horizon = VecDeque::from([(0, start)]);

    while let Some((cost, vertex)) = horizon.pop_front() {
        if done(vertex) {
            return cost;
        }

        if visited[vertex] {
            continue;
        }
        visited[vertex] = true;

        for neighbour in heightmap.neighbours_orthogonal(vertex) {
            if can_jump(vertex, neighbour) {
                horizon.push_back(((cost + 1), neighbour));
            }
        }
    }

    unreachable!()
}

fn can_jump_to(heightmap: &Grid<char>, from: (usize, usize), to: (usize, usize)) -> bool {
    (heightmap[to] as u8).saturating_sub(heightmap[from] as u8) <= 1
}

pub fn run(input: &str) -> (Solution, Solution) {
    let mut heightmap: Grid<char> = input.lines().map(|line| line.chars()).collect();

    let mut start = None;
    let mut end = None;

    for y in 0..heightmap.height() {
        for x in 0..heightmap.width() {
            if heightmap[(x, y)] == 'S' {
                start = Some((x, y));
                heightmap[(x, y)] = 'a';
            } else if heightmap[(x, y)] == 'E' {
                end = Some((x, y));
                heightmap[(x, y)] = 'z';
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let result1 = solve(
        &heightmap,
        start,
        |vertex, neighbour| can_jump_to(&heightmap, vertex, neighbour),
        |vertex| vertex == end,
    );

    // For part 2, just find the shortest path from the end to a vertex with height 'a'
    // This requires flipping the jump condition
    let result2 = solve(
        &heightmap,
        end,
        |vertex, neighbour| can_jump_to(&heightmap, neighbour, vertex),
        |vertex| heightmap[vertex] == 'a',
    );

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
