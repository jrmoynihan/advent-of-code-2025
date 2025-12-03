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
            let id_str = id.to_string();
            let len = id_str.len();
            if len % 2 != 0 {
                continue;
            }
            // Split the id in half and compare string slices directly
            // Since there are no leading zeros, string equality implies number equality
            let (left, right) = id_str.split_at(len / 2);
            if left == right {
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
            let id_str = id.to_string();
            let len = id_str.len();
            let bytes = id_str.as_bytes();

            // Only check divisors of len (skip len itself since that would be 1 chunk)
            // Iterate from largest to smallest divisor for early exit
            for size in (1..len).rev() {
                // Skip if size doesn't divide len evenly
                if len % size != 0 {
                    continue;
                }

                // Compare all chunks to the first chunk
                let first_chunk = &bytes[..size];
                if bytes.chunks(size).skip(1).all(|chunk| chunk == first_chunk) {
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
