use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};

#[allow(unused)]
use crate::prelude::*;

#[derive(Clone, Copy)]
enum Shift {
    Left,
    Right,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Shift>> {
    many1(alt((
        value(Shift::Left, tag("<")),
        value(Shift::Right, tag(">")),
    )))(input)
}

#[derive(Clone, Copy)]
enum Piece {
    Horizontal,
    Plus,
    Angle,
    Vertical,
    Square,
}

impl Piece {
    fn initial_squares(self, height: usize) -> Vec<(usize, usize)> {
        let squares = match self {
            Piece::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Piece::Plus => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Piece::Angle => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Piece::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Piece::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        };

        squares
            .into_iter()
            .map(|(x, y)| (x + 2, y + height + 4))
            .collect()
    }
}

fn shift_left(piece: Vec<(usize, usize)>, chamber: &[Vec<bool>]) -> Vec<(usize, usize)> {
    if let Some(next_piece) = piece
        .iter()
        .copied()
        .map(|(x, y)| {
            let x_next = x.checked_sub(1)?;

            if !chamber[x_next][y] {
                Some((x_next, y))
            } else {
                None
            }
        })
        .collect()
    {
        next_piece
    } else {
        piece
    }
}

fn shift_right(piece: Vec<(usize, usize)>, chamber: &[Vec<bool>]) -> Vec<(usize, usize)> {
    if let Some(next_piece) = piece
        .iter()
        .copied()
        .map(|(x, y)| {
            let x_next = x.checked_add(1)?;

            if x_next < chamber.len() && !chamber[x_next][y] {
                Some((x_next, y))
            } else {
                None
            }
        })
        .collect()
    {
        next_piece
    } else {
        piece
    }
}

fn shift_down(
    piece: Vec<(usize, usize)>,
    chamber: &[Vec<bool>],
) -> Result<Vec<(usize, usize)>, Vec<(usize, usize)>> {
    if let Some(next_piece) = piece
        .iter()
        .copied()
        .map(|(x, y)| {
            let y_next = y.checked_sub(1)?;

            if !chamber[x][y_next] {
                Some((x, y_next))
            } else {
                None
            }
        })
        .collect()
    {
        Ok(next_piece)
    } else {
        Err(piece)
    }
}

pub fn run(input: &str) -> (Solution, Solution) {
    let shifts = parse_input(input).unwrap().1;
    let mut jets = shifts.iter().copied().cycle();

    let pieces = [
        Piece::Horizontal,
        Piece::Plus,
        Piece::Angle,
        Piece::Vertical,
        Piece::Square,
    ]
    .into_iter()
    .cycle();

    let mut height = 0;
    let mut chamber: Vec<Vec<bool>> = vec![Vec::new(); 7];

    const NUM_PIECES: usize = 2022;

    let result1 = {
        for piece in pieces.take(NUM_PIECES) {
            for column in chamber.iter_mut() {
                column.resize(height + 8, false); // 3 gap + tallest piece has height 4
            }

            let mut squares = piece.initial_squares(height);

            for shift in jets.by_ref() {
                squares = match shift {
                    Shift::Left => shift_left(squares, &chamber),
                    Shift::Right => shift_right(squares, &chamber),
                };

                match shift_down(squares, &chamber) {
                    Ok(next_squares) => squares = next_squares,
                    Err(old_squares) => {
                        squares = old_squares;
                        break;
                    }
                }
            }

            height = height.max(squares.iter().map(|&(_, y)| y).max().unwrap());

            for (x, y) in squares {
                chamber[x][y] = true;
            }
        }

        height
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
        const DAY: u32 = 17;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
