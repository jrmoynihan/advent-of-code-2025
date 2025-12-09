advent_of_code::solution!(5);

fn capture_id_ranges_and_ingredient_ids(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    input
        .split_once("\n\n")
        .map(|(id_ranges, ingredient_ids)| {
            (
                id_ranges
                    .split_whitespace()
                    .filter_map(|range| {
                        range.split_once('-').and_then(|(start, end)| {
                            let start = start.parse::<u64>().ok()?;
                            let end = end.parse::<u64>().ok()?;
                            Some((start, end))
                        })
                    })
                    .collect(),
                ingredient_ids
                    .split_whitespace()
                    .map(|id| id.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .unwrap()
}

fn sort_id_ranges(id_ranges: &mut Vec<(u64, u64)>) {
    id_ranges.sort_by_key(|(start, _)| *start);
}

fn merge_overlapping_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
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
    merged_ranges
}

pub fn part_one(input: &str) -> Option<u64> {
    // Capture id ranges and ingredient IDs from input
    let (mut id_ranges, ingredient_ids) = capture_id_ranges_and_ingredient_ids(input);

    // Sort ranges by start value
    sort_id_ranges(&mut id_ranges);

    // Merge overlapping ranges
    let merged_ranges = merge_overlapping_ranges(id_ranges);

    // Find the ingredient IDs that are in any of the merged ranges
    // Use binary search since ranges are sorted and non-overlapping
    let fresh_ingredient_id_count = ingredient_ids
        .iter()
        .filter(|id| {
            // Binary search for the rightmost range where start <= id
            // Since ranges are sorted by start and non-overlapping,
            // we need to find the range that could contain this id
            let id_val = **id;
            let idx = merged_ranges
                .binary_search_by_key(&id_val, |(start, _)| *start)
                .unwrap_or_else(|idx| idx.saturating_sub(1));

            // Check if id falls within the range at idx or idx-1
            // (idx might point to the range after, so check idx-1)
            (idx > 0 && {
                let (start, end) = merged_ranges[idx - 1];
                id_val >= start && id_val <= end
            }) || (idx < merged_ranges.len() && {
                let (start, end) = merged_ranges[idx];
                id_val >= start && id_val <= end
            })
        })
        .count();

    // Return the ingredient ID count
    Some(fresh_ingredient_id_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Capture id ranges and ingredient IDs from input
    let (mut id_ranges, _) = capture_id_ranges_and_ingredient_ids(input);

    // Sort ranges by start value
    sort_id_ranges(&mut id_ranges);

    // Merge overlapping ranges
    let merged_ranges = merge_overlapping_ranges(id_ranges);

    // Find the total number of unique ingredient IDs in the merged ranges
    let fresh_ingredient_id_count = merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();
    Some(fresh_ingredient_id_count)
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
        println!("Result: {:?}", result);
        assert_eq!(result, Some(14));
    }
}
