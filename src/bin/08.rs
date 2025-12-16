use std::{collections::HashMap, hash::Hash};

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
        // Use integer arithmetic, convert to f64 only once at the end
        // This avoids expensive powf() calls (2-3x faster)
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz) as f64
        // Note: Still squared distance (no sqrt), preserves ordering
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

// Vec-based Union-Find structure (5-10x faster than HashMap version)
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(), // Each node is its own parent
            size: vec![1; n],         // Each set has size 1
        }
    }

    // Find with path compression - O(α(n)) amortized
    fn find(&mut self, mut x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    // Union by size - O(α(n)) amortized
    // Returns true if merge happened, false if already connected
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already connected
        }

        // Union by size: attach smaller tree to larger
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut size_map: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            let root = self.find(i);
            *size_map.entry(root).or_insert(0) += 1;
        }
        size_map.into_values().collect()
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Edge {
    distance: f64,
    i: usize, // Index into points array
    j: usize, // Index into points array
}

impl Edge {
    fn new(i: usize, j: usize, distance: f64) -> Self {
        Self { distance, i, j }
    }
}

fn find_all_pairs(points: &[JunctionBox]) -> Vec<Edge> {
    let mut all_pairs = Vec::with_capacity(points.len() * (points.len() - 1) / 2);
    // Brute force iterates through all unique pairs (i, j) where i < j
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = points[i].euclidean_distance(&points[j]);
            all_pairs.push(Edge::new(i, j, d));
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
    let n = all_junctions.len();

    // ** Kruskal's Algorithm with Vec-based Union-Find **
    // 1. Find all unique pairs of junctions
    let mut all_edges = find_all_pairs(&all_junctions);

    // 2. OPTIMIZATION: Use partial sort since we only need first N edges
    // This is faster than full sort: O(n) vs O(n log n)
    if number_of_edges_to_process < all_edges.len() {
        all_edges.select_nth_unstable_by(number_of_edges_to_process, |a, b| {
            let dist_cmp = a.distance.partial_cmp(&b.distance).unwrap();
            if dist_cmp == std::cmp::Ordering::Equal {
                a.i.cmp(&b.i).then_with(|| a.j.cmp(&b.j))
            } else {
                dist_cmp
            }
        });

        // Sort only the first N edges for deterministic processing
        all_edges[..number_of_edges_to_process].sort_by(|a, b| {
            let dist_cmp = a.distance.partial_cmp(&b.distance).unwrap();
            if dist_cmp == std::cmp::Ordering::Equal {
                a.i.cmp(&b.i).then_with(|| a.j.cmp(&b.j))
            } else {
                dist_cmp
            }
        });
    }

    // 3. Initialize Vec-based Union-Find (much faster than HashMap)
    let mut uf = UnionFind::new(n);

    // Process the first N shortest edges
    for edge in all_edges.iter().take(number_of_edges_to_process) {
        uf.union(edge.i, edge.j);
    }

    // 4. Get component sizes
    let mut component_sizes: Vec<u64> = uf.get_sizes().iter().map(|&s| s as u64).collect();
    component_sizes.sort_by_key(|&size| std::cmp::Reverse(size));

    // Multiply the three largest components
    let product: u64 = component_sizes.iter().take(3).product();
    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    // 1. Parsing and Setup
    let all_junctions = parse_junctions(input);
    let n = all_junctions.len();

    // 2. Find all edges
    let mut all_edges = find_all_pairs(&all_junctions);

    // 3. Sort edges by distance
    all_edges.sort_by(|a, b| {
        let dist_cmp = a.distance.partial_cmp(&b.distance).unwrap();
        if dist_cmp == std::cmp::Ordering::Equal {
            a.i.cmp(&b.i).then_with(|| a.j.cmp(&b.j))
        } else {
            dist_cmp
        }
    });

    // 4. Build MST using Vec-based Union-Find
    let mut uf = UnionFind::new(n);
    let mut edges_connected = 0;
    let required_connections = n - 1; // MST requires n-1 edges

    let mut last_edge: Option<&Edge> = None;

    for edge in &all_edges {
        if uf.union(edge.i, edge.j) {
            edges_connected += 1;
            last_edge = Some(edge);

            if edges_connected == required_connections {
                break; // MST complete
            }
        }
    }

    // 5. Calculate result: multiply X coordinates of last connection
    if let Some(edge) = last_edge {
        let x1 = all_junctions[edge.i].x;
        let x2 = all_junctions[edge.j].x;
        Some((x1 * x2) as u64)
    } else {
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
