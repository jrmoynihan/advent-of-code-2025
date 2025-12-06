use grid::Grid;

advent_of_code::solution!(6);

fn make_grid(input: &str, order: Option<grid::Order>) -> Grid<&str> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines.first().map(|l| l.len()).unwrap_or(0);
    let mut grid: Grid<&str> =
        Grid::with_capacity_and_order(rows, cols, order.unwrap_or(grid::Order::RowMajor));
    for (i, line) in lines.into_iter().enumerate() {
        grid.insert_row(i, line.split_whitespace().collect());
    }
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let problem_grid = make_grid(input, None);
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

fn parse_digits_in_column_to_u64(column: &[&char]) -> u64 {
    column
        .iter()
        .filter_map(|c| c.to_digit(10))
        .fold(0u64, |acc, d| acc * 10 + d as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut grid: Grid<char> = Grid::with_capacity_and_order(rows, cols, grid::Order::RowMajor);
    for (row, line) in lines.iter().enumerate() {
        let mut r_vec: Vec<char> = line.chars().collect();
        r_vec.resize(cols, ' ');
        grid.insert_row(row, r_vec);
    }
    let mut numbers: Vec<u64> = Vec::new();
    let results: u64 = grid
        .iter_cols()
        .rev()
        .filter_map(|col| {
            let col_vec: Vec<&char> = col.collect();
            if col_vec.iter().any(|c| c.is_ascii_digit()) {
                let operator = col_vec.last()?;
                let remaining = &col_vec[..col_vec.len() - 1];
                let number = parse_digits_in_column_to_u64(remaining);
                numbers.push(number);
                match operator {
                    '*' => {
                        let product: u64 = numbers.iter().product();
                        numbers.clear();
                        Some(product)
                    }
                    '+' => {
                        let sum: u64 = numbers.iter().sum();
                        numbers.clear();
                        Some(sum)
                    }
                    _ => None,
                }
            } else {
                numbers.clear();
                None
            }
        })
        .sum();

    Some(results)
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
        assert_eq!(result, Some(3263827));
    }
}
