advent_of_code::solution!(1);

// use ahash::RandomState;
use counter::Counter;
use rustc_hash::FxBuildHasher;

use winnow::{
    ascii::{dec_uint, newline, space1},
    combinator::{separated, separated_pair},
    Parser,
};

fn parse_line(input: &mut &[u8]) -> winnow::PResult<(u32, u32)> {
    separated_pair(dec_uint::<_, u32, _>, space1, dec_uint).parse_next(input)
}
fn parse_to_vec(input: &mut &[u8]) -> winnow::PResult<(Vec<u32>, Vec<u32>)> {
    separated(1.., parse_line, newline)
        .map(|v: Vec<_>| v.into_iter().unzip())
        .parse_next(input)
}

type UCounter = Counter<u32, u32, FxBuildHasher>;

fn parse_to_counter(input: &mut &[u8]) -> winnow::PResult<(Vec<u32>, UCounter)> {
    separated(1.., parse_line, newline)
        .map(|v: Vec<_>| v.into_iter().unzip())
        .parse_next(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.as_bytes();
    let (mut col1, mut col2) = parse_to_vec(&mut input).unwrap();

    col1.sort();
    col2.sort();

    let tot: u32 = col1.into_iter().zip(col2).map(|l| l.0.abs_diff(l.1)).sum();

    Some(tot)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.as_bytes();
    let (col1, col2) = parse_to_counter(&mut input).unwrap();

    let a: u32 = col1.iter().map(|f| f * col2[f]).sum();

    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
