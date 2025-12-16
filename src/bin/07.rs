advent_of_code::solution!(7);

/// Bit-packed beam state using 3 u64s (supports up to 192 positions, actual: 141)
#[derive(Clone, Copy)]
struct BeamState {
    chunks: [u64; 3],
}

impl BeamState {
    fn new() -> Self {
        Self { chunks: [0; 3] }
    }

    #[inline]
    fn get(&self, pos: usize) -> bool {
        let chunk_idx = pos / 64;
        let bit_idx = pos % 64;
        (self.chunks[chunk_idx] & (1u64 << bit_idx)) != 0
    }

    #[inline]
    fn set(&mut self, pos: usize) {
        let chunk_idx = pos / 64;
        let bit_idx = pos % 64;
        self.chunks[chunk_idx] |= 1u64 << bit_idx;
    }

    #[inline]
    fn clear(&mut self, pos: usize) {
        let chunk_idx = pos / 64;
        let bit_idx = pos % 64;
        self.chunks[chunk_idx] &= !(1u64 << bit_idx);
    }

    #[inline]
    fn split(&mut self, pos: usize) {
        self.clear(pos);
        self.set(pos - 1);
        self.set(pos + 1);
    }
}

/// OLD implementation using Vec<bool> for comparison
fn part_one_old(input: &str) -> Option<u64> {
    let mut splits: u64 = 0;
    let mut lines = input.lines();
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
            if *c == b'^' && beam_positions_on_this_line[i] {
                splits += 1;
                beam_positions_on_this_line[i] = false;
                beam_positions_on_this_line[i + 1] = true;
                beam_positions_on_this_line[i - 1] = true;
            }
        }
    }

    Some(splits)
}

/// NEW implementation using bit-packed BeamState
pub fn part_one(input: &str) -> Option<u64> {
    let mut splits: u64 = 0;
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();

    // Use bit-packed version for better cache performance
    let mut beams = BeamState::new();

    // Set the starting beam position
    for (i, c) in first_line.chars().enumerate() {
        if c == 'S' {
            beams.set(i);
        }
    }

    // Process the rest of the lines
    for line in lines {
        let bytes = line.as_bytes();
        for (i, &c) in bytes.iter().enumerate() {
            if c == b'^' && beams.get(i) {
                splits += 1;
                beams.split(i);
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

    #[test]
    fn benchmark_old_vs_new() {
        use std::time::Instant;

        let input = advent_of_code::template::read_file("inputs", DAY);
        let lines_count = input.lines().count();
        let width = input.lines().next().unwrap().len();

        // Warmup
        let _ = part_one_old(&input);
        let _ = part_one(&input);

        // Benchmark OLD implementation (Vec<bool>)
        let iterations = 100;
        let start_old = Instant::now();
        for _ in 0..iterations {
            let _ = part_one_old(&input);
        }
        let duration_old = start_old.elapsed() / iterations;

        // Benchmark NEW implementation (bit-packed)
        let start_new = Instant::now();
        for _ in 0..iterations {
            let _ = part_one(&input);
        }
        let duration_new = start_new.elapsed() / iterations;

        // Verify results match
        let result_old = part_one_old(&input);
        let result_new = part_one(&input);
        assert_eq!(result_old, result_new, "Results don't match!");

        println!("\n=== Day 7 Part 1 Performance Comparison ===");
        println!("Grid size: {} lines × {} columns", lines_count, width);
        println!("Iterations: {}", iterations);
        println!("\nOLD (Vec<bool>):  {:?}", duration_old);
        println!("NEW (bit-packed): {:?}", duration_new);
        println!(
            "\nSpeedup: {:.2}x",
            duration_old.as_secs_f64() / duration_new.as_secs_f64()
        );
        println!("Time saved: {:?} per run", duration_old - duration_new);

        // Memory comparison
        println!("\n=== Memory Usage ===");
        println!("OLD: {} bytes (Vec<bool> + metadata)", width + 24);
        println!("NEW: 24 bytes (3 × u64)");
        println!(
            "Savings: {:.1}% reduction",
            100.0 * (1.0 - 24.0 / (width + 24) as f64)
        );
    }
}
