use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    pub fn euclidean_distance(&self, other: &JunctionBox) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)).sqrt()
    }
}
impl From<Vec<i64>> for JunctionBox {
    fn from(v: Vec<i64>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

// Union-Find structure for managing circuits (sets)
struct UnionFind {
    parent: HashMap<JunctionBox, JunctionBox>,
    size: HashMap<JunctionBox, u64>,
}

impl UnionFind {
    fn new(points: &[JunctionBox]) -> Self {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for &p in points {
            parent.insert(p, p); // Each point is its own parent initially
            size.insert(p, 1); // Each set has size 1
        }
        UnionFind { parent, size }
    }

    // Find the representative (root) of the set containing p (with path compression)
    fn find(&mut self, p: JunctionBox) -> JunctionBox {
        if self.parent[&p] == p {
            p
        } else {
            let root = self.find(self.parent[&p]);
            self.parent.insert(p, root); // Path compression
            root
        }
    }

    // Union the sets containing p1 and p2 (union by size)
    // Returns true if a merge happened, false if already connected
    fn union(&mut self, p1: JunctionBox, p2: JunctionBox) -> bool {
        let root1 = self.find(p1);
        let root2 = self.find(p2);

        if root1 != root2 {
            let size1 = *self.size.get(&root1).unwrap();
            let size2 = *self.size.get(&root2).unwrap();

            // Merge smaller tree into larger tree (Union by Size)
            if size1 < size2 {
                self.parent.insert(root1, root2);
                self.size.insert(root2, size1 + size2);
            } else {
                self.parent.insert(root2, root1);
                self.size.insert(root1, size1 + size2);
            }
            true // Merge successful
        } else {
            false // Already connected
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct ClosestPairResult {
    distance: f64,
    p1: JunctionBox,
    p2: JunctionBox,
}
impl ClosestPairResult {
    fn new(p1: JunctionBox, p2: JunctionBox, distance: f64) -> Self {
        Self { distance, p1, p2 }
    }
}

fn find_all_pairs(points: &[JunctionBox]) -> Vec<ClosestPairResult> {
    let mut all_pairs = Vec::new();
    // Brute force iterates through all unique pairs (i, j) where i < j
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let d = p1.euclidean_distance(&p2);
            // Since we need all pairs, we just push the result
            all_pairs.push(ClosestPairResult::new(p1, p2, d));
        }
    }
    all_pairs
}

fn parse_junctions(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<JunctionBox>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    // NOTE: Important to change this for the test vs the example!
    let number_of_edges_to_process = 10;

    let all_junctions = parse_junctions(input);

    // ** Kruskal's Algorithm **
    // 1. Find all unique pairs of junctions
    let mut all_pairs = find_all_pairs(&all_junctions);

    // 2. Sort all unique pairs by distance
    // all_pairs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    // 2. Sort all unique pairs by distance
    all_pairs.sort_by(|a, b| {
        // 1. Primary sort key: Distance (f64)
        let dist_cmp = a.distance.partial_cmp(&b.distance).unwrap();

        // 2. Secondary sort key: Only if distances are equal (tie-breaker)
        if dist_cmp == std::cmp::Ordering::Equal {
            // Compare the points themselves. Since JunctionBox is PartialOrd/Ord,
            // this provides a stable, deterministic secondary sort key.
            // We use P1, then P2 to break the tie completely.
            a.p1.cmp(&b.p1).then_with(|| a.p2.cmp(&b.p2))
        } else {
            dist_cmp
        }
    });

    // 3. Initialize and Run Union-Find
    let mut uf = UnionFind::new(&all_junctions);

    // Iterate over the first N edges, regardless of merge success
    for result in all_pairs.into_iter().take(number_of_edges_to_process) {
        // Perform the union for the shortest 10 edges.
        // If they are already connected, union() returns false, but the edge is "made".
        uf.union(result.p1, result.p2);
    }

    // for (p, root) in uf.parent.iter() {
    //     println!("p: {:?}, root: {:?}", p, root);
    // }

    // Calculate Final Result (sizes of the circuits)
    // Iterate over all points and find the root, then count sizes.
    let mut root_sizes: HashMap<JunctionBox, u64> = HashMap::new();
    for &p in &all_junctions {
        let root = uf.find(p); // Use the Find method to get the final root
        *root_sizes.entry(root).or_insert(0) += 1;
    }

    let mut final_circuit_sizes: Vec<u64> = root_sizes.into_values().collect();

    // Sort to get the largest circuits in descending order
    final_circuit_sizes.sort_by_key(|&size| std::cmp::Reverse(size));

    // Optional: Print the sizes of the circuits after 10 connections
    // println!("Sizes of all final circuits: {:?}", final_circuit_sizes);

    // "Multiplying together the sizes of the three largest circuits"
    let product: u64 = final_circuit_sizes.iter().take(3).product();

    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    // 1. Parsing and Setup
    let all_junctions = parse_junctions(input);

    let num_junctions = all_junctions.len();

    // 2. Kruskal's Step 1: Find all unique pairs of junctions (O(N^2))
    let mut all_pairs = find_all_pairs(&all_junctions);

    // 3. Kruskal's Step 2: Sort all edges by distance (O(E log E))
    // Use the robust f64 sort with the coordinate tie-breaker.
    all_pairs.sort_by(|a, b| {
        let dist_cmp = a.distance.partial_cmp(&b.distance).unwrap();
        if dist_cmp == std::cmp::Ordering::Equal {
            a.p1.cmp(&b.p1).then_with(|| a.p2.cmp(&b.p2))
        } else {
            dist_cmp
        }
    });

    // 4. Kruskal's Step 3: Initialize and Run Union-Find
    let mut uf = UnionFind::new(&all_junctions);
    let mut edges_connected = 0;
    let required_connections = num_junctions - 1; // MST requires V-1 edges

    let mut last_connection: Option<ClosestPairResult> = None;

    for result in all_pairs.into_iter() {
        // Try to connect the pair (Union)
        if uf.union(result.p1, result.p2) {
            // A new edge was successfully added and two circuits merged
            edges_connected += 1;
            last_connection = Some(result);

            if edges_connected == required_connections {
                // All junctions are now in a single circuit (the MST is complete)
                break;
            }
        }
    }

    // 5. Calculate Final Result (Multiplication)
    if let Some(final_edge) = last_connection {
        // The last edge that completed the single circuit
        let x1 = final_edge.p1.x;
        let x2 = final_edge.p2.x;

        // Multiply the X coordinates of the last two junction boxes connected
        let product = x1 * x2;

        // println!(
        //     "The last connection to complete the circuit was between {:?} and {:?}",
        //     final_edge.p1, final_edge.p2
        // );
        // println!("X1 * X2 = {} * {} = {}", x1, x2, product);

        // Return the product as u64
        Some(product as u64)
    } else {
        // Should not happen for a connected set of points
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
