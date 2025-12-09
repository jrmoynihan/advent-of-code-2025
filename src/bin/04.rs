advent_of_code::solution!(4);

// Count matching neighbors directly without collecting into array first
// This eliminates the overhead of array building and Option unwrapping
// fn count_matching_neighbors(grid: &Grid<u8>, r: usize, c: usize, target: u8) -> u32 {
//     let rows = grid.rows();
//     let cols = grid.cols();
//     let r_signed = r as isize;
//     let c_signed = c as isize;
//     let mut count = 0u32;

//     // Check all 8 neighbors directly
//     for dr in -1..=1 {
//         for dc in -1..=1 {
//             if dr == 0 && dc == 0 {
//                 continue;
//             }
//             let nr = r_signed + dr;
//             let nc = c_signed + dc;
//             if nr >= 0 && nc >= 0 && nr < rows as isize && nc < cols as isize {
//                 if let Some(&val) = grid.get(nr as usize, nc as usize) {
//                     if val == target {
//                         count += 1;
//                     }
//                 }
//             }
//         }
//     }
//     count
// }

// fn find_accessible_locations(grid: &Grid<u8>) -> Option<Vec<(usize, usize)>> {
//     let mut accessible_locations = Vec::new();
//     let rows = grid.rows();
//     let cols = grid.cols();

//     for r in 0..rows {
//         for c in 0..cols {
//             if grid.get(r, c) != Some(&b'@') {
//                 continue;
//             }

//             let roll_count = count_matching_neighbors(grid, r, c, b'@');
//             if roll_count < 4 {
//                 accessible_locations.push((r, c));
//             }
//         }
//     }
//     Some(accessible_locations)
// }

// fn make_grid(input: &str) -> Grid<u8> {
//     let lines: Vec<&str> = input.lines().collect();
//     let rows = lines.len();
//     let cols = lines.first().map(|l| l.len()).unwrap_or(0);
//     let mut grid: Grid<u8> = Grid::with_capacity(rows, cols);
//     for (i, line) in lines.iter().enumerate() {
//         let bytes = line.as_bytes();
//         let mut row = Vec::with_capacity(cols);
//         row.extend_from_slice(bytes);
//         grid.insert_row(i, row);
//     }
//     grid
// }

// pub fn part_one(input: &str) -> Option<u64> {
//     let grid = make_grid(input);
//     if let Some(accessible_locations) = find_accessible_locations(&grid) {
//         Some(accessible_locations.len() as u64)
//     } else {
//         None
//     }
// }

// pub fn part_two(input: &str) -> Option<u64> {
//     let mut grid = make_grid(input);
//     let initial_rolls = grid.iter().filter(|&&c| c == b'@').count() as u64;

//     // Optimized: Only check cells that might have changed (adjacent to removed cells)
//     // Use a work queue to track which cells need re-checking
//     let rows = grid.rows();
//     let cols = grid.cols();
//     let mut to_check: std::collections::HashSet<(usize, usize)> = (0..rows)
//         .flat_map(|r| (0..cols).map(move |c| (r, c)))
//         .filter(|&(r, c)| grid.get(r, c) == Some(&b'@'))
//         .collect();

//     while !to_check.is_empty() {
//         let mut to_remove = Vec::new();
//         let mut next_to_check = std::collections::HashSet::new();

//         // Check all cells in the work queue
//         for (r, c) in &to_check {
//             if grid.get(*r, *c) != Some(&b'@') {
//                 continue;
//             }

//             let roll_count = count_matching_neighbors(&grid, *r, *c, b'@');
//             if roll_count < 4 {
//                 to_remove.push((*r, *c));
//             }
//         }

//         // Remove accessible locations and mark their neighbors for re-checking
//         for (r, c) in &to_remove {
//             if let Some(space) = grid.get_mut(*r, *c) {
//                 *space = b'x';

//                 // Add neighbors to the next work queue
//                 let r_signed = *r as isize;
//                 let c_signed = *c as isize;
//                 for dr in -1..=1 {
//                     for dc in -1..=1 {
//                         if dr == 0 && dc == 0 {
//                             continue;
//                         }
//                         let nr = r_signed + dr;
//                         let nc = c_signed + dc;
//                         if nr >= 0 && nc >= 0 && nr < rows as isize && nc < cols as isize {
//                             let nr = nr as usize;
//                             let nc = nc as usize;
//                             // Only check neighbors that are still '@'
//                             if grid.get(nr, nc) == Some(&b'@') {
//                                 next_to_check.insert((nr, nc));
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         to_check = next_to_check;
//     }

//     let final_rolls = grid.iter().filter(|&&c| c == b'@').count() as u64;
//     Some(initial_rolls - final_rolls)
// }

fn has_4_or_fewer_neighbors(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
    // Count all '@' in 3x3 area (including center cell)
    // Since center is included, we check <= 4 instead of < 4
    grid[row - 1..=row + 1]
        .iter()
        .map(|row| row[col - 1..=col + 1].iter().filter(|&&cell| cell).count())
        .sum::<usize>()
        <= 4
}

/// Build a grid with border padding to avoid bounds checking
fn make_padded_grid(input: &str) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            let mut row = vec![false]; // Left border
            row.extend(line.chars().map(|c| c == '@'));
            row.push(false); // Right border
            row
        })
        .collect();

    // Add top and bottom borders
    let col_count = grid[0].len();
    grid.insert(0, vec![false; col_count]);
    grid.push(vec![false; col_count]);
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = make_padded_grid(input);
    let col_count = grid[0].len();

    // Start inside the grid "border" to avoid bounds checking
    let count = (1..grid.len() - 1)
        .map(|row| {
            (1..col_count - 1)
                .filter(|&col| grid[row][col] && has_4_or_fewer_neighbors(&grid, row, col))
                .count()
        })
        .sum::<usize>();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Build grid with border padding
    let mut grid = make_padded_grid(input);
    let col_count = grid[0].len();

    let initial_rolls = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell)
        .count() as u64;

    // Row-by-row backtracking algorithm
    let mut row = 1; // Start after border
    while row < grid.len() - 1 {
        let mut changed = false;

        // Process current row left-to-right
        for col in 1..col_count - 1 {
            if grid[row][col] && has_4_or_fewer_neighbors(&grid, row, col) {
                grid[row][col] = false;
                changed = true;
            }
        }

        // If we removed anything, go back one row (but not before border)
        if changed && row > 1 {
            row -= 1;
        } else {
            row += 1;
        }
    }

    let final_rolls = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell)
        .count() as u64;

    Some(initial_rolls - final_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(43));
    }
}
