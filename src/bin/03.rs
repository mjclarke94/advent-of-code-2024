#![allow(unused_assignments)]

advent_of_code::solution!(3);

use nom::{
    bytes::complete::tag,
    character::complete::u32,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

fn parse_mul(input: &[u8]) -> IResult<&[u8], u32, ()> {
    map(
        delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")")),
        |(a, b)| a * b,
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.as_bytes();
    let mut tot = 0;

    while !input.is_empty() {
        if input[0] != b'm' {
            input = &input[1..];
            continue;
        }

        if let Ok((remainder, product)) = parse_mul(input) {
            input = remainder;
            tot += product;
        } else {
            input = &input[1..];
        }
    }

    Some(tot)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.as_bytes();
    let mut tot = 0;

    while !input.is_empty() {
        if input[0] != b'm' {
            input = &input[1..];
            continue;
        }

        if let Ok((remainder, product)) = parse_mul(input) {
            input = remainder;
            tot += product;
        } else if let Ok((remainder, _)) = tag::<_, _, ()>(b"don't()")(input) {
            input = remainder;
            break;
        }

        while !input.is_empty() {
            if input[0] != b'd' {
                input = &input[1..];
                continue;
            }

            if let Ok((remainder, _)) = tag::<_, _, ()>(b"do()")(input) {
                input = remainder;
                break;
            } else {
                input = &input[1..];
            }
        }
    }

    Some(tot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
