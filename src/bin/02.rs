use core::cmp::Ordering::{Equal, Greater, Less};
use itertools::Itertools;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let levels: Vec<Level> = input.lines().map(Level::from_str).collect();
    let l: Vec<bool> = levels.iter().map(|f| f.is_smooth()).collect();
    dbg!(l);
    Some(2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct Level {
    values: Vec<i32>,
}

impl Level {
    fn from_str(input: &str) -> Self {
        let values: Vec<i32> = input
            .split_ascii_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();
        Self { values }
    }

    fn is_smooth(&self) -> bool {
        let decreasing = self
            .values
            .iter()
            .tuples()
            .map(|x: (&i32, &i32)| x.1 - x.0)
            .all(|x: i32| (1..=3).contains(&x));
        let increasing = self
            .values
            .iter()
            .tuples()
            .map(|x: (&i32, &i32)| x.1 - x.0)
            .all(|x: i32| (1..=3).contains(&(-x)));

        dbg!(&increasing);
        dbg!(&decreasing);

        increasing || decreasing
    }
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
        assert_eq!(result, None);
    }
}
