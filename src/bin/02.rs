advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0u64;
    let ranges = input.split(',');
    ranges.for_each(|range| {
        // Split the range into start and end by a '-' delimiter
        let mut parts = range.split('-').map(|s| s.trim().parse::<u64>().unwrap());
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();

        for id in start..=end {
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
                continue;
            }
            // Compare first half with second half (reversed)
            let half = len / 2;
            let mut matches = true;
            for i in 0..half {
                if digits[i] != digits[half + i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                sum += id;
            }
        }
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0u64;
    let ranges = input.split(',');
    ranges.for_each(|range| {
        // Split the range into start and end by a '-' delimiter
        let mut parts = range.split('-').map(|s| s.trim().parse::<u64>().unwrap());
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();

        for id in start..=end {
            // Extract digits
            let mut digits = [0u8; 20];
            let mut len = 0;
            let mut temp = id;
            while temp > 0 {
                digits[len] = (temp % 10) as u8;
                temp /= 10;
                len += 1;
            }

            // Check divisors
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
                    sum += id;
                    break;
                }
            }
        }
    });
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
