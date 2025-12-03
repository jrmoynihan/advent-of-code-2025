use rayon::{iter::ParallelIterator, str::ParallelString};

advent_of_code::solution!(3);

fn find_max_joltage(line: &str) -> Option<u64> {
    let bytes = line.as_bytes();

    // Precompute suffix maximum: for each position, what's the max digit from that position onwards
    // This allows O(n) instead of O(n²) by avoiding repeated scans
    let mut suffix_max = vec![0u8; bytes.len()];
    let mut max_so_far = 0u8;

    // Backward pass: build suffix maximum array
    for i in (0..bytes.len()).rev() {
        if bytes[i].is_ascii_digit() {
            max_so_far = max_so_far.max(bytes[i]);
        }
        suffix_max[i] = max_so_far;
    }

    // Forward pass: find the best pair (d1, d2) where d2 appears after d1
    let mut best_value = 0u64;
    for (i, &d1_byte) in bytes.iter().enumerate() {
        if !d1_byte.is_ascii_digit() {
            continue;
        }

        // Early termination: if we've found 99 (maximum possible), we're done
        if best_value == 99 {
            break;
        }

        // Use precomputed suffix maximum for position after current
        if i + 1 < bytes.len() && suffix_max[i + 1] > 0 {
            let d1 = (d1_byte - b'0') as u64;
            let d2 = (suffix_max[i + 1] - b'0') as u64;
            let value = d1 * 10 + d2;
            best_value = best_value.max(value);
        }
    }

    if best_value > 0 {
        Some(best_value)
    } else {
        None
    }
}

fn max_of_window(substring: &str) -> Option<(usize, u64)> {
    let bytes = substring.as_bytes();
    let max_entry = bytes
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b.is_ascii_digit())
        // Use max_by for custom tie-breaking logic, so we get the leftmost occurrence with the largest value
        .max_by(|(i_a, a_byte), (i_b, b_byte)| {
            // Convert bytes to actual digit values
            let val_a = *a_byte - b'0';
            let val_b = *b_byte - b'0';

            // 1. Prioritize the LARGER digit value (val_a vs val_b)
            // 2. If the values are equal (tie), prioritize the SMALLER index (leftmost occurrence)
            val_a.cmp(&val_b).then_with(|| i_b.cmp(&i_a))
        });

    max_entry.map(|(i, &val)| (i, (val - b'0') as u64))
}

fn find_12_cell_joltage(line: &str) -> Option<u64> {
    let mut volts = Vec::<u64>::with_capacity(12);
    let mut current_index = 0;
    // Use a while loop because the number of iterations depends on the output of each step
    while volts.len() < 12 {
        let cells_needed = 12 - volts.len();

        // Calculate minimum required elements to leave behind:
        // We need 'cells_needed' in total, and the current iteration will provide 1.
        // So, we must leave 'cells_needed - 1' elements after the current selection.
        let min_to_leave = cells_needed - 1;

        let remaining_len = line.len() - current_index;

        // Calculate the maximum length of the window we are allowed to search
        // The search window must end right before the 'min_to_leave' section starts.
        let window_search_len = remaining_len.checked_sub(min_to_leave).unwrap_or(0);

        // If the window size is zero or less, we cannot make a valid selection
        // and still guarantee the remaining cells, so we break.
        if window_search_len == 0 {
            println!("BREAK: Cannot guarantee 12 elements. Needed: {cells_needed}");
            break;
        }

        // Slice the line to the calculated search window size
        let window_slice = &line[current_index..current_index + window_search_len];

        // println!("window_slice: {window_slice}");

        if let Some((relative_index, max)) = max_of_window(window_slice) {
            volts.push(max);
            // Advance the current position to the digit *after* the one we just consumed.
            // This is the correct greedy advancement for extraction problems.
            current_index += relative_index + 1;
            // println!("new_start_index: {current_index}, max_index: {relative_index}, max: {max}");
        }
    }
    // println!("volts: {volts:?}");

    // Combine the volts into a single number
    let combined = volts.iter().fold(0, |acc, x| acc * 10 + x);
    Some(combined)
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .par_lines()
        .map(|line: &str| find_max_joltage(line).unwrap_or(0))
        .sum();
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: u64 = input
        .par_lines()
        .map(|line: &str| {
            // println!("line: {line}");
            let max = find_12_cell_joltage(line).unwrap_or(0);
            // println!("max: {max}");
            // println!("------------");
            max
        })
        .sum();
    Some(sum as u64)
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
