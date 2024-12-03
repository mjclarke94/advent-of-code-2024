advent_of_code::solution!(1);

// use ahash::RandomState;
use counter::Counter;
use rustc_hash::FxBuildHasher;

use nom::{
    character::complete::{newline, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse_line(input: &[u8]) -> nom::IResult<&[u8], (u32, u32)> {
    separated_pair(u32, space1, u32)(input)
}
fn parse_to_vec(input: &[u8]) -> nom::IResult<&[u8], (Vec<u32>, Vec<u32>)> {
    map(separated_list1(newline, parse_line), |v| {
        v.into_iter().unzip()
    })(input)
}

type UCounter = Counter<u32, u32, FxBuildHasher>;

fn parse_to_counter(input: &[u8]) -> nom::IResult<&[u8], (Vec<u32>, UCounter)> {
    map(separated_list1(newline, parse_line), |v| {
        v.into_iter().unzip()
    })(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let (mut col1, mut col2) = parse_to_vec(input).unwrap().1;

    col1.sort();
    col2.sort();

    let tot: u32 = col1.into_iter().zip(col2).map(|l| l.0.abs_diff(l.1)).sum();

    Some(tot)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let (col1, col2) = parse_to_counter(input).unwrap().1;

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
