advent_of_code::solution!(11);

use cached::proc_macro::cached;

use counter::Counter;

type S = usize;

#[cached]
fn blink_pebble(value: S) -> Counter<S> {
    match value {
        0 => Counter::from_iter(vec![1]),
        a if a.ilog10() % 2 == 1 => {
            let fac = (10 as S).pow((a.ilog10() + 1) / 2);
            Counter::from_iter(vec![a / fac, a - (a / fac) * fac])
        }
        _ => Counter::from_iter(vec![value * 2024]),
    }
}

fn blink(pebbles: Counter<S>) -> Counter<S> {
    let mut out: Counter<S> = Counter::new();

    for (val, count) in pebbles {
        let mut c = blink_pebble(val);
        c.iter_mut().for_each(|(_, val)| {
            *val *= count;
        });
        out += c;
    }

    out
}

fn parse_input(s: &str) -> Counter<S> {
    s.split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .map(|f| f.parse::<S>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut pebbles = parse_input(input);

    for _ in 0..25 {
        pebbles = blink(pebbles);
    }

    Some(pebbles.into_iter().map(|(_, c)| c).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pebbles = parse_input(input);

    for _ in 0..75 {
        pebbles = blink(pebbles);
    }

    Some(pebbles.into_iter().map(|(_, c)| c).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
