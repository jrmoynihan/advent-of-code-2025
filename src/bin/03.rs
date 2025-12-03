use rayon::{iter::ParallelIterator, str::ParallelString};

advent_of_code::solution!(3);

fn find_max_joltage(line: &str) -> Option<u64> {
    let bytes = line.as_bytes();
    let max_pair_result = bytes
        .iter()
        .enumerate()
        // D1: Outer loop iterates over every possible starting byte (d1_byte)
        .filter_map(|(i1, &d1_byte)| {
            // Ignore non-digit characters as D1
            if !d1_byte.is_ascii_digit() {
                return None;
            }

            // D2: Inner search finds the maximum byte appearing AFTER i1
            let best_d2_byte = bytes
                .iter()
                .skip(i1 + 1) // Start search immediately AFTER D1
                .filter(|&&b| b.is_ascii_digit()) // Only consider digits
                .max();

            // If a valid D2 exists, calculate the combined value
            best_d2_byte.map(|&d2_byte| {
                // Convert the string bytes to its integer value (e.g. b'9' - b'0' = 9)
                let d1 = (d1_byte - b'0') as u64;
                let d2 = (d2_byte - b'0') as u64;

                // Return (combined_value, d1, d2)
                (d1 * 10 + d2, d1, d2)
            })
        })
        // Find the single result tuple that has the highest combined_value
        .max_by_key(|&(val, _, _)| val);

    // Extract the digits and calculate the final joltage sum
    max_pair_result.map(|(val, _, _)| val)
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .par_lines()
        .map(|line: &str| find_max_joltage(line).unwrap_or(0))
        .sum();
    Some(sum as u64)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(3121910778619));
    }
}
