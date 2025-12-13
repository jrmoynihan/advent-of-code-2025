use std::collections::HashSet;

advent_of_code::solution!(9);

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
    // For each extreme point, find the other extreme points that are furthest away from it in any dimension
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

// Point-in-polygon using ray casting for axis-aligned polygons
// Returns true if point is inside or on the boundary
fn point_in_polygon(p: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();

    if polygon.contains(&p) {
        return true;
    }

    for i in 0..n {
        let v1 = polygon[i];
        let v2 = polygon[(i + 1) % n];

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

    let mut crossings = 0;

    for i in 0..n {
        let v1 = polygon[i];
        let v2 = polygon[(i + 1) % n];

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

pub fn part_two(input: &str) -> Option<u64> {
    let red_original = make_points(input);

    if red_original.len() < 2 {
        return None;
    }

    // Coordinate compression
    let (x_coords, y_coords, compressed) = compress_coordinates(&red_original);
    let mut red_compressed = compressed;
    red_compressed.sort_by_key(|p| (p.x, p.y)); // Better memory access patterns

    // Build valid grid
    let valid = build_valid_grid(&x_coords, &y_coords, &red_compressed, &red_original);

    // Build prefix sum for fast queries
    let prefix = build_prefix_sum(&valid);

    // Find maximum rectangle area
    let mut max_area = 0;

    for (i, p1) in red_compressed.iter().enumerate() {
        for p2 in red_compressed.iter().skip(i + 1) {
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

    Some(max_area as u64)
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
}
