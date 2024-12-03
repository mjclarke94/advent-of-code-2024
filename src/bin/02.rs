use itertools::Itertools;
advent_of_code::solution!(2);

use nom::{
    character::complete::{newline, space1, u8},
    combinator::iterator,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

type Size = u8;

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<Size>, ()> {
    separated_list1(space1, u8)(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.as_bytes();
    let levels = iterator(input, terminated(parse_line, newline))
        .filter(check_smoothness)
        .count();

    Some(levels)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.as_bytes();

    Some(
        iterator(input, terminated(parse_line, newline))
            .map(|f| {
                let l = f.len() - 1;
                f.into_iter().combinations(l).any(|f| check_smoothness(&f))
            })
            .filter(|f| *f)
            .count(),
    )
}

#[allow(clippy::ptr_arg)]
fn check_smoothness(values: &Vec<u8>) -> bool {
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
