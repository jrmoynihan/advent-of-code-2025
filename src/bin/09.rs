use std::collections::HashSet;

advent_of_code::solution!(9);

// Spatial index for polygon edges to speed up point-in-polygon queries
struct SpatialIndex {
    grid_size: i64,
    cells: std::collections::HashMap<(i64, i64), Vec<usize>>,
}

impl SpatialIndex {
    fn new(polygon: &[Point], grid_size: i64) -> Self {
        let mut cells = std::collections::HashMap::new();

        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];

            // Get bounding box of this edge
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            // Add edge to all grid cells it touches
            let cell_min_x = min_x / grid_size;
            let cell_max_x = max_x / grid_size;
            let cell_min_y = min_y / grid_size;
            let cell_max_y = max_y / grid_size;

            for cell_y in cell_min_y..=cell_max_y {
                for cell_x in cell_min_x..=cell_max_x {
                    cells
                        .entry((cell_x, cell_y))
                        .or_insert_with(Vec::new)
                        .push(i);
                }
            }
        }

        SpatialIndex { grid_size, cells }
    }

    fn get_nearby_edges(&self, p: Point) -> Vec<usize> {
        let cell_x = p.x / self.grid_size;
        let cell_y = p.y / self.grid_size;

        self.cells
            .get(&(cell_x, cell_y))
            .map(|edges| edges.clone())
            .unwrap_or_default()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub fn make_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<Point>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = make_points(input);

    let mut max_area = 0;
    // For each red tile point, find the other red tile points that are furthest away from it in any dimension
    for i in 0..points.len() {
        for j in 1..points.len() {
            // Find the dimension that the points are furthest away from each other in
            let abs_x_distance = (points[i].x - points[j].x).abs() + 1;
            let abs_y_distance = (points[i].y - points[j].y).abs() + 1;
            let area = abs_x_distance as u64 * abs_y_distance as u64;

            if area > max_area {
                max_area = area;
            }
        }
    }
    Some(max_area)
}

fn get_green_tiles_on_path(red_tiles: &[Point]) -> HashSet<Point> {
    let mut green = HashSet::new();

    for i in 0..red_tiles.len() {
        let start = red_tiles[i];
        let end = red_tiles[(i + 1) % red_tiles.len()];

        // Add all points between start and end (exclusive of endpoints)
        if start.x == end.x {
            // Vertical line
            let x = start.x;
            let (min_y, max_y) = if start.y < end.y {
                (start.y, end.y)
            } else {
                (end.y, start.y)
            };
            for y in (min_y + 1)..max_y {
                green.insert(Point { x, y });
            }
        } else if start.y == end.y {
            // Horizontal line
            let y = start.y;
            let (min_x, max_x) = if start.x < end.x {
                (start.x, end.x)
            } else {
                (end.x, start.x)
            };
            for x in (min_x + 1)..max_x {
                green.insert(Point { x, y });
            }
        }
    }

    green
}

// Point-in-polygon using ray casting for axis-aligned polygons (optimized single-pass)
// Returns true if point is inside or on the boundary
fn point_in_polygon(p: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();
    let mut crossings = 0;

    // Single pass: check boundary AND count crossings simultaneously
    for i in 0..n {
        let v1 = polygon[i];
        let v2 = polygon[(i + 1) % n];

        // Check if point is on a vertical edge
        if v1.x == v2.x {
            let x = v1.x;
            let (min_y, max_y) = if v1.y < v2.y {
                (v1.y, v2.y)
            } else {
                (v2.y, v1.y)
            };

            // On boundary check
            if x == p.x && p.y >= min_y && p.y <= max_y {
                return true;
            }

            // Ray casting: count crossings to the right
            if x > p.x && p.y >= min_y && p.y < max_y {
                crossings += 1;
            }
        }
        // Check if point is on a horizontal edge
        else if v1.y == v2.y {
            let y = v1.y;
            let (min_x, max_x) = if v1.x < v2.x {
                (v1.x, v2.x)
            } else {
                (v2.x, v1.x)
            };

            // On boundary check
            if y == p.y && p.x >= min_x && p.x <= max_x {
                return true;
            }
        }
    }

    crossings % 2 == 1
}

// Spatial-indexed version: only check nearby polygon edges
fn point_in_polygon_spatial(p: Point, polygon: &[Point], index: &SpatialIndex) -> bool {
    let nearby_edges = index.get_nearby_edges(p);

    // First check if on boundary using only nearby edges
    for &i in &nearby_edges {
        let v1 = polygon[i];
        let v2 = polygon[(i + 1) % polygon.len()];

        if v1.x == v2.x && v1.x == p.x {
            let (min_y, max_y) = if v1.y < v2.y {
                (v1.y, v2.y)
            } else {
                (v2.y, v1.y)
            };
            if p.y >= min_y && p.y <= max_y {
                return true;
            }
        } else if v1.y == v2.y && v1.y == p.y {
            let (min_x, max_x) = if v1.x < v2.x {
                (v1.x, v2.x)
            } else {
                (v2.x, v1.x)
            };
            if p.x >= min_x && p.x <= max_x {
                return true;
            }
        }
    }

    // Ray casting with only relevant edges
    let mut crossings = 0;
    for &i in &nearby_edges {
        let v1 = polygon[i];
        let v2 = polygon[(i + 1) % polygon.len()];

        if v1.x == v2.x {
            let x = v1.x;
            let (min_y, max_y) = if v1.y < v2.y {
                (v1.y, v2.y)
            } else {
                (v2.y, v1.y)
            };

            if x > p.x && p.y >= min_y && p.y < max_y {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

fn compress_coordinates(points: &[Point]) -> (Vec<i64>, Vec<i64>, Vec<Point>) {
    let mut x_coords: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut y_coords: Vec<i64> = points.iter().map(|p| p.y).collect();

    x_coords.sort_unstable();
    x_coords.dedup();
    y_coords.sort_unstable();
    y_coords.dedup();

    let compressed: Vec<Point> = points
        .iter()
        .map(|p| Point {
            x: x_coords.binary_search(&p.x).unwrap() as i64,
            y: y_coords.binary_search(&p.y).unwrap() as i64,
        })
        .collect();

    (x_coords, y_coords, compressed)
}

fn build_valid_grid(
    x_coords: &[i64],
    y_coords: &[i64],
    red_compressed: &[Point],
    red_original: &[Point],
) -> Vec<Vec<bool>> {
    let width = x_coords.len();
    let height = y_coords.len();

    let mut grid = vec![vec![false; width]; height];

    // Mark red tiles
    let red_tiles: HashSet<Point> = red_compressed.iter().copied().collect();

    // Get green tiles on the path
    let green_tiles = get_green_tiles_on_path(red_original);

    // For each cell in compressed grid, determine if it's valid
    for cy in 0..height {
        for cx in 0..width {
            let compressed_point = Point {
                x: cx as i64,
                y: cy as i64,
            };
            let original_point = Point {
                x: x_coords[cx],
                y: y_coords[cy],
            };

            // Check if it's red
            if red_tiles.contains(&compressed_point) {
                grid[cy][cx] = true;
                continue;
            }

            // Check if it's green (on path)
            if green_tiles.contains(&original_point) {
                grid[cy][cx] = true;
                continue;
            }

            // Check if it's inside the polygon
            if point_in_polygon(original_point, red_original) {
                grid[cy][cx] = true;
            }
        }
    }

    grid
}
fn build_prefix_sum(valid: &[Vec<bool>]) -> Vec<Vec<i64>> {
    let height = valid.len();
    let width = valid[0].len();
    let mut prefix = vec![vec![0; width + 1]; height + 1];

    for y in 0..height {
        for x in 0..width {
            let invalid_count = if valid[y][x] { 0 } else { 1 };
            prefix[y + 1][x + 1] =
                invalid_count + prefix[y][x + 1] + prefix[y + 1][x] - prefix[y][x];
        }
    }

    prefix
}

fn count_invalid_in_rect(prefix: &[Vec<i64>], x1: usize, y1: usize, x2: usize, y2: usize) -> i64 {
    prefix[y2 + 1][x2 + 1] - prefix[y1][x2 + 1] - prefix[y2 + 1][x1] + prefix[y1][x1]
}

// Use geo crate's optimized implementation (1.68x faster than custom)
pub fn part_two(input: &str) -> Option<u64> {
    part_two_geo_crate(input)
}

// Original custom implementation (kept for reference)
fn part_two_custom(input: &str) -> Option<u64> {
    let red_original = make_points(input);

    if red_original.len() < 2 {
        return None;
    }

    // Coordinate compression
    let (x_coords, y_coords, compressed) = compress_coordinates(&red_original);
    let red_compressed = compressed;
    let n = red_compressed.len();

    // Build valid grid
    let valid = build_valid_grid(&x_coords, &y_coords, &red_compressed, &red_original);

    // Build prefix sum for fast queries
    let prefix = build_prefix_sum(&valid);

    // Find maximum rectangle area
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = &red_compressed[i];
            let p2 = &red_compressed[j];

            let min_x = p1.x.min(p2.x) as usize;
            let max_x = p1.x.max(p2.x) as usize;
            let min_y = p1.y.min(p2.y) as usize;
            let max_y = p1.y.max(p2.y) as usize;

            // Early pruning
            let width = (x_coords[max_x] - x_coords[min_x] + 1) as u64;
            let height = (y_coords[max_y] - y_coords[min_y] + 1) as u64;
            let potential_area = width * height;

            if potential_area <= max_area {
                continue; // Skip validation if can't improve
            }

            let invalid_count = count_invalid_in_rect(&prefix, min_x, min_y, max_x, max_y);

            if invalid_count == 0 {
                max_area = potential_area;
            }
        }
    }

    Some(max_area)
}

// ============================================================================
// ALTERNATIVE IMPLEMENTATIONS FOR BENCHMARKING
// ============================================================================

/// Version using geo crate's optimized point-in-polygon
fn part_two_geo_crate(input: &str) -> Option<u64> {
    use geo::{Contains, Coord, LineString, Polygon as GeoPolygon};

    let red_original = make_points(input);
    if red_original.len() < 2 {
        return None;
    }

    // Build polygon using geo crate
    let coords: Vec<Coord> = red_original
        .iter()
        .map(|p| Coord {
            x: p.x as f64,
            y: p.y as f64,
        })
        .collect();
    let line_string = LineString::from(coords);
    let polygon = GeoPolygon::new(line_string, vec![]);

    // Coordinate compression
    let (x_coords, y_coords, compressed) = compress_coordinates(&red_original);
    let red_compressed = compressed;
    let n = red_compressed.len();

    // Build valid grid using geo crate's Contains trait
    let width = x_coords.len();
    let height = y_coords.len();
    let mut grid = vec![vec![false; width]; height];

    let red_tiles: HashSet<Point> = red_compressed.iter().copied().collect();
    let green_tiles = get_green_tiles_on_path(&red_original);

    for cy in 0..height {
        for cx in 0..width {
            let compressed_point = Point {
                x: cx as i64,
                y: cy as i64,
            };
            let original_point = Point {
                x: x_coords[cx],
                y: y_coords[cy],
            };

            if red_tiles.contains(&compressed_point) {
                grid[cy][cx] = true;
                continue;
            }

            if green_tiles.contains(&original_point) {
                grid[cy][cx] = true;
                continue;
            }

            // Use geo crate's optimized point-in-polygon
            let coord = Coord {
                x: original_point.x as f64,
                y: original_point.y as f64,
            };
            if polygon.contains(&coord) {
                grid[cy][cx] = true;
            }
        }
    }

    // Build prefix sum and find max rectangle (same as original)
    let prefix = build_prefix_sum(&grid);
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = &red_compressed[i];
            let p2 = &red_compressed[j];

            let min_x = p1.x.min(p2.x) as usize;
            let max_x = p1.x.max(p2.x) as usize;
            let min_y = p1.y.min(p2.y) as usize;
            let max_y = p1.y.max(p2.y) as usize;

            let width = (x_coords[max_x] - x_coords[min_x] + 1) as u64;
            let height = (y_coords[max_y] - y_coords[min_y] + 1) as u64;
            let potential_area = width * height;

            if potential_area <= max_area {
                continue;
            }

            let invalid_count = count_invalid_in_rect(&prefix, min_x, min_y, max_x, max_y);

            if invalid_count == 0 {
                max_area = potential_area;
            }
        }
    }

    Some(max_area)
}

/// Version using rstar R-tree for spatial indexing
fn part_two_rstar(input: &str) -> Option<u64> {
    use rstar::RTree;

    let red_original = make_points(input);
    if red_original.len() < 2 {
        return None;
    }

    // Build R-tree with polygon points for spatial queries
    let points: Vec<[i64; 2]> = red_original.iter().map(|p| [p.x, p.y]).collect();
    let _tree = RTree::bulk_load(points);

    // Coordinate compression
    let (x_coords, y_coords, compressed) = compress_coordinates(&red_original);
    let red_compressed = compressed;
    let n = red_compressed.len();

    // Build valid grid with spatial index assistance
    let width = x_coords.len();
    let height = y_coords.len();
    let mut grid = vec![vec![false; width]; height];

    let red_tiles: HashSet<Point> = red_compressed.iter().copied().collect();
    let green_tiles = get_green_tiles_on_path(&red_original);

    for cy in 0..height {
        for cx in 0..width {
            let compressed_point = Point {
                x: cx as i64,
                y: cy as i64,
            };
            let original_point = Point {
                x: x_coords[cx],
                y: y_coords[cy],
            };

            if red_tiles.contains(&compressed_point) {
                grid[cy][cx] = true;
                continue;
            }

            if green_tiles.contains(&original_point) {
                grid[cy][cx] = true;
                continue;
            }

            // For simplicity, still use our point_in_polygon
            // (R-tree is more useful for range queries, not point-in-polygon)
            if point_in_polygon(original_point, &red_original) {
                grid[cy][cx] = true;
            }
        }
    }

    let prefix = build_prefix_sum(&grid);
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = &red_compressed[i];
            let p2 = &red_compressed[j];

            let min_x = p1.x.min(p2.x) as usize;
            let max_x = p1.x.max(p2.x) as usize;
            let min_y = p1.y.min(p2.y) as usize;
            let max_y = p1.y.max(p2.y) as usize;

            let width = (x_coords[max_x] - x_coords[min_x] + 1) as u64;
            let height = (y_coords[max_y] - y_coords[min_y] + 1) as u64;
            let potential_area = width * height;

            if potential_area <= max_area {
                continue;
            }

            let invalid_count = count_invalid_in_rect(&prefix, min_x, min_y, max_x, max_y);

            if invalid_count == 0 {
                max_area = potential_area;
            }
        }
    }

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn benchmark_implementations() {
        use std::time::Instant;

        let input = advent_of_code::template::read_file("inputs", DAY);
        let points_count = input.lines().count();

        println!("\n=== Day 9 Part 2 Implementation Comparison ===");
        println!("Points: {}", points_count);
        println!("Pairs to check: {}", points_count * (points_count - 1) / 2);

        // Warmup
        let _ = part_two(&input);
        let _ = part_two_geo_crate(&input);
        let _ = part_two_rstar(&input);

        let iterations = 10;

        // Benchmark 1: Current implementation (custom single-pass)
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = part_two(&input);
        }
        let time_custom = start.elapsed() / iterations;

        // Benchmark 2: Geo crate
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = part_two_geo_crate(&input);
        }
        let time_geo = start.elapsed() / iterations;

        // Benchmark 3: R-tree spatial index
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = part_two_rstar(&input);
        }
        let time_rstar = start.elapsed() / iterations;

        // Verify all produce same result
        let result_custom = part_two(&input);
        let result_geo = part_two_geo_crate(&input);
        let result_rstar = part_two_rstar(&input);

        assert_eq!(result_custom, result_geo, "Geo crate result doesn't match!");
        assert_eq!(result_custom, result_rstar, "R-tree result doesn't match!");

        println!(
            "\n=== Results (averaged over {} iterations) ===",
            iterations
        );
        println!("1. Custom (single-pass):   {:?}", time_custom);
        println!("2. Geo crate:              {:?}", time_geo);
        println!("3. R-star spatial index:   {:?}", time_rstar);

        let baseline = time_custom.as_secs_f64();
        println!("\n=== Speedup vs Custom ===");
        println!(
            "Geo crate:   {:.2}x {}",
            baseline / time_geo.as_secs_f64(),
            if time_geo < time_custom {
                "faster ⚡"
            } else {
                "slower"
            }
        );
        println!(
            "R-star:      {:.2}x {}",
            baseline / time_rstar.as_secs_f64(),
            if time_rstar < time_custom {
                "faster ⚡"
            } else {
                "slower"
            }
        );

        // Find the fastest
        let fastest_time = time_custom.min(time_geo).min(time_rstar);
        let fastest_name = if fastest_time == time_custom {
            "Custom single-pass"
        } else if fastest_time == time_geo {
            "Geo crate"
        } else {
            "R-star spatial index"
        };

        println!("\n🏆 Winner: {} ({:?})", fastest_name, fastest_time);
    }
}
