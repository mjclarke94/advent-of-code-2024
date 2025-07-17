advent_of_code::solution!(12);

const MAX_SIZE: usize = 150;

struct Grid<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    #[inline(always)]
    fn new(input: &'a str) -> Self {
        let data = input.as_bytes();
        let width = data.iter().position(|&b| b == b'\n').unwrap_or(data.len());
        let height = (data.len() + 1) / (width + 1);
        Self {
            data,
            width,
            height,
        }
    }

    #[inline(always)]
    fn get(&self, row: usize, col: usize) -> u8 {
        if row >= self.height || col >= self.width {
            return 0;
        }
        self.data[row * (self.width + 1) + col]
    }
}

fn solve_both_parts(input: &str) -> (usize, usize) {
    let grid = Grid::new(input);
    let mut visited = [false; MAX_SIZE * MAX_SIZE];
    let mut stack = Vec::with_capacity(MAX_SIZE * MAX_SIZE);
    let mut part1_total = 0;
    let mut part2_total = 0;

    for start_row in 0..grid.height {
        for start_col in 0..grid.width {
            let start_idx = start_row * MAX_SIZE + start_col;
            if visited[start_idx] {
                continue;
            }

            let plant_type = grid.get(start_row, start_col);
            let mut area = 0;
            let mut perimeter = 0;
            let mut corners = 0;

            // Flood fill using stack
            stack.clear();
            stack.push((start_row, start_col));
            visited[start_idx] = true;

            while let Some((row, col)) = stack.pop() {
                area += 1;

                // Check 4 neighbors for perimeter and flood fill
                let n = row > 0 && grid.get(row - 1, col) == plant_type;
                let s = row + 1 < grid.height && grid.get(row + 1, col) == plant_type;
                let e = col + 1 < grid.width && grid.get(row, col + 1) == plant_type;
                let w = col > 0 && grid.get(row, col - 1) == plant_type;

                // Count perimeter edges
                if !n {
                    perimeter += 1;
                }
                if !s {
                    perimeter += 1;
                }
                if !e {
                    perimeter += 1;
                }
                if !w {
                    perimeter += 1;
                }

                // Add unvisited neighbors to stack
                if n {
                    let idx = (row - 1) * MAX_SIZE + col;
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row - 1, col));
                    }
                }
                if s {
                    let idx = (row + 1) * MAX_SIZE + col;
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row + 1, col));
                    }
                }
                if e {
                    let idx = row * MAX_SIZE + (col + 1);
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row, col + 1));
                    }
                }
                if w {
                    let idx = row * MAX_SIZE + (col - 1);
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row, col - 1));
                    }
                }

                // Count corners for part 2
                let ne =
                    row > 0 && col + 1 < grid.width && grid.get(row - 1, col + 1) == plant_type;
                let nw = row > 0 && col > 0 && grid.get(row - 1, col - 1) == plant_type;
                let se = row + 1 < grid.height
                    && col + 1 < grid.width
                    && grid.get(row + 1, col + 1) == plant_type;
                let sw =
                    row + 1 < grid.height && col > 0 && grid.get(row + 1, col - 1) == plant_type;

                // Outside corners
                if !n && !w {
                    corners += 1;
                }
                if !n && !e {
                    corners += 1;
                }
                if !s && !w {
                    corners += 1;
                }
                if !s && !e {
                    corners += 1;
                }

                // Inside corners
                if n && w && !nw {
                    corners += 1;
                }
                if n && e && !ne {
                    corners += 1;
                }
                if s && w && !sw {
                    corners += 1;
                }
                if s && e && !se {
                    corners += 1;
                }
            }

            part1_total += area * perimeter;
            part2_total += area * corners;
        }
    }

    (part1_total, part2_total)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (part1, _) = solve_both_parts(input);
    Some(part1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, part2) = solve_both_parts(input);
    Some(part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
