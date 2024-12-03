use itertools::Itertools;
advent_of_code::solution!(2);

use nom::{
    character::complete::{newline, space1, u32},
    multi::separated_list1,
    IResult,
};

type Level = Vec<u32>;

fn parse_line(input: &[u8]) -> IResult<&[u8], Level> {
    separated_list1(space1, u32)(input)
}

fn parse_file(input: &[u8]) -> IResult<&[u8], Vec<Level>> {
    (separated_list1(newline, parse_line))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.as_bytes();
    let levels = parse_file(input).ok()?.1;

    Some(levels.into_iter().filter(check_smoothness).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.as_bytes();
    let levels = parse_file(input).ok()?.1;

    Some(
        levels
            .into_iter()
            .map(|f| {
                let l = f.len() - 1;
                f.into_iter().combinations(l).any(|f| check_smoothness(&f))
            })
            .filter(|f| *f)
            .count(),
    )
}

#[allow(clippy::ptr_arg)]
fn check_smoothness(values: &Vec<u32>) -> bool {
    let check_ascending = values.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3);
    let check_descending = values.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3);
    check_ascending || check_descending
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
