advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    // Capture id ranges and ingredient IDs from input
    let (id_ranges, ingredient_ids) = input.split_once("\n").unwrap();
    // Merge overlapping id ranges
    let id_ranges = id_ranges
    .split_whitespace()
    .map(|range| range.split_once('-').map(|(start, end)| {

    }))
    // Merge overlapping id ranges
    let id_ranges = merge_overlapping_id_ranges(id_ranges);
    let ingredient_ids = ingredient_ids.split_whitespace().map(|id| id.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    // Find the ingredient ID that is not in any of the id ranges
    let ingredient_id = ingredient_ids.iter().find(|id| !id_ranges.iter().any(|range| range.contains(id))).unwrap();
    // Return the ingredient ID
    Some(*ingredient_id)
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
