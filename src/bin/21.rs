advent_of_code::solution!(21);

use std::fmt::Debug;

use itertools::Itertools;
use memoize::memoize;
use num_enum::IntoPrimitive;

type S = usize;

fn parse_line(s: &str) -> (S, Vec<NumericKey>) {
    let mut v: Vec<NumericKey> = vec![NumericKey::A];
    v.extend(s.chars().map(NumericKey::from));

    let n: S = s
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();

    (n, v)
}

fn parse_input(input: &str) -> (Vec<S>, Vec<Vec<NumericKey>>) {
    input.lines().map(parse_line).collect()
}
#[derive(Debug)]
enum PathOrder {
    Both,
    Single,
}

trait GetCoord: Into<u8> + Debug + Clone + Copy {
    const WIDTH: u8 = 3;
    const NP: (u8, u8);

    fn get_xy(self) -> (u8, u8) {
        let n: u8 = self.into();

        (n % Self::WIDTH, n / Self::WIDTH)
    }

    fn get_paths(start: Self, end: Self) -> (Vec<DirectionKey>, PathOrder) {
        let (sx, sy) = start.get_xy();
        let (ex, ey) = end.get_xy();

        let dx = sx.abs_diff(ex);
        let dy = sy.abs_diff(ey);

        let mut v: Vec<DirectionKey> = vec![];

        // Initial array is in "universally safe" order (always avoids null)
        if ex > sx {
            v.extend(vec![DirectionKey::Right; dx as usize])
        }
        if ey > sy {
            v.extend(vec![DirectionKey::Up; dy as usize])
        }
        if sy > ey {
            v.extend(vec![DirectionKey::Down; dy as usize])
        }
        if sx > ex {
            v.extend(vec![DirectionKey::Left; dx as usize])
        }

        if (ex == sx) // Line search, no need to fan out
            | (ey == sy) // Line search, no need to fan out
            | ((Self::NP.0 == sx) & (Self::NP.1 == ey)) // Null avoidance logic
            | ((Self::NP.1 == sy) & (Self::NP.0 == ex))
        {
            (v, PathOrder::Single)
        } else {
            (v, PathOrder::Both)
        }
    }
}

#[derive(IntoPrimitive, Debug, Clone, Copy, Hash)]
#[repr(u8)]
enum NumericKey {
    _Null, // +---+---+---+
    Zero,  // | 7 | 8 | 9 |
    A,     // +---+---+---+
    One,   // | 4 | 5 | 6 |
    Two,   // +---+---+---+
    Three, // | 1 | 2 | 3 |
    Four,  // +---+---+---+
    Five,  //     | 0 | A |
    Six,   //     +---+---+
    Seven,
    Eight,
    Nine,
}

impl GetCoord for NumericKey {
    const NP: (u8, u8) = (0, 0);
}

impl From<char> for NumericKey {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::A,
            _ => Self::_Null,
        }
    }
}

#[derive(IntoPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum DirectionKey {
    Left,  //     +---+---+
    Down,  //     | ^ | A |
    Right, // +---+---+---+
    _Null, // | < | v | > |
    Up,    // +---+---+---+
    A,
}

impl GetCoord for DirectionKey {
    const NP: (u8, u8) = (0, 1);
}

#[memoize]
fn recursive_search(start: DirectionKey, end: DirectionKey, depth: u8) -> S {
    dispatch_seq(start, end, depth)
}

// Indirection needed as NumericKey and DirectionKey are different types so cannot share cache
fn numeric_length(seq: Vec<NumericKey>, depth: u8) -> usize {
    seq.into_iter()
        .tuple_windows()
        .map(|(start, end)| dispatch_seq(start, end, depth))
        .sum()
}

fn dispatch_seq<T: GetCoord>(start: T, end: T, depth: u8) -> usize {
    let (mut seq, order) = T::get_paths(start, end);
    if depth == 0 {
        seq.len() + 1
    } else {
        match order {
            PathOrder::Single => {
                seq.push(DirectionKey::A);
                seq.reverse();
                seq.push(DirectionKey::A);
                seq.reverse();

                seq.into_iter()
                    .tuple_windows()
                    .map(|(start, end)| recursive_search(start, end, depth - 1))
                    .sum()
            }
            PathOrder::Both => {
                seq.push(DirectionKey::A);
                seq.reverse();
                seq.push(DirectionKey::A);
                let backwards = seq.to_owned();

                seq.reverse();

                let forward_sum: S = seq
                    .into_iter()
                    .tuple_windows()
                    .map(|(start, end)| recursive_search(start, end, depth - 1))
                    .sum();

                let backward_sum: S = backwards
                    .into_iter()
                    .tuple_windows()
                    .map(|(start, end)| recursive_search(start, end, depth - 1))
                    .sum();
                forward_sum.min(backward_sum)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<S> {
    memoized_flush_recursive_search();
    let (values, inputs) = parse_input(input);
    let s: Vec<S> = inputs.into_iter().map(|v| numeric_length(v, 2)).collect();
    let p: S = s.into_iter().zip(values).map(|(l, r)| l * r).sum();

    Some(p)
}

pub fn part_two(input: &str) -> Option<S> {
    memoized_flush_recursive_search();
    let (values, inputs) = parse_input(input);
    let s: Vec<S> = inputs.into_iter().map(|v| numeric_length(v, 25)).collect();
    let p: S = s.into_iter().zip(values).map(|(l, r)| l * r).sum();

    Some(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
