#[allow(unused)]
use crate::prelude::*;

pub fn run(input: &str) -> (Solution, Solution) {
    let map: Grid<_> = input.lines().map(|line| line.chars()).collect();

    let elves: HashSet<_> = (0..map.height())
        .cartesian_product(0..map.width())
        .filter_map(|(y, x)| (map[(x, y)] == '#').then(|| (x as isize, y as isize)))
        .collect();

    let mut moves = [
        |(x, y): (isize, isize)| ((x, y - 1), [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]),
        |(x, y): (isize, isize)| ((x, y + 1), [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]),
        |(x, y): (isize, isize)| ((x - 1, y), [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]),
        |(x, y): (isize, isize)| ((x + 1, y), [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]),
    ]
    .into_iter()
    .cycle();

    let result1 = {
        let mut elves = elves.clone();

        for round in 0.. {
            // let (min_x, max_x) = elves
            //     .iter()
            //     .copied()
            //     .map(|(x, _)| x)
            //     .minmax()
            //     .into_option()
            //     .unwrap();
            // let (min_y, max_y) = elves
            //     .iter()
            //     .copied()
            //     .map(|(_, y)| y)
            //     .minmax()
            //     .into_option()
            //     .unwrap();

            // let mut map_2 = grid!['.'; (max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize];
            // for (x, y) in (min_x..=max_x).cartesian_product(min_y..=max_y) {
            //     if elves.contains(&(x, y)) {
            //         map_2[((x - min_x) as usize, (y - min_y) as usize)] = '#';
            //     }
            // }

            // println!("{map_2}");

            let mut intended_moves = HashMap::<(isize, isize), Vec<(isize, isize)>>::default();

            'elf_loop: for elf in elves.iter().copied() {
                let moves = moves.clone().take(4);

                if [-1, 0, 1]
                    .into_iter()
                    .cartesian_product([-1, 0, 1])
                    .filter(|&(x, y)| x != 0 || y != 0)
                    .any(|(x, y)| elves.contains(&(elf.0 + x, elf.1 + y)))
                {
                    for move_attempt in moves {
                        let (next, seek) = move_attempt(elf);

                        if !seek.iter().any(|seek| elves.contains(&seek)) {
                            intended_moves.entry(next).or_default().push(elf);
                            continue 'elf_loop;
                        }
                    }
                }

                assert!(intended_moves.insert(elf, vec![elf]).is_none());
            }

            let mut next_elves = HashSet::default();

            for (position, moving_elves) in intended_moves {
                match moving_elves.as_slice() {
                    [_single_elf] => {
                        next_elves.insert(position);
                    }
                    _ => {
                        for elf in moving_elves {
                            next_elves.insert(elf);
                        }
                    }
                }
            }

            if elves == next_elves {
                panic!("Got the answer and it was {round}", round = round + 1);
            }

            elves = next_elves;
            moves.next().unwrap();
        }

        let (min_x, max_x) = elves
            .iter()
            .copied()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = elves
            .iter()
            .copied()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();

        dbg!(&(min_x, max_x));
        dbg!(&(min_y, max_y));
        dbg!(elves.iter().count());

        dbg!(&elves);

        let mut map_2 = grid!['.'; (max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize];
        for (x, y) in (min_x..=max_x).cartesian_product(min_y..=max_y) {
            if elves.contains(&(x, y)) {
                map_2[((x - min_x) as usize, (y - min_y) as usize)] = '#';
            }
        }

        println!("{map_2}");

        ((max_x - min_x + 1) * (max_y - min_y + 1) - elves.into_iter().count() as isize) as usize
    };

    let result2 = {
        // Part 2
        0
    };

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 23;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
