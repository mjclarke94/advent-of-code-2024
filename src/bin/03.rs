#![allow(unused_assignments)]

advent_of_code::solution!(3);

use winnow::{
    ascii::dec_uint,
    combinator::{delimited, separated_pair},
    token::literal,
    Parser,
};

fn parse_mul(input: &mut &[u8]) -> winnow::PResult<u32> {
    delimited(
        literal(b"mul("),
        separated_pair(dec_uint::<_, u32, _>, b",", dec_uint::<_, u32, _>).map(|(a, b)| a * b),
        b")",
    )
    .parse_next(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.as_bytes();
    let mut tot = 0;

    while !input.is_empty() {
        if input[0] != b'm' {
            input = &input[1..];
            continue;
        }

        let mut parser_input = input;
        if let Ok(product) = parse_mul(&mut parser_input) {
            input = parser_input;
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

        let mut parser_input = input;
        if let Ok(product) = parse_mul(&mut parser_input) {
            input = parser_input;
            tot += product;
        } else {
            let mut dont_input = input;
            if literal::<_, _, ()>(b"don't()")
                .parse_next(&mut dont_input)
                .is_ok()
            {
                input = dont_input;
                break;
            }
        }

        while !input.is_empty() {
            if input[0] != b'd' {
                input = &input[1..];
                continue;
            }

            let mut do_input = input;
            if literal::<_, _, ()>(b"do()")
                .parse_next(&mut do_input)
                .is_ok()
            {
                input = do_input;
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
