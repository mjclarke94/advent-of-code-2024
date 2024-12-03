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

    let (stable, unstable): (Vec<Vec<_>>, Vec<Vec<_>>) =
        iterator(input, terminated(parse_line, newline)).partition_map(|f| match check_smoothness(
            &f,
        ) {
            true => itertools::Either::Left(f),
            false => itertools::Either::Right(f),
        });

    let stable_ish = unstable
        .into_iter()
        .map(|f| {
            (0..f.len()).any(|i| {
                let rc = [&f[0..i], &f[i + 1..]].concat();
                check_smoothness(&rc)
            })
        })
        .filter(|f| *f)
        .count();

    Some(stable.len() + stable_ish)
}

#[allow(clippy::ptr_arg)]
fn check_smoothness(values: &Vec<Size>) -> bool {
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
