advent_of_code::solution!(10);

type Coord = u16;
type Height = u8;

struct Grid {
    data: Vec<Height>,
    width: Coord,
    height: Coord,
    trailheads: Vec<usize>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
        let height = lines.len() as Coord;
        let width = lines[0].len() as Coord;

        let mut data = Vec::with_capacity((width * height) as usize);
        let mut trailheads = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                let h = ch - b'0';
                if h == 0 {
                    trailheads.push(row * width as usize + col);
                }
                data.push(h);
            }
        }

        Self {
            data,
            width,
            height,
            trailheads,
        }
    }

    fn find_reachable_peaks(&self, start_idx: usize) -> u32 {
        // Use a bitset for small grids, otherwise use HashSet
        if self.data.len() <= 256 {
            self.find_reachable_peaks_bitset(start_idx)
        } else {
            self.find_reachable_peaks_hashset(start_idx)
        }
    }

    fn find_reachable_peaks_bitset(&self, start_idx: usize) -> u32 {
        let mut peaks = [0u64; 4]; // Support up to 256 positions
        let mut to_visit = [0u16; 256];
        let mut visit_count = 1;
        to_visit[0] = start_idx as u16;

        let mut visited = [0u64; 4];
        let word = start_idx >> 6;
        let bit = start_idx & 63;
        visited[word] |= 1u64 << bit;

        let width = self.width as usize;

        while visit_count > 0 {
            visit_count -= 1;
            let idx = to_visit[visit_count] as usize;
            let height = self.data[idx];

            if height == 9 {
                let word = idx >> 6;
                let bit = idx & 63;
                peaks[word] |= 1u64 << bit;
                continue;
            }

            let row = idx / width;
            let col = idx % width;
            let next_height = height + 1;

            // Check all 4 directions
            let neighbors = [
                (row > 0).then(|| idx - width),
                (col + 1 < width).then(|| idx + 1),
                (row + 1 < self.height as usize).then(|| idx + width),
                (col > 0).then(|| idx - 1),
            ];

            for next_idx in neighbors.into_iter().flatten() {
                let word = next_idx >> 6;
                let bit = next_idx & 63;

                if (visited[word] & (1u64 << bit)) == 0 && self.data[next_idx] == next_height {
                    visited[word] |= 1u64 << bit;
                    to_visit[visit_count] = next_idx as u16;
                    visit_count += 1;
                }
            }
        }

        peaks.iter().map(|p| p.count_ones()).sum()
    }

    fn find_reachable_peaks_hashset(&self, start_idx: usize) -> u32 {
        use std::collections::HashSet;
        let mut peaks = HashSet::new();
        let mut to_visit = vec![start_idx];
        let mut visited = vec![false; self.data.len()];
        visited[start_idx] = true;

        while let Some(idx) = to_visit.pop() {
            let height = self.data[idx];

            if height == 9 {
                peaks.insert(idx);
                continue;
            }

            let row = idx / self.width as usize;
            let col = idx % self.width as usize;
            let next_height = height + 1;

            // Up
            if row > 0 {
                let next_idx = idx - self.width as usize;
                if !visited[next_idx] && self.data[next_idx] == next_height {
                    visited[next_idx] = true;
                    to_visit.push(next_idx);
                }
            }

            // Right
            if col + 1 < self.width as usize {
                let next_idx = idx + 1;
                if !visited[next_idx] && self.data[next_idx] == next_height {
                    visited[next_idx] = true;
                    to_visit.push(next_idx);
                }
            }

            // Down
            if row + 1 < self.height as usize {
                let next_idx = idx + self.width as usize;
                if !visited[next_idx] && self.data[next_idx] == next_height {
                    visited[next_idx] = true;
                    to_visit.push(next_idx);
                }
            }

            // Left
            if col > 0 {
                let next_idx = idx - 1;
                if !visited[next_idx] && self.data[next_idx] == next_height {
                    visited[next_idx] = true;
                    to_visit.push(next_idx);
                }
            }
        }

        peaks.len() as u32
    }

    fn count_paths(&self) -> Vec<u32> {
        let size = self.data.len();
        let mut paths = vec![0u32; size];

        // Process in reverse height order (9 to 0)
        for height in (0..=9).rev() {
            for idx in 0..size {
                if self.data[idx] != height {
                    continue;
                }

                if height == 9 {
                    paths[idx] = 1;
                } else {
                    let row = idx / self.width as usize;
                    let col = idx % self.width as usize;
                    let next_height = height + 1;
                    let mut count = 0;

                    // Up
                    if row > 0 {
                        let next_idx = idx - self.width as usize;
                        if self.data[next_idx] == next_height {
                            count += paths[next_idx];
                        }
                    }

                    // Right
                    if col + 1 < self.width as usize {
                        let next_idx = idx + 1;
                        if self.data[next_idx] == next_height {
                            count += paths[next_idx];
                        }
                    }

                    // Down
                    if row + 1 < self.height as usize {
                        let next_idx = idx + self.width as usize;
                        if self.data[next_idx] == next_height {
                            count += paths[next_idx];
                        }
                    }

                    // Left
                    if col > 0 {
                        let next_idx = idx - 1;
                        if self.data[next_idx] == next_height {
                            count += paths[next_idx];
                        }
                    }

                    paths[idx] = count;
                }
            }
        }

        paths
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);

    let total: u32 = grid
        .trailheads
        .iter()
        .map(|&idx| grid.find_reachable_peaks(idx))
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let paths = grid.count_paths();

    let total: u32 = grid.trailheads.iter().map(|&idx| paths[idx]).sum();

    Some(total)
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
}
