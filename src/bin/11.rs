use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

/// Recursive DFS function with backtracking
fn dfs_count_paths<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    current_node: &'a str,
    destination: &'a str,
    mut visited: HashSet<&'a str>, // We modify this copy for the current branch
) -> u64 {
    // 1. Mark the current node as visited for this path
    visited.insert(current_node);
    let mut path_count = 0;

    // 2. Base Case: If we reached the destination, we found one complete valid path.
    if current_node == destination {
        // Return 1 for this single successful path
        return 1;
    }

    // 3. Recursive Step: Explore all neighbors
    if let Some(neighbors) = graph.get(current_node) {
        for neighbor in neighbors {
            // Only continue down this branch if the neighbor hasn't been visited yet
            if !visited.contains(neighbor) {
                // Recursively count paths from the neighbor to the destination
                // We pass a CLONE of the 'visited' set to ensure backtracking works correctly,
                // so subsequent neighbors of the current_node don't see each other's visited nodes.
                path_count += dfs_count_paths(graph, neighbor, destination, visited.clone());
            }
        }
    }

    // 4. Backtracking handled implicitly by the function scope and the use of 'visited.clone()'
    // When the function returns, the 'visited' set specific to that call frame is dropped,
    // which is the essence of backtracking.

    path_count
}

fn map_nodes(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let right = right.split_whitespace().collect::<Vec<&str>>();
        map.insert(left.trim(), right);
    }
    map
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = map_nodes(input);
    // Call the recursive DFS function
    // Start at "you", target "out", with an empty visited set
    let unique_paths = dfs_count_paths(&map, "you", "out", HashSet::new());
    Some(unique_paths)
}

// Recursive DFS function to count paths that satisfy specific conditions
fn dfs_count_conditional_paths<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    current_node: &'a str,
    destination: &'a str,
    mut visited: HashSet<&'a str>, // The nodes visited *along the current path*
    has_seen_dac: bool,            // Have we visited "dac" yet in this path?
    has_seen_fft: bool,            // Have we visited "fft" yet in this path?
) -> u64 {
    // Mark current node as visited for this recursive branch
    visited.insert(current_node);

    // Update flags if we hit the required nodes
    let new_seen_dac = has_seen_dac || (current_node == "dac");
    let new_seen_fft = has_seen_fft || (current_node == "fft");

    // Base Case: If we reached the destination
    if current_node == destination {
        // Only count this path if BOTH required nodes were seen during the traversal
        return if new_seen_dac && new_seen_fft { 1 } else { 0 };
    }

    let mut path_count = 0;

    // Recursive Step: Explore all neighbors
    if let Some(neighbors) = graph.get(current_node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                // Recursively call for the neighbor, passing cloned visited set and updated flags
                path_count += dfs_count_conditional_paths(
                    graph,
                    neighbor,
                    destination,
                    visited.clone(), // Backtracking via clone
                    new_seen_dac,    // Pass updated flag
                    new_seen_fft,    // Pass updated flag
                );
            }
        }
    }

    path_count
}

// Memoized recursive DFS function to count paths from 'start' to 'end'
// This assumes the graph is a DAG and has no cycles
fn dfs_memoized_count<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    end: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if current == end {
        return 1;
    }
    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut count = 0;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            count += dfs_memoized_count(graph, neighbor, end, memo);
        }
    }

    memo.insert(current, count);
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = map_nodes(input);

    // let total_paths = dfs_count_conditional_paths(&map, "svr", "out", HashSet::new(), false, false);
    let mut memo1 = HashMap::new();
    let mut memo2 = HashMap::new();
    let mut memo3 = HashMap::new();

    // The order might be DAC then FFT, or FFT then DAC. We assume we must enforce one order for this method to work cleanly.
    // If we assume a canonical order SVR -> DAC -> FFT -> OUT:
    let paths_svr_to_dac = dfs_memoized_count(&map, "svr", "dac", &mut memo1);
    let paths_dac_to_fft = dfs_memoized_count(&map, "dac", "fft", &mut memo2);
    let paths_fft_to_out = dfs_memoized_count(&map, "fft", "out", &mut memo3);

    // And then the other order SVR -> FFT -> DAC -> OUT:
    let paths_svr_to_fft = dfs_memoized_count(&map, "svr", "fft", &mut memo1);
    let paths_fft_to_dac = dfs_memoized_count(&map, "fft", "dac", &mut memo2);
    let paths_dac_to_out = dfs_memoized_count(&map, "dac", "out", &mut memo3);

    // The total is the product of paths in each segment
    let total_paths = (paths_svr_to_dac * paths_dac_to_fft * paths_fft_to_out)
        + (paths_svr_to_fft * paths_fft_to_dac * paths_dac_to_out);

    Some(total_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let test_input = "svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out";
        let result = part_two(&test_input);
        assert_eq!(result, Some(2));
    }
}
