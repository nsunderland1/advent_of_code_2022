#[allow(unused)]
use crate::prelude::*;

enum Instruction {
    Noop,
    Add(i32),
}

pub fn run(input: &str) -> (Solution, Solution) {
    let instructions = input.lines().map(|line| match line {
        "noop" => Instruction::Noop,
        _ => {
            let (_, value) = line.split_once(' ').unwrap();
            Instruction::Add(value.parse().unwrap())
        }
    });

    let mut total = 0;
    let mut register = 1;
    let mut cycle = 0;
    let mut row = [' '; 40];

    for instruction in instructions {
        let num_cycles = match instruction {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        };

        for _ in 0..num_cycles {
            cycle += 1;

            let mod_cycle: usize = (cycle - 1) % 40;
            if (mod_cycle as i64).abs_diff(register as i64) <= 1 && (0i32..40).contains(&register) {
                row[mod_cycle] = '#';
            }

            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    total += cycle as i32 * register;
                }
                40 | 80 | 120 | 160 | 200 | 240 => {
                    let line = row.iter().copied().collect::<String>();
                    println!("{line}");
                    row.fill(' ');
                }
                _ => (),
            }
        }

        match instruction {
            Instruction::Noop => (),
            Instruction::Add(val) => register += val,
        }
    }

    let result1 = total as usize;
    let result2 = 0;

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 10;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
