use std::{collections::HashSet, ops::Range, str::FromStr};
advent_of_code::solution!(6);

type S = i16;

#[derive(Debug, Clone)]
struct Guard {
    domain: (Range<S>, Range<S>),
    start: (S, S),
    position: (S, S),
    direction: (S, S),
    visited: HashSet<((S, S), (S, S))>,
    obstacles: HashSet<(S, S)>,
    done: bool,
    looped: bool,
}

impl Guard {
    fn from_start(start: (S, S), domain: (Range<S>, Range<S>), obstacles: HashSet<(S, S)>) -> Self {
        Guard {
            start,
            position: start,
            direction: (0, -1),
            visited: HashSet::new(),
            domain,
            obstacles,
            done: false,
            looped: false,
        }
    }

    fn reset(&mut self) {
        self.visited = HashSet::new();
        self.direction = (0, -1);
        self.position = self.start;
        self.looped = false;
        self.done = false;
    }

    fn step(&mut self) {
        let pos_n = (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        );
        if !self.domain.0.contains(&pos_n.0) || !self.domain.1.contains(&pos_n.1) {
            self.done = true;
        } else if self.obstacles.contains(&pos_n) {
            self.rotate();
        } else {
            self.position = pos_n;
            if !self.visited.insert((pos_n, self.direction)) {
                self.looped = true;
            }
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => (0, 0),
        }
    }

    fn solve(&mut self) {
        while (!self.done) && (!self.looped) {
            self.step();
        }
    }

    fn get_locs(&mut self) -> HashSet<(S, S)> {
        self.solve();
        self.visited.iter().map(|f| f.0).collect()
    }
}

impl FromStr for Guard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: S = s.find("\n").unwrap() as S;
        let y: S = (s.len() as S / (x + 1)) as S;

        let mut obstacles = HashSet::new();
        let mut start: (S, S) = (0, 0);

        for (j, line) in s.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((i as S, j as S));
                    }
                    '^' => start = (i as S, j as S),
                    _ => {}
                }
            }
        }

        Ok(Self::from_start(start, (0..x, 0..y), obstacles))
    }
}

pub fn part_one(input: &str) -> Option<S> {
    let mut guard = Guard::from_str(input).unwrap();
    let locs = guard.get_locs();
    Some(locs.len() as S)
}

pub fn part_two(input: &str) -> Option<S> {
    let mut guard = Guard::from_str(input).unwrap();
    let mut locs = guard.get_locs();

    locs.remove(&(guard.start));

    let mut loop_count: S = 0;

    for obs in locs {
        guard.obstacles.insert(obs);
        guard.solve();

        if guard.looped {
            loop_count += 1
        }

        guard.reset();
        guard.obstacles.remove(&obs);
    }

    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
