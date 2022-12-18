use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

#[allow(unused)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Fill {
    Air,
    Water,
    Lava,
}

struct Space {
    min: (isize, isize, isize),
    max: (isize, isize, isize),
    space: Vec<Fill>,
}

impl Space {
    fn x_span(&self) -> usize {
        (self.max.0 - self.min.0 + 1) as usize
    }

    fn y_span(&self) -> usize {
        (self.max.1 - self.min.1 + 1) as usize
    }

    fn z_span(&self) -> usize {
        (self.max.2 - self.min.2 + 1) as usize
    }

    fn len(&self) -> usize {
        self.space.len()
    }

    fn new(min: (isize, isize, isize), max: (isize, isize, isize)) -> Self {
        let mut this = Self {
            min,
            max,
            space: Vec::new(),
        };

        this.space
            .resize(this.x_span() * this.y_span() * this.z_span(), Fill::Air);

        this
    }

    fn neighbours(
        &self,
        (x, y, z): (isize, isize, isize),
    ) -> impl Iterator<Item = (isize, isize, isize)> {
        let (min_x, min_y, min_z) = self.min;
        let (max_x, max_y, max_z) = self.max;

        [-1, 1]
            .into_iter()
            .flat_map(move |shift| [(x + shift, y, z), (x, y + shift, z), (x, y, z + shift)])
            .filter(move |(x, y, z)| {
                (min_x..=max_x).contains(&x)
                    && (min_y..=max_y).contains(&y)
                    && (min_z..=max_z).contains(&z)
            })
    }
}

impl Index<(isize, isize, isize)> for Space {
    type Output = Fill;

    fn index(&self, (x, y, z): (isize, isize, isize)) -> &Self::Output {
        let (min_x, min_y, min_z) = self.min;
        let x = (x - min_x) as usize;
        let y = (y - min_y) as usize;
        let z = (z - min_z) as usize;
        let x_span = self.x_span();
        let y_span = self.y_span();

        &self.space[z * y_span * x_span + y * x_span + x]
    }
}

impl IndexMut<(isize, isize, isize)> for Space {
    fn index_mut(&mut self, (x, y, z): (isize, isize, isize)) -> &mut Self::Output {
        let (min_x, min_y, min_z) = self.min;
        let x = (x - min_x) as usize;
        let y = (y - min_y) as usize;
        let z = (z - min_z) as usize;
        let x_span = self.x_span();
        let y_span = self.y_span();

        &mut self.space[z * y_span * x_span + y * x_span + x]
    }
}

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

    let min_x = coords.iter().copied().map(|(x, _, _)| x).min().unwrap() - 1;
    let min_y = coords.iter().copied().map(|(_, y, _)| y).min().unwrap() - 1;
    let min_z = coords.iter().copied().map(|(_, _, z)| z).min().unwrap() - 1;
    let min = (min_x, min_y, min_z);

    let max_x = coords.iter().copied().map(|(x, _, _)| x).max().unwrap() + 1;
    let max_y = coords.iter().copied().map(|(_, y, _)| y).max().unwrap() + 1;
    let max_z = coords.iter().copied().map(|(_, _, z)| z).max().unwrap() + 1;
    let max = (max_x, max_y, max_z);

    let mut space = Space::new(min, max);

    for coord in coords.iter().copied() {
        space[coord] = Fill::Lava;
    }

    let result1 = coords
        .iter()
        .copied()
        .map(|coord| {
            space
                .neighbours(coord)
                .filter(|&neighbour| space[neighbour] != Fill::Lava)
                .count()
        })
        .sum::<usize>();

    let mut queue = VecDeque::with_capacity(space.len());
    queue.push_back(min);

    let mut result2 = 0;

    while let Some(coord) = queue.pop_front() {
        for neighbour in space.neighbours(coord) {
            match space[neighbour] {
                Fill::Lava => result2 += 1,
                Fill::Air => {
                    queue.push_back(neighbour);
                    space[neighbour] = Fill::Water;
                }
                Fill::Water => (),
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
