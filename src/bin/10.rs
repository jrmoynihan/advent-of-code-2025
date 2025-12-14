advent_of_code::solution!(10);

/// Solves a system of linear equations over GF(2) using Gaussian elimination.
/// Returns the minimum number of button presses needed.
fn solve_gf2(buttons: &[Vec<bool>], target: &[bool]) -> Option<u64> {
    let n = target.len(); // number of lights (rows)
    let m = buttons.len(); // number of buttons (columns)

    // Create augmented matrix: each row is a light, each column is a button
    // Last column is the target state for that light
    let mut matrix = vec![vec![false; m + 1]; n];

    for row in 0..n {
        for col in 0..m {
            matrix[row][col] = buttons[col][row];
        }
        matrix[row][m] = target[row];
    }

    println!("\n=== Initial Matrix ===");
    print_matrix(&matrix, m);

    // Track which button (column) is the pivot for each row
    let mut pivot_col = vec![None; n];
    let mut pivot_row_for_col = vec![None; m];
    let mut current_row = 0;

    // Forward elimination to Reduced Row Echelon Form
    for col in 0..m {
        // Find a pivot in this column (starting from current_row)
        let mut found_pivot = None;
        for row in current_row..n {
            if matrix[row][col] {
                found_pivot = Some(row);
                break;
            }
        }

        let Some(pivot) = found_pivot else {
            // No pivot in this column, it's a free variable
            continue;
        };

        // Swap rows to bring pivot to current_row
        if pivot != current_row {
            matrix.swap(current_row, pivot);
        }

        pivot_col[current_row] = Some(col);
        pivot_row_for_col[col] = Some(current_row);

        println!("\n=== Processing column {} (Button {}) ===", col, col);
        println!("Pivot at row {}", current_row);

        // Eliminate all other 1s in this column (both above and below)
        for row in 0..n {
            if row != current_row && matrix[row][col] {
                println!("Eliminating row {} using pivot row {}", row, current_row);
                // XOR this row with the pivot row
                for c in 0..=m {
                    matrix[row][c] ^= matrix[current_row][c];
                }
            }
        }

        print_matrix(&matrix, m);

        current_row += 1;
        if current_row >= n {
            break;
        }
    }

    println!("\n=== Final RREF Matrix ===");
    print_matrix(&matrix, m);

    // Check for inconsistency: row with all zeros except target
    for row in 0..n {
        let all_zeros = (0..m).all(|col| !matrix[row][col]);
        if all_zeros && matrix[row][m] {
            println!("INCONSISTENT: Row {} is [0 0 0 ... | 1]", row);
            return None;
        }
    }

    // Extract solution: read directly from RREF
    // For each pivot column, check if we need to press that button
    let mut solution = vec![false; m];

    for row in 0..n {
        if let Some(col) = pivot_col[row] {
            // In RREF, this row has the form: [0 ... 1 ... 0 | target_bit]
            // The 1 is in position 'col', so button 'col' should be pressed
            // if and only if target_bit is 1
            solution[col] = matrix[row][m];
        }
    }

    println!("\n=== Solution Vector ===");
    println!("Buttons to press: {:?}", solution);
    let count = solution.iter().filter(|&&x| x).count() as u64;
    println!("Total presses: {}", count);

    Some(count)
}

fn print_matrix(matrix: &[Vec<bool>], num_buttons: usize) {
    print!("        ");
    for i in 0..num_buttons {
        print!("B{:<2} ", i);
    }
    println!("| T");
    println!("        {}", "-".repeat(num_buttons * 4 + 4));

    for (i, row) in matrix.iter().enumerate() {
        print!("Row {}: ", i);
        for j in 0..num_buttons {
            print!(" {}  ", if row[j] { '1' } else { '0' });
        }
        print!("|  {}", if row[num_buttons] { '1' } else { '0' });
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    let machines = input.lines().collect::<Vec<&str>>();
    // For each machine (line)
    for machine in machines {
        // Get the indicator lights by counting the characters in between the "[" and "]"
        let (indicator_lights, rest) = machine.split_once("]")?;
        let indicator_lights = indicator_lights.strip_prefix("[")?;

        let target_light_state: Vec<bool> = indicator_lights.chars().map(|c| c == '#').collect();

        let (buttons_str, _joltages_str) = rest.split_once("{")?;
        let buttons: Vec<Vec<bool>> = buttons_str
            .split_whitespace()
            .filter(|s| s.starts_with('('))
            .map(|b| {
                let indices: Vec<usize> = b
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();
                let mut vec = vec![false; target_light_state.len()];
                for i in indices {
                    vec[i] = true;
                }
                vec
            })
            .collect();

        // Brute force: try all 2^m combinations
        let min_presses = solve_all_possible_button_combinations(&buttons, &target_light_state)?;
        total += min_presses;
    }
    Some(total)
}

/// Try all possible combinations of button presses (0 or 1 times each)
/// and find the minimum number of presses needed.
fn solve_all_possible_button_combinations(buttons: &[Vec<bool>], target: &[bool]) -> Option<u64> {
    let num_buttons = buttons.len();
    let num_lights = target.len();

    // Try all 2^num_buttons combinations
    let mut min_presses: Option<u64> = None;

    for combination in 0..(1 << num_buttons) {
        // Start with all lights off
        let mut state = vec![false; num_lights];

        // For each button, check if we press it in this combination
        let mut presses = 0;
        for button_idx in 0..num_buttons {
            // Check if bit button_idx is set in combination
            if (combination & (1 << button_idx)) != 0 {
                // Press this button: XOR its effects into state
                for light_idx in 0..num_lights {
                    state[light_idx] ^= buttons[button_idx][light_idx];
                }
                presses += 1;
            }
        }

        // Check if this combination achieves the target
        if state == target {
            min_presses = Some(match min_presses {
                None => presses,
                Some(current_min) => current_min.min(presses),
            });
        }
    }

    min_presses
}

/// Solve using BFS to find minimum button presses.
/// Treat this as a shortest path problem where each state is the current joltage levels.
fn bfs_solve(buttons: &[Vec<i64>], target: &[i64]) -> Option<u64> {
    use std::collections::{HashMap, VecDeque};

    let n = target.len();

    // Precompute: for each counter, which buttons can affect it and by how much
    let mut max_increment = vec![0i64; n];
    for button in buttons {
        for i in 0..n {
            max_increment[i] = max_increment[i].max(button[i]);
        }
    }

    // Early check: ensure all counters can be reached
    for i in 0..n {
        if target[i] > 0 && max_increment[i] == 0 {
            return None; // Counter i can't be increased
        }
    }

    // BFS with memoization
    let mut queue = VecDeque::new();
    let mut memo: HashMap<Vec<i64>, u64> = HashMap::new();

    let initial = vec![0i64; n];
    queue.push_back((initial.clone(), 0u64));
    memo.insert(initial, 0);

    while let Some((state, presses)) = queue.pop_front() {
        // Check if reached target
        if state == *target {
            return Some(presses);
        }

        // Skip if we've found a better path to this state
        if let Some(&best) = memo.get(&state) {
            if presses > best {
                continue;
            }
        }

        // Try each button
        for button in buttons {
            let mut next = state.clone();
            let mut valid = true;
            let mut progress = false;

            for i in 0..n {
                next[i] += button[i];
                if next[i] > target[i] {
                    valid = false;
                    break;
                }
                // Track if this button makes progress toward target
                if button[i] > 0 && state[i] < target[i] {
                    progress = true;
                }
            }

            // Optimization: only press buttons that make progress
            if !valid || !progress {
                continue;
            }

            let new_presses = presses + 1;

            // Only explore if we haven't seen this state or found a better path
            let should_visit = match memo.get(&next) {
                None => true,
                Some(&best) => new_presses < best,
            };

            if should_visit {
                memo.insert(next.clone(), new_presses);
                queue.push_back((next, new_presses));
            }
        }
    }

    None
}

// Solve using A* search with a heuristic to find minimum button presses efficiently.
fn a_star_solve(buttons: &[Vec<i64>], target: &[i64]) -> Option<u64> {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    let n = target.len();

    // Calculate heuristic: minimum presses needed for each counter independently
    let mut min_presses_per_counter = vec![i64::MAX; n];
    for (i, &target_val) in target.iter().enumerate() {
        for button in buttons {
            if button[i] > 0 {
                let presses_needed = (target_val + button[i] - 1) / button[i]; // Ceiling division
                min_presses_per_counter[i] = min_presses_per_counter[i].min(presses_needed);
            }
        }
    }

    // Heuristic: sum of minimum presses needed for each counter
    let heuristic = |state: &[i64]| -> i64 {
        let mut h = 0;
        for i in 0..n {
            let remaining = target[i] - state[i];
            if remaining > 0 && min_presses_per_counter[i] != i64::MAX {
                // Estimate minimum presses to add 'remaining' to counter i
                h += (remaining + min_presses_per_counter[i] - 1) / min_presses_per_counter[i];
            }
        }
        h
    };

    // A* search: priority queue ordered by (presses + heuristic)
    // Using Reverse to make BinaryHeap a min-heap
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();

    let initial_state = vec![0i64; n];
    let initial_h = heuristic(&initial_state);

    heap.push((Reverse(initial_h), 0u64, initial_state.clone()));
    visited.insert(initial_state, 0u64);

    while let Some((Reverse(_priority), presses, state)) = heap.pop() {
        // Check if we've reached the target
        if state == *target {
            return Some(presses);
        }

        // Skip if we've found a better path to this state
        if let Some(&best_presses) = visited.get(&state) {
            if presses > best_presses {
                continue;
            }
        }

        // Try pressing each button
        for button in buttons {
            let mut new_state = state.clone();
            let mut valid = true;

            // Apply button effect
            for i in 0..n {
                new_state[i] += button[i];
                // Prune: don't exceed target
                if new_state[i] > target[i] {
                    valid = false;
                    break;
                }
            }

            if !valid {
                continue;
            }

            let new_presses = presses + 1;

            // Only explore if we haven't seen this state with fewer presses
            let should_explore = match visited.get(&new_state) {
                None => true,
                Some(&best) => new_presses < best,
            };

            if should_explore {
                visited.insert(new_state.clone(), new_presses);
                let h = heuristic(&new_state);
                let priority = new_presses as i64 + h;
                heap.push((Reverse(priority), new_presses, new_state));
            }
        }
    }

    None // No solution found
}

/// Solve Part 2 using the recursive parity approach.
/// This brilliantly connects Part 1 (parity) with Part 2 (counters).
fn recursive_parity_solve(buttons: &[Vec<i64>], target: &[i64]) -> Option<u64> {
    use std::collections::HashMap;

    let n = target.len();

    // Precompute all possible button press patterns and their effects
    // Group by parity pattern for fast lookup
    let mut patterns: HashMap<Vec<i64>, Vec<(Vec<i64>, u64)>> = HashMap::new();

    // Try all 2^m combinations of pressing buttons 0 or 1 times
    let num_buttons = buttons.len();
    for mask in 0..(1 << num_buttons) {
        let mut joltages = vec![0i64; n];
        let mut presses = 0u64;

        for btn_idx in 0..num_buttons {
            if (mask & (1 << btn_idx)) != 0 {
                for i in 0..n {
                    joltages[i] += buttons[btn_idx][i];
                }
                presses += 1;
            }
        }

        // Compute parity pattern
        let parity: Vec<i64> = joltages.iter().map(|&x| x % 2).collect();

        patterns
            .entry(parity)
            .or_insert_with(Vec::new)
            .push((joltages, presses));
    }

    // Recursive solver with memoization
    let mut memo: HashMap<Vec<i64>, Option<u64>> = HashMap::new();

    fn solve_recursive(
        target: &[i64],
        patterns: &HashMap<Vec<i64>, Vec<(Vec<i64>, u64)>>,
        memo: &mut HashMap<Vec<i64>, Option<u64>>,
    ) -> Option<u64> {
        // Base case: all zeros
        if target.iter().all(|&x| x == 0) {
            return Some(0);
        }

        // Check for negative values
        if target.iter().any(|&x| x < 0) {
            return None;
        }

        // Check memo
        if let Some(&result) = memo.get(target) {
            return result;
        }

        // Compute parity of target
        let parity: Vec<i64> = target.iter().map(|&x| x % 2).collect();

        // Try all patterns matching this parity
        let mut best: Option<u64> = None;

        if let Some(pattern_list) = patterns.get(&parity) {
            for (joltages, presses) in pattern_list {
                // Check if this pattern doesn't exceed target
                let valid = target.iter().zip(joltages.iter()).all(|(&t, &j)| j <= t);

                if !valid {
                    continue;
                }

                // Compute new target: (target - joltages) / 2
                let new_target: Vec<i64> = target
                    .iter()
                    .zip(joltages.iter())
                    .map(|(&t, &j)| (t - j) / 2)
                    .collect();

                // Recursively solve
                if let Some(recursive_cost) = solve_recursive(&new_target, patterns, memo) {
                    let total_cost = presses + 2 * recursive_cost;
                    best = Some(match best {
                        None => total_cost,
                        Some(current) => current.min(total_cost),
                    });
                }
            }
        }

        memo.insert(target.to_vec(), best);
        best
    }

    solve_recursive(target, &patterns, &mut memo)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;

    for machine in input.lines() {
        let (_, rest) = machine.split_once("]")?;
        let (buttons_str, joltages_str) = rest.split_once("{")?;
        let joltages_str = joltages_str.strip_suffix("}")?;

        // Parse target joltages
        let target: Vec<i64> = joltages_str
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        // Parse buttons - now as integer vectors
        let buttons: Vec<Vec<i64>> = buttons_str
            .split_whitespace()
            .filter(|s| s.starts_with('('))
            .map(|b| {
                let indices: Vec<usize> = b
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();
                let mut vec = vec![0i64; target.len()];
                for i in indices {
                    vec[i] = 1;
                }
                vec
            })
            .collect();

        // Solve using recursive parity (even/odd) approach
        let min_presses = recursive_parity_solve(&buttons, &target)?;

        total += min_presses;
    }

    Some(total)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part2_first_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(part_two(input), Some(10));
    }

    #[test]
    fn test_part2_second_machine() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        assert_eq!(part_two(input), Some(12));
    }

    #[test]
    fn test_part2_third_machine() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part_two(input), Some(11));
    }
}
