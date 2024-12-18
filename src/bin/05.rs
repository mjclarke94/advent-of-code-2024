use rustc_hash::FxBuildHasher;
use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(5);

type S = u16;

impl FromStr for Manual {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pages: s.split(',').map(|f| f.parse().unwrap()).collect(),
        })
    }
}
#[derive(Debug)]
struct Manual {
    pages: Vec<S>,
}

pub fn part_one(input: &str) -> Option<S> {
    let (input, rules) = parse_rules(input);
    let manuals: Vec<Manual> = input
        .lines()
        .map(|f| Manual::from_str(f).unwrap())
        .collect();

    let a: S = manuals
        .into_iter()
        .filter(|f| f.pages.is_sorted_by(|l, r| !rules.contains(&(*r, *l))))
        .map(|f| f.pages[f.pages.len() / 2])
        .sum();

    Some(a)
}

pub fn part_two(input: &str) -> Option<S> {
    let (input, rules) = parse_rules(input);
    let mut manuals: Vec<Manual> = input
        .lines()
        .map(|f| Manual::from_str(f).unwrap())
        .collect();

    let a: S = manuals
        .iter_mut()
        .filter(|f| !f.pages.is_sorted_by(|l, r| !rules.contains(&(*r, *l))))
        .map(|f| {
            f.pages.sort_by(|l, r| match rules.contains(&(*r, *l)) {
                false => std::cmp::Ordering::Greater,
                true => std::cmp::Ordering::Less,
            });
            f.pages[f.pages.len() / 2]
        })
        .sum();

    Some(a)
}

fn parse_rules(s: &str) -> (&str, HashSet<(S, S), FxBuildHasher>) {
    if let Some((l, r)) = s.split_once("\n\n") {
        let rules: HashSet<(S, S), FxBuildHasher> = l
            .lines()
            .map(|f| {
                let (a, b) = f.split_once('|').unzip();
                (a.unwrap().parse().unwrap(), b.unwrap().parse().unwrap())
            })
            .collect();
        (r, rules)
    } else {
        panic!("Oops")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
