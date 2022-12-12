#[allow(unused)]
use crate::prelude::*;

fn part_1(
    trees: &Grid<usize>,
    x_iterator: impl IntoIterator<Item = usize> + Clone,
    y_iterator: impl IntoIterator<Item = usize>,
    can_see_directions: &mut Grid<usize>,
) {
    let mut tallest_by_x: Vec<Option<usize>> = vec![None; trees.width()];

    for y in y_iterator {
        let mut tallest_by_y: Option<usize> = None;

        for x in x_iterator.clone() {
            match tallest_by_x[x] {
                Some(tallest) if tallest >= trees[(x, y)] => {
                    can_see_directions[(x, y)] -= 1;
                }
                _ => tallest_by_x[x] = Some(trees[(x, y)]),
            }

            match tallest_by_y {
                Some(tallest) if tallest >= trees[(x, y)] => {
                    can_see_directions[(x, y)] -= 1;
                }
                _ => tallest_by_y = Some(trees[(x, y)]),
            }
        }
    }
}

pub fn run(input: &str) -> (Solution, Solution) {
    let trees: Grid<usize> = input
        .lines()
        .map(|row| row.chars().map(|c| c.to_string().parse::<usize>().unwrap()))
        .collect();

    let result1 = {
        let mut can_see_directions = grid![4usize; trees.width(), trees.height()];

        part_1(
            &trees,
            0..trees.width(),
            0..trees.height(),
            &mut can_see_directions,
        );
        part_1(
            &trees,
            (0..trees.width()).rev(),
            (0..trees.height()).rev(),
            &mut can_see_directions,
        );

        can_see_directions
            .into_flat_iter()
            .filter(|&tree| tree > 0)
            .count()
    };

    let mut can_see = grid![1usize; trees.width(), trees.height()];

    let mut stack = vec![];

    for x in 0..trees.width() {
        for y in 0..trees.height() {
            while let Some((top, _, top_y)) = stack.last().cloned() {
                if trees[(x, y)] >= top {
                    can_see[(x, top_y)] *= y - top_y;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((trees[(x, y)], x, y));
        }

        for (_, x, y) in stack.drain(..) {
            can_see[(x, y)] *= trees.height() - y - 1;
        }

        for y in (0..trees.height()).rev() {
            while let Some((top, _, top_y)) = stack.last().cloned() {
                if trees[(x, y)] >= top {
                    can_see[(x, top_y)] *= top_y - y;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((trees[(x, y)], x, y));
        }

        for (_, x, y) in stack.drain(..) {
            can_see[(x, y)] *= y;
        }
    }

    for y in 0..trees.height() {
        for x in 0..trees.width() {
            while let Some((top, top_x, _)) = stack.last().cloned() {
                if trees[(x, y)] >= top {
                    can_see[(top_x, y)] *= x - top_x;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((trees[(x, y)], x, y));
        }

        for (_, x, y) in stack.drain(..) {
            can_see[(x, y)] *= trees.width() - x - 1;
        }

        for x in (0..trees.width()).rev() {
            while let Some((top, top_x, _)) = stack.last().cloned() {
                if trees[(x, y)] >= top {
                    can_see[(top_x, y)] *= top_x - x;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((trees[(x, y)], x, y));
        }

        for (_, x, y) in stack.drain(..) {
            can_see[(x, y)] *= x;
        }
    }

    let result2 = can_see.into_flat_iter().max().unwrap();

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 8;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
