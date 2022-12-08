#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (Solution, Solution) {
    let trees: Grid<usize> = input
        .lines()
        .map(|row| row.chars().map(|c| c.to_string().parse::<usize>().unwrap()))
        .collect();

    let mut can_see_directions = grid![4usize; trees.width(), trees.height()];

    for x in 0..trees.width() {
        let mut tallest = None;

        for y in 0..trees.height() {
            match tallest {
                Some(tallest) if tallest >= trees[(x, y)] => {
                    can_see_directions[(x, y)] -= 1;
                }
                _ => (),
            }

            tallest = Some(tallest.unwrap_or(0).max(trees[(x, y)]));
        }

        tallest = None;

        for y in (0..trees.height()).rev() {
            match tallest {
                Some(tallest) if tallest >= trees[(x, y)] => {
                    can_see_directions[(x, y)] -= 1;
                }
                _ => (),
            }

            tallest = Some(tallest.unwrap_or(0).max(trees[(x, y)]));
        }
    }

    for y in 0..trees.height() {
        let mut tallest = None;

        for x in 0..trees.width() {
            match tallest {
                Some(tallest) if tallest >= trees[(x, y)] => {
                    can_see_directions[(x, y)] -= 1;
                }
                _ => (),
            }

            tallest = Some(tallest.unwrap_or(0).max(trees[(x, y)]));
        }

        tallest = None;

        for x in (0..trees.width()).rev() {
            match tallest {
                Some(tallest) if tallest >= trees[(x, y)] => {
                    can_see_directions[(x, y)] -= 1;
                }
                _ => (),
            }

            tallest = Some(tallest.unwrap_or(0).max(trees[(x, y)]));
        }
    }

    let result1 = can_see_directions
        .into_flat_iter()
        .filter(|&tree| tree > 0)
        .count();

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
