use itertools::Itertools;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let levels: Vec<Level> = input.lines().map(Level::from_str).collect();
    let l: usize = levels
        .iter()
        .map(|f| f.check_undamped())
        .filter(|f| *f)
        .count();
    Some(l)
}

pub fn part_two(input: &str) -> Option<usize> {
    let levels: Vec<Level> = input.lines().map(Level::from_str).collect();
    let l: usize = levels
        .iter()
        .map(|f| f.check_damped())
        .filter(|f| *f)
        .count();
    Some(l)
}

fn check_smoothness(values: &[i32]) -> bool {
    let check_ascending = values.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3);
    let check_descending = values.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3);
    check_ascending || check_descending
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

    fn check_undamped(&self) -> bool {
        check_smoothness(&self.values)
    }

    fn check_damped(&self) -> bool {
        let l = self.values.len();
        self.values
            .clone()
            .into_iter()
            .combinations(l - 1)
            .any(|damped| check_smoothness(&damped))
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
        assert_eq!(result, Some(4));
    }
}
