use counter::Counter;
use std::{num::ParseIntError, str::FromStr};
use unroll::unroll_for_loops;

advent_of_code::solution!(22);

type S = u64;
type C = i32;

const SHIFT_64: S = 6;
const SHIFT_32: S = 5;
const SHIFT_2048: S = 11;

const N: u16 = 2000;

const PRUNE: S = 16777215; // 2 ** 24 - 1

#[derive(Debug)]
struct SecretNumber {
    value: S,
    price: Vec<C>,
    delta: Vec<C>,
    patterns: Counter<(C, C, C, C), C>,
}

impl SecretNumber {
    #[unroll_for_loops]
    fn update(&mut self) {
        let mut value = self.value;

        for _ in 0..N {
            value ^= value << SHIFT_64;
            value &= PRUNE;
            value ^= value >> SHIFT_32;
            // self.value &= PRUNE; //needed?
            value ^= value << SHIFT_2048;
            value &= PRUNE;
        }

        self.value = value;
    }

    fn update_tracked(&mut self) {
        let mut value = self.value;
        self.price.push((value % 10) as C);

        for _ in 0..N {
            value ^= value << SHIFT_64;
            value &= PRUNE;
            value ^= value >> SHIFT_32;
            // self.value &= PRUNE; //needed?
            value ^= value << SHIFT_2048;
            value &= PRUNE;

            self.price.push((value % 10) as C);
        }
        self.value = value;
    }

    fn calc_deltas(&mut self) {
        self.delta
            .extend(self.price.windows(2).map(|f| f[1] - f[0]))
    }

    fn get_prices(&mut self) {
        self.delta
            .windows(4)
            .zip(self.price.iter().skip(4))
            .for_each(|(pattern, price)| {
                self.patterns
                    .entry((pattern[0], pattern[1], pattern[2], pattern[3]))
                    .or_insert(*price);
            });
    }
}

impl FromStr for SecretNumber {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;
        Ok(Self {
            value,
            price: Vec::with_capacity(2000),
            delta: Vec::with_capacity(2000),
            patterns: Counter::with_capacity(500),
        })
    }
}

pub fn part_one(input: &str) -> Option<S> {
    let try_parse: Result<Vec<SecretNumber>, ParseIntError> =
        input.lines().map(SecretNumber::from_str).collect();

    let mut secrets = try_parse.unwrap();

    secrets.iter_mut().for_each(|f| f.update());

    Some(secrets.into_iter().map(|f| f.value).sum())
}

pub fn part_two(input: &str) -> Option<C> {
    let try_parse: Result<Vec<SecretNumber>, ParseIntError> =
        input.lines().map(SecretNumber::from_str).collect();

    let mut secrets = try_parse.unwrap();

    secrets.iter_mut().for_each(|f| {
        f.update_tracked();
        f.calc_deltas();
        f.get_prices();
    });

    let total_bananas: Counter<(C, C, C, C), C> = secrets
        .into_iter()
        .map(|f| f.patterns)
        .fold(Counter::new(), |acc, x| acc + x);

    Some(total_bananas.most_common()[0].1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));

        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
