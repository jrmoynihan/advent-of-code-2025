advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut splits: u64 = 0;
    let mut lines = input.lines();
    // Create a vector of bools for the beam positions
    let first_line = lines.next().unwrap();
    let mut beam_positions_on_this_line = vec![false; first_line.len()];
    // Set the starting beam position
    for (i, c) in first_line.chars().enumerate() {
        if c == 'S' {
            beam_positions_on_this_line[i] = true;
        }
    }
    // Process the rest of the lines
    for line in lines {
        let bytes = line.as_bytes();
        for (i, c) in bytes.iter().enumerate() {
            if *c == b'^' && beam_positions_on_this_line[i] == true {
                splits += 1;
                beam_positions_on_this_line[i] = false;
                beam_positions_on_this_line[i + 1] = true;
                beam_positions_on_this_line[i - 1] = true;
            }
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    // Create a vector of bools for the beam positions
    let first_line = lines.next().unwrap();
    let mut possible_beams = vec![0; first_line.len()];
    // Set the starting beam position
    for (i, c) in first_line.chars().enumerate() {
        if c == 'S' {
            possible_beams[i] = 1;
        }
    }
    // Process the rest of the lines
    for line in lines {
        let bytes = line.as_bytes();
        for (i, c) in bytes.iter().enumerate() {
            if *c == b'^' && possible_beams[i] > 0 {
                possible_beams[i + 1] += possible_beams[i];
                possible_beams[i - 1] += possible_beams[i];
                possible_beams[i] = 0;
            }
        }
    }

    Some(possible_beams.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(40));
    }
}
