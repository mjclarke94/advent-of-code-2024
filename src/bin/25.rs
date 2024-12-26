use std::ops::BitOr;

use itertools::Itertools;

advent_of_code::solution!(25);

type S = usize;
const SIXTEEN: S = 16;

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone)]
enum EntityType {
    Key,
    Lock,
}

#[derive(Debug, Clone)]
struct HeightMap {
    heights: S, // octal map, sum of pair shouldn't exceed 5
    variant: EntityType,
}

impl BitOr for &HeightMap {
    type Output = bool;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut n = self.heights + rhs.heights;

        for _ in 0..5 {
            if n % 16 > 5 {
                return false;
            }
            n /= 16;
        }
        true
    }
}

impl HeightMap {
    fn from_schematic(input: &str) -> Self {
        let variant = match input.chars().nth(0) {
            Some('#') => EntityType::Lock,
            Some('.') => EntityType::Key,
            _ => panic!("Invalid schematic"),
        };

        let l: Vec<&str> = match variant {
            EntityType::Key => input.lines().rev().skip(1).collect(),
            EntityType::Lock => input.lines().skip(1).collect(),
        };

        let heights = l
            .into_iter()
            .map(|f| {
                f.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(i, f)| SIXTEEN.pow(i as u32) * (1 - ((*f - 35) / 11)) as S)
                    .sum::<S>()
            })
            .sum::<usize>();

        Self { heights, variant }
    }
}

fn parse_input(input: &str) -> (Vec<HeightMap>, Vec<HeightMap>) {
    input
        .split("\n\n")
        .map(HeightMap::from_schematic)
        .partition(|f| matches!(f.variant, EntityType::Key))
}

pub fn part_one(input: &str) -> Option<S> {
    let (keys, locks) = parse_input(input);

    Some(
        keys.into_iter()
            .cartesian_product(locks)
            .filter(|(key, lock)| key | lock)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
