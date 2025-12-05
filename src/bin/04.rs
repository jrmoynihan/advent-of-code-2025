use grid::Grid;

advent_of_code::solution!(4);

fn find_accessible_locations(grid: &Grid<char>) -> Option<Vec<(usize, usize)>> {
    let mut accessible_locations = Vec::new();
    grid.iter_rows().enumerate().for_each(|(r, row)| {
        row.enumerate().for_each(|(c, space)| {
            if *space != '@' {
                return;
            } // Skip if not an '@'
            let mut roll_count = 0;
            'outer: for dr in -1..=1 as isize {
                for dc in -1..=1 as isize {
                    if r as isize + dr < 0
                        || c as isize + dc < 0
                        || r as isize + dr >= grid.rows() as isize
                        || c as isize + dc >= grid.cols() as isize
                        || dr == 0 && dc == 0
                    {
                        continue;
                    }
                    if let Some('@') =
                        grid.get((r as isize + dr) as usize, (c as isize + dc) as usize)
                    {
                        roll_count += 1;
                        if roll_count >= 4 {
                            break 'outer;
                        }
                    }
                }
            }
            if roll_count < 4 {
                accessible_locations.push((r, c));
            }
        });
    });
    // println!("Accessible locations: {:?}", accessible_locations);
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
    while find_and_remove_accessible_locations(&mut grid) > 0 {
        continue;
    }
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
