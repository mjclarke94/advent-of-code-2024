advent_of_code::solution!(1);

use counter::Counter;

pub fn part_one(input: &str) -> Option<i32> {
    let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            // Split each line, parse into i32, and collect exactly two numbers
            let mut nums = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok());
            Some((nums.next()?, nums.next()?)) // Extract exactly two numbers
        })
        .unzip();

    col1.sort();
    col2.sort();

    let tot: i32 = col1.into_iter().zip(col2).map(|l| (l.0 - l.1).abs()).sum();

    Some(tot)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (col1, col2): (Counter<_>, Counter<_>) = input
        .lines()
        .filter_map(|line| {
            // Split each line, parse into i32, and collect exactly two numbers
            let mut nums = line
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok());
            Some((nums.next()?, nums.next()?)) // Extract exactly two numbers
        })
        .unzip();

    let a: usize = col1.into_iter().map(|f| f.0 * f.1 * col2[&f.0]).sum();
    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
