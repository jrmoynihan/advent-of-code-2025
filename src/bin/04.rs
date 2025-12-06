use grid::Grid;

advent_of_code::solution!(4);

fn find_accessible_locations(grid: &Grid<char>) -> Option<Vec<(usize, usize)>> {
    let mut accessible_locations = Vec::new();
    let rows = grid.rows();
    let cols = grid.cols();

    for r in 0..rows {
        for c in 0..cols {
            if grid.get(r, c) != Some(&'@') {
                continue;
            }

            let mut roll_count = 0;
            let r_signed = r as isize;
            let c_signed = c as isize;

            'outer: for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r_signed + dr;
                    let nc = c_signed + dc;
                    if nr >= 0 && nc >= 0 && nr < rows as isize && nc < cols as isize {
                        if grid.get(nr as usize, nc as usize) == Some(&'@') {
                            roll_count += 1;
                            if roll_count >= 4 {
                                break 'outer;
                            }
                        }
                    }
                }
            }
            if roll_count < 4 {
                accessible_locations.push((r, c));
            }
        }
    }
    Some(accessible_locations)
}

fn find_and_remove_accessible_locations(grid: &mut Grid<char>) -> u64 {
    let accessible_locations = find_accessible_locations(grid);
    if let Some(accessible_locations) = accessible_locations {
        for (r, c) in &accessible_locations {
            if let Some(space) = grid.get_mut(*r, *c) {
                *space = 'x';
            }
        }
        let rolls = accessible_locations.len() as u64;
        rolls
    } else {
        0
    }
}

fn make_grid(input: &str) -> Grid<char> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::with_capacity(rows, cols);
    for (i, line) in input.lines().enumerate() {
        grid.insert_row(i, line.chars().collect());
    }
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = make_grid(input);
    if let Some(accessible_locations) = find_accessible_locations(&grid) {
        Some(accessible_locations.len() as u64)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = make_grid(input);
    let initial_rolls = grid.iter().filter(|c| **c == '@').count() as u64;
    while find_and_remove_accessible_locations(&mut grid) > 0 {}
    let final_rolls = grid.iter().filter(|c| **c == '@').count() as u64;
    let result = initial_rolls - final_rolls;
    // println!("Final Grid: {:?}", grid);
    Some(result)
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
