use rayon::prelude::*;

advent_of_code::solution!(2);

#[inline]
fn is_palindrome_number(id: u64) -> bool {
    // Extract digits into a small array (max ~20 digits for u64)
    let mut digits = [0u8; 20];
    let mut len = 0;
    let mut temp = id;
    while temp > 0 {
        digits[len] = (temp % 10) as u8;
        temp /= 10;
        len += 1;
    }
    if len % 2 != 0 {
        return false;
    }
    // Compare first half with second half (reversed)
    let half = len / 2;
    for i in 0..half {
        if digits[i] != digits[half + i] {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.split(',').collect();

    let sum: u64 = ranges
        .par_iter()
        .map(|range| {
            // Split the range into start and end by a '-' delimiter
            let mut parts = range.split('-').map(|s| s.trim().parse::<u64>().unwrap());
            let start = parts.next().unwrap();
            let end = parts.next().unwrap();

            let mut range_sum = 0u64;
            for id in start..=end {
                if is_palindrome_number(id) {
                    range_sum += id;
                }
            }
            range_sum
        })
        .sum();

    Some(sum)
}

#[inline]
fn has_repeating_pattern(id: u64) -> bool {
    // Extract digits
    let mut digits = [0u8; 20];
    let mut len = 0;
    let mut temp = id;
    while temp > 0 {
        digits[len] = (temp % 10) as u8;
        temp /= 10;
        len += 1;
    }

    // Check divisors from largest to smallest
    for size in (1..len).rev() {
        if len % size != 0 {
            continue;
        }

        // Compare chunks
        let first_chunk = &digits[..size];
        let mut all_match = true;
        for chunk_start in (size..len).step_by(size) {
            if &digits[chunk_start..chunk_start + size] != first_chunk {
                all_match = false;
                break;
            }
        }
        if all_match {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.split(',').collect();

    let sum: u64 = ranges
        .par_iter()
        .map(|range| {
            // Split the range into start and end by a '-' delimiter
            let mut parts = range.split('-').map(|s| s.trim().parse::<u64>().unwrap());
            let start = parts.next().unwrap();
            let end = parts.next().unwrap();

            let mut range_sum = 0u64;
            for id in start..=end {
                if has_repeating_pattern(id) {
                    range_sum += id;
                }
            }
            range_sum
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
