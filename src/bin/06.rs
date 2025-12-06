use grid::Grid;

advent_of_code::solution!(6);

fn make_grid(input: &str, order: Option<grid::Order>) -> Grid<&str> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines.first().map(|l| l.len()).unwrap_or(0);
    let mut grid: Grid<&str> =
        Grid::with_capacity_and_order(rows, cols, order.unwrap_or(grid::Order::RowMajor));
    for (i, line) in lines.iter().enumerate() {
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

fn parse_digits_in_column_to_u64(column: &[&u8]) -> u64 {
    column
        .iter()
        .filter_map(|&&b| {
            if b >= b'0' && b <= b'9' {
                Some((b - b'0') as u64)
            } else {
                None
            }
        })
        .fold(0u64, |acc, d| acc * 10 + d)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut grid: Grid<u8> = Grid::with_capacity_and_order(rows, cols, grid::Order::RowMajor);
    for (row, line) in lines.iter().enumerate() {
        let bytes = line.as_bytes();
        let mut r_vec = Vec::with_capacity(cols);
        r_vec.extend_from_slice(bytes);
        r_vec.resize(cols, b' ');
        grid.insert_row(row, r_vec);
    }
    let mut numbers: Vec<u64> = Vec::with_capacity(16);
    let results: u64 = grid
        .iter_cols()
        .rev()
        .filter_map(|col| {
            let col_vec: Vec<&u8> = col.collect();
            if col_vec.iter().any(|&&b| b >= b'0' && b <= b'9') {
                let operator = col_vec.last()?;
                let remaining = &col_vec[..col_vec.len() - 1];
                let number = parse_digits_in_column_to_u64(remaining);
                numbers.push(number);
                match **operator {
                    b'*' => {
                        let product: u64 = numbers.iter().product();
                        numbers.clear();
                        Some(product)
                    }
                    b'+' => {
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
