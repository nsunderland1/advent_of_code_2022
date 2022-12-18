use std::collections::VecDeque;

#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (Solution, Solution) {
    let coords: HashSet<(isize, isize, isize)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let result1 = coords
        .iter()
        .copied()
        .map(|(x, y, z)| {
            let mut surface_area = 0;
            for shift in [-1, 1] {
                if !coords.contains(&(x + shift, y, z)) {
                    surface_area += 1;
                }

                if !coords.contains(&(x, y + shift, z)) {
                    surface_area += 1;
                }

                if !coords.contains(&(x, y, z + shift)) {
                    surface_area += 1;
                }
            }
            surface_area
        })
        .sum::<usize>();

    let min_x = coords.iter().copied().map(|(x, _, _)| x).min().unwrap() - 1;
    let min_y = coords.iter().copied().map(|(_, y, _)| y).min().unwrap() - 1;
    let min_z = coords.iter().copied().map(|(_, _, z)| z).min().unwrap() - 1;
    let min = (min_x, min_y, min_z);

    let max_x = coords.iter().copied().map(|(x, _, _)| x).max().unwrap() + 1;
    let max_y = coords.iter().copied().map(|(_, y, _)| y).max().unwrap() + 1;
    let max_z = coords.iter().copied().map(|(_, _, z)| z).max().unwrap() + 1;
    let max = (max_x, max_y, max_z);

    let mut reachable = HashSet::default();

    let neighbours = |(x, y, z)| {
        [-1, 1].into_iter().flat_map(move |shift| {
            [(x + shift, y, z), (x, y + shift, z), (x, y, z + shift)]
                .into_iter()
                .filter(move |neighbour| {
                    (min.0..=max.0).contains(&neighbour.0)
                        && (min.1..=max.1).contains(&neighbour.1)
                        && (min.2..=max.2).contains(&neighbour.2)
                })
        })
    };

    let mut queue = VecDeque::new();
    queue.push_back(min);
    reachable.insert(min);

    let mut result2 = 0;

    while let Some(coord) = queue.pop_front() {
        for neighbour in neighbours(coord) {
            if coords.contains(&neighbour) {
                result2 += 1;
            } else if !reachable.contains(&neighbour) {
                queue.push_back(neighbour);
                reachable.insert(neighbour);
            }
        }
    }

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 18;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
