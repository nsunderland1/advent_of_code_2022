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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

fn shift_left(piece: Vec<(usize, usize)>, chamber: &[u8]) -> Vec<(usize, usize)> {
    if let Some(next_piece) = piece
        .iter()
        .copied()
        .map(|(x, y)| {
            let x_next = x.checked_sub(1)?;

            if chamber[y] & (1 << x_next) == 0 {
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

fn shift_right(piece: Vec<(usize, usize)>, chamber: &[u8]) -> Vec<(usize, usize)> {
    if let Some(next_piece) = piece
        .iter()
        .copied()
        .map(|(x, y)| {
            let x_next = x.checked_add(1)?;

            if x_next < 7 && chamber[y] & (1 << x_next) == 0 {
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
    chamber: &[u8],
) -> Result<Vec<(usize, usize)>, Vec<(usize, usize)>> {
    if let Some(next_piece) = piece
        .iter()
        .copied()
        .map(|(x, y)| {
            let y_next = y.checked_sub(1)?;

            if chamber[y_next] & (1 << x) == 0 {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Key {
    piece: Piece,
    jet_index: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Value {
    height: usize,
    piece_index: usize,
}

pub fn run(input: &str) -> (Solution, Solution) {
    let shifts = parse_input(input).unwrap().1;
    let mut jets = shifts.iter().copied().enumerate().cycle();

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

    const NUM_PIECES: usize = 2022;

    // In the worst case, the tallest piece is 4 units tall, and has an initial gap of 3
    let mut chamber: Vec<u8> = vec![0; 7 * NUM_PIECES];

    let mut cache = HashMap::<Key, Vec<Value>>::default();

    let result1 = {
        for (piece_index, piece) in pieces.take(NUM_PIECES).enumerate() {
            let mut squares = piece.initial_squares(height);

            let piece_index = piece_index + 1;
            let mut final_jet_index = None;

            for (i, shift) in jets.by_ref() {
                final_jet_index = Some(i);

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

            cache
                .entry(Key {
                    piece,
                    jet_index: final_jet_index.unwrap(),
                })
                .or_insert_with(Vec::new)
                .push(Value {
                    height,
                    piece_index,
                });

            for (x, y) in squares {
                chamber[y] |= 1 << x;
            }
        }

        height
    };

    const PART_2_NUM_PIECES: usize = 1_000_000_000_000;

    let result2 = cache
        .into_iter()
        .find_map(|(_, values)| {
            if values.len() < 2 {
                return None;
            };

            let step = values[1].piece_index - values[0].piece_index;
            let start = values[0].piece_index;

            if (PART_2_NUM_PIECES - start) % step == 0 {
                let start_height = values[0].height;
                let step_height = values[1].height - values[0].height;

                let num_steps = (PART_2_NUM_PIECES - start) / step;
                Some(start_height + num_steps * step_height)
            } else {
                None
            }
        })
        .unwrap();

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
