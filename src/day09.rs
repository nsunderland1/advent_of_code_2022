#[allow(unused)]
use crate::prelude::*;

fn update_position(head_pos: (i32, i32), tail_pos: &mut (i32, i32)) {
    if head_pos.0 == tail_pos.0 && head_pos.1.abs_diff(tail_pos.1) > 1 {
        tail_pos.1 += (head_pos.1 - tail_pos.1).signum();
    } else if head_pos.1 == tail_pos.1 && head_pos.0.abs_diff(tail_pos.0) > 1 {
        tail_pos.0 += (head_pos.0 - tail_pos.0).signum();
    } else if head_pos.0.abs_diff(tail_pos.0) > 1 || head_pos.1.abs_diff(tail_pos.1) > 1 {
        tail_pos.1 += (head_pos.1 - tail_pos.1).signum();
        tail_pos.0 += (head_pos.0 - tail_pos.0).signum();
    }
}

pub fn run(input: &str) -> (Solution, Solution) {
    const LAST: usize = 9;
    let mut seen_second = HashSet::default();
    let mut seen_last = HashSet::default();
    let mut knots = [(0i32, 0i32); LAST + 1];

    seen_second.insert(knots[1]);
    seen_last.insert(knots[LAST]);

    for step in input.lines() {
        let (dir, count) = step.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();

        let shift = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..count {
            knots[0].0 += shift.0;
            knots[0].1 += shift.1;

            for i in 0..LAST {
                let (left, right) = knots.split_at_mut(i + 1);
                let head = left[i];
                let tail = &mut right[0];

                update_position(head, tail);
            }
            seen_second.insert(knots[1]);
            seen_last.insert(knots[LAST]);
        }
    }

    (seen_second.len().into(), seen_last.len().into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 9;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
