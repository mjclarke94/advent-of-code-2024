advent_of_code::solution!(10);

use std::collections::{HashMap, HashSet};

type Coord = i16;
type Height = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: Coord,
    y: Coord,
}

struct Grid {
    data: Vec<Height>,
    width: Coord,
    height: Coord,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() as Coord;
        let width = lines.first().map(|l| l.len()).unwrap_or(0) as Coord;

        let mut data = Vec::with_capacity((width * height) as usize);
        for line in lines {
            data.extend(line.bytes().map(|b| b - b'0'));
        }

        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, pos: Pos) -> Option<Height> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            Some(self.data[(pos.y * self.width + pos.x) as usize])
        } else {
            None
        }
    }

    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        const DIRS: [(Coord, Coord); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        DIRS.iter().filter_map(move |&(dx, dy)| {
            let new_pos = Pos {
                x: pos.x + dx,
                y: pos.y + dy,
            };
            if new_pos.x >= 0 && new_pos.x < self.width && new_pos.y >= 0 && new_pos.y < self.height
            {
                Some(new_pos)
            } else {
                None
            }
        })
    }

    fn find_trailheads(&self) -> Vec<Pos> {
        let mut trailheads = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos { x, y };
                if self.get(pos) == Some(0) {
                    trailheads.push(pos);
                }
            }
        }
        trailheads
    }

    fn find_reachable_peaks(&self, start: Pos) -> HashSet<Pos> {
        let mut reachable = HashSet::new();
        let mut to_visit = vec![start];
        let mut visited = HashSet::new();

        while let Some(pos) = to_visit.pop() {
            if !visited.insert(pos) {
                continue;
            }

            let current_height = self.get(pos).unwrap();

            if current_height == 9 {
                reachable.insert(pos);
                continue;
            }

            for neighbor in self.neighbors(pos) {
                if let Some(neighbor_height) = self.get(neighbor) {
                    if neighbor_height == current_height + 1 {
                        to_visit.push(neighbor);
                    }
                }
            }
        }

        reachable
    }

    fn count_paths(&self, start: Pos) -> u32 {
        let mut memo = HashMap::new();
        self.count_paths_memo(start, &mut memo)
    }

    fn count_paths_memo(&self, pos: Pos, memo: &mut HashMap<Pos, u32>) -> u32 {
        if let Some(&cached) = memo.get(&pos) {
            return cached;
        }

        let height = self.get(pos).unwrap();

        if height == 9 {
            memo.insert(pos, 1);
            return 1;
        }

        let mut total_paths = 0;

        for neighbor in self.neighbors(pos) {
            if let Some(neighbor_height) = self.get(neighbor) {
                if neighbor_height == height + 1 {
                    total_paths += self.count_paths_memo(neighbor, memo);
                }
            }
        }

        memo.insert(pos, total_paths);
        total_paths
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let trailheads = grid.find_trailheads();

    let total_score: u32 = trailheads
        .iter()
        .map(|&trailhead| grid.find_reachable_peaks(trailhead).len() as u32)
        .sum();

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let trailheads = grid.find_trailheads();

    let total_rating: u32 = trailheads
        .iter()
        .map(|&trailhead| grid.count_paths(trailhead))
        .sum();

    Some(total_rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_grid_parsing() {
        let input = "012\n345\n678";
        let grid = Grid::new(input);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.get(Pos { x: 0, y: 0 }), Some(0));
        assert_eq!(grid.get(Pos { x: 2, y: 2 }), Some(8));
    }

    #[test]
    fn test_neighbors() {
        let input = "012\n345\n678";
        let grid = Grid::new(input);
        let neighbors: Vec<_> = grid.neighbors(Pos { x: 1, y: 1 }).collect();
        assert_eq!(neighbors.len(), 4);

        let corner_neighbors: Vec<_> = grid.neighbors(Pos { x: 0, y: 0 }).collect();
        assert_eq!(corner_neighbors.len(), 2);
    }

    #[test]
    fn test_find_trailheads() {
        let input = "102\n045\n078";
        let grid = Grid::new(input);
        let trailheads = grid.find_trailheads();
        // Checking positions: '102' row 0, '045' row 1, '078' row 2
        // 0 is at: (0,1), (0,2), and (1,0)
        assert_eq!(trailheads.len(), 3);
        assert!(trailheads.contains(&Pos { x: 0, y: 2 }));
        assert!(trailheads.contains(&Pos { x: 0, y: 1 }));
        assert!(trailheads.contains(&Pos { x: 1, y: 0 })); // '102' has 0 at position (1,0)
    }

    #[test]
    fn test_simple_trail() {
        let input = "0123\n1234\n8765\n9876";
        let grid = Grid::new(input);
        let trailheads = grid.find_trailheads();
        assert_eq!(trailheads.len(), 1);
        assert_eq!(trailheads[0], Pos { x: 0, y: 0 });
    }
}
