advent_of_code::solution!(7);

type S = u64;
const TEN: S = 10;

#[derive(Debug)]
struct Equation {
    target: S,
    components: Vec<S>,
}

impl Equation {
    fn from_lines(s: &str) -> Vec<Self> {
        s.lines()
            .map(|line| {
                let mut i = line.split([' ', ':']).filter(|f| !f.is_empty());
                let target: S = i.next().unwrap().parse().unwrap();
                let components: Vec<S> = i.map(|f| f.parse().unwrap()).collect();
                Self { target, components }
            })
            .collect()
    }

    fn solveable(&self) -> bool {
        Self::dfs(self.target, self.components[0], &self.components[1..])
    }

    fn solveable_concat(&self) -> bool {
        Self::dfs_concat(self.target, self.components[0], &self.components[1..])
    }

    fn dfs(target: S, buffer: S, remaining: &[S]) -> bool {
        if remaining.is_empty() {
            target == buffer
        } else if buffer > target {
            false
        } else {
            Self::dfs(target, buffer + remaining[0], &remaining[1..])
                || Self::dfs(target, buffer * remaining[0], &remaining[1..])
        }
    }

    fn dfs_concat(target: S, buffer: S, remaining: &[S]) -> bool {
        if remaining.is_empty() {
            target == buffer
        } else if buffer > target {
            false
        } else {
            Self::dfs_concat(target, buffer + remaining[0], &remaining[1..])
                || Self::dfs_concat(target, buffer * remaining[0], &remaining[1..])
                || Self::dfs_concat(target, concat(buffer, remaining[0]), &remaining[1..])
        }
    }
}

fn concat(l: S, r: S) -> S {
    l * TEN.pow(r.ilog10() + 1) + r
}

pub fn part_one(input: &str) -> Option<S> {
    let v = Equation::from_lines(input);

    Some(
        v.into_iter()
            .filter(|f| f.solveable())
            .map(|f| f.target)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<S> {
    let v = Equation::from_lines(input);

    Some(
        v.into_iter()
            .filter(|f| f.solveable_concat())
            .map(|f| f.target)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
