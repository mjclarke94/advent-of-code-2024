advent_of_code::solution!(21);
use itertools::repeat_n;
use itertools::Itertools;
use num_enum::IntoPrimitive;
use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

type S = u32;

trait PathsFromKeys: Sized + Into<u8> + Clone {
    const WIDTH: u8 = 3;

    fn get_seq(self, other: Self) -> Vec<DirectionKey> {
        let (sx, sy) = self.get_xy();
        let (fx, fy) = other.get_xy();

        let mut out = match fx.cmp(&sx) {
            Ordering::Equal => vec![],
            Ordering::Greater => [DirectionKey::Right].repeat((fx - sx).into()),
            Ordering::Less => [DirectionKey::Left].repeat((sx - fx).into()),
        };

        let v = match fy.cmp(&sy) {
            Ordering::Equal => vec![],
            Ordering::Greater => [DirectionKey::Up].repeat((fy - sy).into()),
            Ordering::Less => [DirectionKey::Down].repeat((sy - fy).into()),
        };

        out.extend(v);

        out.sort();
        out
    }

    fn get_xy(self) -> (u8, u8) {
        let n: u8 = self.clone().into();

        (n % Self::WIDTH, n / Self::WIDTH)
    }
}

#[derive(IntoPrimitive, Debug, Clone)]
#[repr(u8)]
enum NumericKey {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    Null,
    Zero,
    A,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
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
            _ => Self::Null,
        }
    }
}

#[derive(IntoPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum DirectionKey {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    Left,
    Down,
    Right,
    Null,
    Up,
    A,
}

impl PartialOrd for DirectionKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self, other) {
            (DirectionKey::Right, _) => Some(Ordering::Less),
            (_, DirectionKey::Right) => Some(Ordering::Greater),
            (DirectionKey::Up, _) => Some(Ordering::Less),
            (_, DirectionKey::Up) => Some(Ordering::Greater),
            (DirectionKey::Down, _) => Some(Ordering::Less),
            (_, DirectionKey::Down) => Some(Ordering::Greater),
            (DirectionKey::Left, _) => Some(Ordering::Less),
            (_, DirectionKey::Left) => Some(Ordering::Greater),
            (DirectionKey::A, _) => Some(Ordering::Less),
            (_, DirectionKey::A) => Some(Ordering::Greater),
            (_, _) => Some(Ordering::Less),
        }
    }
}

impl Ord for DirectionKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PathsFromKeys for DirectionKey {}

impl PathsFromKeys for NumericKey {}

fn parse_input(s: &str) -> (u32, Vec<NumericKey>) {
    let mut v: Vec<NumericKey> = vec![NumericKey::A];
    v.extend(s.chars().map(|c| c.into()));

    let n: S = s
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();

    (n, v)
}

fn step_back<T: PathsFromKeys>(layer: Vec<T>) -> Vec<DirectionKey> {
    let v: Vec<Vec<DirectionKey>> = layer
        .into_iter()
        .tuple_windows()
        .map(|(l, r)| l.get_seq(r))
        .collect();

    let a = repeat_n(vec![DirectionKey::A], 1000);

    a.interleave_shortest(v).flatten().collect()
}

pub fn part_one(input: &str) -> Option<S> {
    let (values, inputs): (Vec<u32>, Vec<Vec<NumericKey>>) =
        input.lines().map(parse_input).collect();

    let mut v: Vec<Vec<DirectionKey>> = inputs.into_iter().map(step_back).collect();

    v = v.into_iter().map(step_back).collect();

    v = v.into_iter().map(step_back).collect();

    let a: Vec<S> = v
        .into_iter()
        .map(|f| (f.len() - 1) as S)
        .zip(&values)
        .map(|(a, b)| a * *b as S)
        .collect();

    for ((t, v), ans) in
        values
            .iter()
            .zip(&a)
            .zip(vec![68 * 29, 60 * 980, 68 * 179, 64 * 456, 64 * 379])
    {
        println!("{} - {} - {ans}", t, v)
    }

    Some(a.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
