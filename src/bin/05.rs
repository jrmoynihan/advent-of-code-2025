advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    // Capture id ranges and ingredient IDs from input
    let (id_ranges, ingredient_ids) = input.split_once("\n\n").unwrap();

    // Parse and validate all ranges first
    let mut ranges: Vec<(u64, u64)> = id_ranges
        .split_whitespace()
        .filter_map(|range| {
            range.split_once('-').and_then(|(start, end)| {
                let start = start.parse::<u64>().ok()?;
                let end = end.parse::<u64>().ok()?;
                if start > end {
                    panic!("Invalid range: start ({start}) > end ({end})");
                }
                Some((start, end))
            })
        })
        .collect();

    // Sort ranges by start value
    ranges.sort_by_key(|(start, _)| *start);

    // Merge overlapping ranges manually
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_last_start, last_end)) = merged_ranges.last_mut() {
            // If current range overlaps or is adjacent to the last merged range, merge them
            // Check if start is within or immediately after the last range
            if start <= *last_end + 1 {
                // Merge: extend the last range if needed
                *last_end = (*last_end).max(end);
            } else {
                // No overlap, add as new range
                merged_ranges.push((start, end));
            }
        } else {
            // First range
            merged_ranges.push((start, end));
        }
    }

    // Parse ingredient IDs
    let ingredient_ids = ingredient_ids
        .split_whitespace()
        .map(|id| id.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Find the ingredient IDs that are in any of the merged ranges
    let fresh_ingredient_id_count = ingredient_ids
        .iter()
        .filter(|id| {
            merged_ranges
                .iter()
                .any(|(start, end)| **id >= *start && **id <= *end)
        })
        .count();

    // Return the ingredient ID count
    Some(fresh_ingredient_id_count as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
