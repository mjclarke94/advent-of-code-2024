advent_of_code::solution!(2);

use winnow::{
    ascii::{dec_uint, newline, space1},
    combinator::{repeat, separated, terminated},
    Parser,
};

type Size = u8;

fn parse_line(input: &mut &[u8]) -> winnow::PResult<Vec<Size>> {
    separated(1.., dec_uint::<_, Size, _>, space1).parse_next(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = input.as_bytes();
    let levels: Vec<Vec<Size>> = repeat(0.., terminated(parse_line, newline))
        .parse_next(&mut input)
        .unwrap();
    let levels = levels.into_iter().filter(check_smoothness).count();

    Some(levels)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = input.as_bytes();

    let levels: Vec<Vec<Size>> = repeat(0.., terminated(parse_line, newline))
        .parse_next(&mut input)
        .unwrap();
    let count = levels
        .into_iter()
        .filter(check_smoothness_with_removal)
        .count();

    Some(count)
}

#[allow(clippy::ptr_arg)]
fn check_smoothness(values: &Vec<Size>) -> bool {
    let check_ascending = values.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3);
    let check_descending = values.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3);
    check_ascending || check_descending
}

fn check_smoothness_with_removal(values: &Vec<Size>) -> bool {
    if check_smoothness(values) {
        return true;
    }

    // Find first violation in both directions
    let ascending_violation = find_violation(values, true);
    let descending_violation = find_violation(values, false);

    // Try removing the problematic element(s) for each direction
    if let Some(idx) = ascending_violation {
        // Try removing current element
        if check_after_removal(values, idx, true) {
            return true;
        }
        // Try removing previous element (if exists)
        if idx > 0 && check_after_removal(values, idx - 1, true) {
            return true;
        }
    }

    if let Some(idx) = descending_violation {
        // Try removing current element
        if check_after_removal(values, idx, false) {
            return true;
        }
        // Try removing previous element (if exists)
        if idx > 0 && check_after_removal(values, idx - 1, false) {
            return true;
        }
    }

    false
}

fn find_violation(values: &[Size], ascending: bool) -> Option<usize> {
    for i in 1..values.len() {
        let valid = if ascending {
            values[i - 1] < values[i] && values[i - 1].abs_diff(values[i]) <= 3
        } else {
            values[i - 1] > values[i] && values[i - 1].abs_diff(values[i]) <= 3
        };

        if !valid {
            return Some(i);
        }
    }
    None
}

fn check_after_removal(values: &[Size], remove_idx: usize, ascending: bool) -> bool {
    let mut prev: Option<Size> = None;

    for (i, &val) in values.iter().enumerate() {
        if i == remove_idx {
            continue;
        }

        if let Some(p) = prev {
            let valid = if ascending {
                p < val && p.abs_diff(val) <= 3
            } else {
                p > val && p.abs_diff(val) <= 3
            };

            if !valid {
                return false;
            }
        }

        prev = Some(val);
    }

    true
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
