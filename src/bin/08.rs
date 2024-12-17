advent_of_code::solution!(8);
use itertools::Itertools;
use std::{ops::Range, str::FromStr};

type S = i16;
#[derive(Debug, Clone, Copy)]
struct Antenna {
    x: S,
    y: S,
    variant: char,
}

impl Antenna {
    fn antinode_pair(self, other: Self) -> Vec<Self> {
        let offset_x = other.x - self.x;
        let offset_y = other.y - self.y;

        vec![
            Self {
                x: other.x + offset_x,
                y: other.y + offset_y,
                variant: self.variant,
            },
            Self {
                x: self.x - offset_x,
                y: self.y - offset_y,
                variant: self.variant,
            },
        ]
    }

    fn antinode_array(self, other: Self, x_domain: &Range<S>, y_domain: &Range<S>) -> Vec<Self> {
        let offset_x = other.x - self.x;
        let offset_y = other.y - self.y;

        let oth_x = (x_domain.end - other.x.max(self.x)) / offset_x;
        let oth_y = (y_domain.end - other.y.max(self.y)) / offset_y;

        let oth_n = oth_x.max(oth_y);

        let self_x = self.x.min(other.x) / offset_x;
        let self_y = self.y.min(other.y) / offset_y;

        let self_n = self_x.max(self_y);

        let mut s: Vec<Self> = (0..=self_n)
            .map(|n| Self {
                x: self.x - n * offset_x,
                y: self.y - n * offset_y,
                variant: self.variant,
            })
            .collect();

        s.extend((0..=oth_n).map(|n| Self {
            x: other.x + n * offset_x,
            y: other.y + n * offset_y,
            variant: self.variant,
        }));

        s
    }
}
#[derive(Debug)]
struct Field {
    antennae: Vec<Antenna>,
    x_domain: Range<S>,
    y_domain: Range<S>,
}

impl Field {
    fn get_antinodes(&mut self) -> Vec<Antenna> {
        let mut antinodes: Vec<Antenna> = Vec::new();

        self.antennae.sort_by_key(|a| a.variant);
        for (_variant, group) in &self.antennae.iter().chunk_by(|a| a.variant) {
            antinodes.extend(
                group
                    .combinations(2)
                    .flat_map(|f| ((f[0]).antinode_pair(*f[1])))
                    .filter(|f| self.validate_node(f)),
            );
        }

        antinodes
    }

    fn get_all_antinodes(&mut self) -> Vec<Antenna> {
        let mut antinodes: Vec<Antenna> = Vec::new();

        self.antennae.sort_by_key(|a| a.variant);
        for (_variant, group) in &self.antennae.iter().chunk_by(|a| a.variant) {
            antinodes.extend(
                group
                    .combinations(2)
                    .flat_map(|f| ((f[0]).antinode_array(*f[1], &self.x_domain, &self.y_domain)))
                    .filter(|f| self.validate_node(f)),
            );
        }

        antinodes
    }

    fn validate_node(&self, node: &Antenna) -> bool {
        self.x_domain.contains(&node.x) && self.y_domain.contains(&node.y)
    }
}

impl FromStr for Field {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.find('\n').unwrap() as S;
        let h = (s.len() as S) / (w + 1);

        let antennae: Vec<Antenna> = s
            .chars()
            .filter(|f| !f.is_whitespace())
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .map(|(i, c)| Antenna {
                x: i as S % w,
                y: i as S / w,
                variant: c,
            })
            .collect();

        Ok(Self {
            antennae,
            x_domain: 0..w,
            y_domain: 0..h,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut f = Field::from_str(input).unwrap();

    let mut an = f.get_antinodes();

    an.sort_by_key(|f| (f.y, f.x));

    an.dedup_by_key(|f| (f.y, f.x));

    Some(an.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut f = Field::from_str(input).unwrap();

    let mut an = f.get_all_antinodes();

    an.sort_by_key(|f| (f.y, f.x));

    an.dedup_by_key(|f| (f.y, f.x));

    Some(an.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
