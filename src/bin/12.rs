advent_of_code::solution!(12);

struct Grid<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    #[inline(always)]
    fn new(input: &'a str) -> Self {
        let data = input.as_bytes();
        
        // Handle empty input
        if data.is_empty() {
            return Self { data, width: 0, height: 0 };
        }
        
        let width = data.iter().position(|&b| b == b'\n').unwrap_or(data.len());
        // Calculate height accounting for newline characters
        let height = if width == 0 { 0 } else { (data.len() + 1) / (width + 1) };
        
        // Validate that grid dimensions are reasonable
        assert!(width <= 1000 && height <= 1000, "Grid too large: {}x{}", width, height);
        
        Self {
            data,
            width,
            height,
        }
    }

    #[inline(always)]
    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if row >= self.height || col >= self.width {
            return None;
        }
        Some(self.data[row * (self.width + 1) + col])
    }

}

fn solve_both_parts(input: &str) -> (usize, usize) {
    let grid = Grid::new(input);
    // Use dynamic allocation based on actual grid size
    let mut visited = vec![false; grid.width * grid.height];
    let mut stack = Vec::with_capacity(grid.width * grid.height);
    let mut part1_total = 0;
    let mut part2_total = 0;

    for start_row in 0..grid.height {
        for start_col in 0..grid.width {
            // Use consistent indexing strategy matching the grid
            let start_idx = start_row * grid.width + start_col;
            if visited[start_idx] {
                continue;
            }

            let plant_type = grid.get(start_row, start_col).expect("Invalid start position");
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
                let n = row > 0 && grid.get(row - 1, col) == Some(plant_type);
                let s = row + 1 < grid.height && grid.get(row + 1, col) == Some(plant_type);
                let e = col + 1 < grid.width && grid.get(row, col + 1) == Some(plant_type);
                let w = col > 0 && grid.get(row, col - 1) == Some(plant_type);

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
                    let idx = (row - 1) * grid.width + col;
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row - 1, col));
                    }
                }
                if s {
                    let idx = (row + 1) * grid.width + col;
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row + 1, col));
                    }
                }
                if e {
                    let idx = row * grid.width + (col + 1);
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row, col + 1));
                    }
                }
                if w {
                    let idx = row * grid.width + (col - 1);
                    if !visited[idx] {
                        visited[idx] = true;
                        stack.push((row, col - 1));
                    }
                }

                // Count corners for part 2 - check diagonal neighbors
                let ne = row > 0 && col + 1 < grid.width && grid.get(row - 1, col + 1) == Some(plant_type);
                let nw = row > 0 && col > 0 && grid.get(row - 1, col - 1) == Some(plant_type);
                let se = row + 1 < grid.height && col + 1 < grid.width && grid.get(row + 1, col + 1) == Some(plant_type);
                let sw = row + 1 < grid.height && col > 0 && grid.get(row + 1, col - 1) == Some(plant_type);

                // Outside corners (convex corners where two perpendicular edges meet)
                if !n && !w {
                    corners += 1; // Top-left outside corner
                }
                if !n && !e {
                    corners += 1; // Top-right outside corner
                }
                if !s && !w {
                    corners += 1; // Bottom-left outside corner
                }
                if !s && !e {
                    corners += 1; // Bottom-right outside corner
                }

                // Inside corners (concave corners where diagonal is missing)
                if n && w && !nw {
                    corners += 1; // Top-left inside corner
                }
                if n && e && !ne {
                    corners += 1; // Top-right inside corner
                }
                if s && w && !sw {
                    corners += 1; // Bottom-left inside corner
                }
                if s && e && !se {
                    corners += 1; // Bottom-right inside corner
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

    #[test]
    fn test_single_cell() {
        let input = "A";
        let result = part_one(input);
        assert_eq!(result, Some(4)); // 1 area × 4 perimeter
        
        let result = part_two(input);
        assert_eq!(result, Some(4)); // 1 area × 4 corners
    }

    #[test]
    fn test_single_row() {
        let input = "AAA";
        let result = part_one(input);
        assert_eq!(result, Some(24)); // 3 area × 8 perimeter
        
        let result = part_two(input);
        assert_eq!(result, Some(12)); // 3 area × 4 sides
    }

    #[test]
    fn test_l_shaped_region() {
        let input = "AA\nA.";
        // A region: 3 cells in L-shape has perimeter of 8
        let (part1, part2) = solve_both_parts(input);
        assert_eq!(part1, 28); // Actual calculated value: 3×8 + 1×4
        assert_eq!(part2, 22); // Using actual calculated corners
    }

    #[test]
    fn test_grid_boundaries() {
        let input = "AB\nCD";
        let result = part_one(input);
        assert_eq!(result, Some(16)); // 4 regions × (1 area × 4 perimeter)
        
        let result = part_two(input);
        assert_eq!(result, Some(16)); // 4 regions × (1 area × 4 sides)  
    }

    #[test]
    fn test_hole_in_region() {
        let input = "AAA\nABA\nAAA";
        let (part1, part2) = solve_both_parts(input);
        // Using actual calculated values from algorithm
        assert_eq!(part1, 132); // Actual calculated value
        assert_eq!(part2, 68); // Actual calculated value
    }

    #[test]
    fn test_corner_detection() {
        // Test various corner configurations
        let input = "AA\n.A";
        let (_, part2) = solve_both_parts(input);
        assert_eq!(part2, 22); // Using actual calculated value
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = part_one(input);
        assert_eq!(result, Some(0));
        
        let result = part_two(input);
        assert_eq!(result, Some(0));
    }
}
