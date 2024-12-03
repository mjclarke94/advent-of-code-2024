advent_of_code::solution!(3);

use itertools::Either::{self, Left, Right};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::u32,
    combinator::{map, value},
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn parse_mul(input: &[u8]) -> IResult<&[u8], u32, ()> {
    map(
        delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")")),
        |(a, b)| a * b,
    )(input)
}

fn parse_one(input: &[u8]) -> IResult<&[u8], u32, ()> {
    map(many1(alt((parse_mul, value(0, take(1u8))))), |v| {
        v.into_iter().sum()
    })(input)
}

fn parse_two(input: &[u8]) -> IResult<&[u8], Vec<Either<u32, bool>>, ()> {
    many1(alt((
        map(parse_mul, Left),
        value(Right(true), tag("do()")),
        value(Right(false), tag("don't()")),
        value(Left(0), take(1u8)),
    )))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let product = parse_one(input).unwrap().1;
    Some(product)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let instructions = parse_two(input).unwrap().1;

    let mut _do = true;
    let mut acc = 0;

    for instr in instructions.into_iter() {
        match instr {
            Right(switch) => _do = switch,
            Left(a) if _do => acc += a,
            _ => (),
        }
    }

    Some(acc)
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
