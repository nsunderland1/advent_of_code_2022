use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (Solution, Solution) {
    let mut heightmap: Grid<char> = input.lines().map(|line| line.chars()).collect();

    let mut start = None;
    let mut end = None;

    let mut a_positions = Vec::new();

    for y in 0..heightmap.height() {
        for x in 0..heightmap.width() {
            if heightmap[(x, y)] == 'S' {
                start = Some((x, y));
                heightmap[(x, y)] = 'a';
            } else if heightmap[(x, y)] == 'E' {
                end = Some((x, y));
                heightmap[(x, y)] = 'z';
            }

            if heightmap[(x, y)] == 'a' {
                a_positions.push((x, y));
            }
        }
    }

    let heightmap = heightmap;

    let mut best = usize::MAX;
    for start in a_positions {
        let end = end.unwrap();

        let mut visited: HashSet<(usize, usize)> = HashSet::default();
        let mut horizon: BinaryHeap<(Reverse<usize>, (usize, usize))> = BinaryHeap::new();

        horizon.push((Reverse(0), start));

        while let Some((Reverse(cost), vertex)) = horizon.pop() {
            if visited.contains(&vertex) {
                continue;
            }
            visited.insert(vertex);

            for neighbour in heightmap.neighbours_orthogonal(vertex) {
                if (heightmap[neighbour] as u8).saturating_sub(heightmap[vertex] as u8) <= 1 {
                    horizon.push((Reverse(cost + 1), neighbour));
                }
            }

            if vertex == end {
                best = best.min(cost);
                break;
            }
        }
    }

    let result1 = 0;
    let result2 = { best };

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
