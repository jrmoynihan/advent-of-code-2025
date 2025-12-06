use grid::Grid;

advent_of_code::solution!(6);

fn make_grid(input: &str) -> Grid<&str> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines.first().map(|l| l.len()).unwrap_or(0);
    let mut grid: Grid<&str> = Grid::with_capacity(rows, cols);
    for (i, line) in lines.into_iter().enumerate() {
        grid.insert_row(i, line.split_whitespace().collect());
    }
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let problem_grid = make_grid(input);
    let answers = problem_grid
        .iter_cols()
        .filter_map(|col| {
            let mut column = col.copied();
            let operator = column.next_back()?;
            match operator {
                "*" => Some(
                    column
                        .filter_map(|operand| operand.parse::<u64>().ok())
                        .fold(1u64, |acc, x| acc * x),
                ),
                "+" => Some(
                    column
                        .filter_map(|operand| operand.parse::<u64>().ok())
                        .sum::<u64>(),
                ),
                _ => None,
            }
        })
        .sum();

    Some(answers)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
