use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

// ============================================================================
// BIT-PACKED REGION (for regions <= 64 cells, e.g., 8x8, 4x16, etc.)
// ============================================================================

#[derive(Debug, Clone)]
struct PackedRegion {
    data: u64,
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl PackedRegion {
    fn new(width: usize, height: usize, counts: Vec<usize>) -> Option<Self> {
        if width * height > 64 {
            return None;
        }
        Some(Self {
            data: 0,
            width,
            height,
            counts,
        })
    }

    fn can_place(
        &self,
        shape_mask: u64,
        row: usize,
        col: usize,
        shape_width: usize,
        shape_height: usize,
    ) -> bool {
        if row + shape_height > self.height || col + shape_width > self.width {
            return false;
        }

        let shift = row * self.width + col;
        let mask = shape_mask << shift;
        (self.data & mask) == 0
    }

    fn place_shape(&mut self, shape_mask: u64, row: usize, col: usize, place: bool) {
        let shift = row * self.width + col;
        let mask = shape_mask << shift;
        if place {
            self.data |= mask;
        } else {
            self.data &= !mask;
        }
    }

    fn largest_empty_component(&self) -> usize {
        let mut visited = 0u64;
        let mut max_size = 0;

        for idx in 0..(self.width * self.height) {
            let bit = 1u64 << idx;
            if (self.data & bit) == 0 && (visited & bit) == 0 {
                let size = self.bfs_component(idx, &mut visited);
                max_size = max_size.max(size);
            }
        }
        max_size
    }

    fn bfs_component(&self, start_idx: usize, visited: &mut u64) -> usize {
        let mut queue = VecDeque::with_capacity(64);
        let row = start_idx / self.width;
        let col = start_idx % self.width;

        queue.push_back((row, col));
        *visited |= 1u64 << start_idx;
        let mut size = 1;

        while let Some((row, col)) = queue.pop_front() {
            let neighbors = [
                (row.wrapping_sub(1), col),
                (row + 1, col),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
            ];

            for (nr, nc) in neighbors {
                if nr < self.height && nc < self.width {
                    let idx = nr * self.width + nc;
                    let bit = 1u64 << idx;
                    if (self.data & bit) == 0 && (*visited & bit) == 0 {
                        *visited |= bit;
                        queue.push_back((nr, nc));
                        size += 1;
                    }
                }
            }
        }
        size
    }
}

// ============================================================================
// STANDARD REGION (for regions > 64 cells)
// ============================================================================

#[derive(Debug, Clone)]
struct Region {
    data: Vec<bool>,
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl Region {
    fn new(width: usize, height: usize, counts: Vec<usize>) -> Self {
        Self {
            data: vec![false; width * height],
            width,
            height,
            counts,
        }
    }

    fn can_place(&self, shape: &Shape, start_row: usize, start_col: usize) -> bool {
        if start_row + shape.height > self.height || start_col + shape.width > self.width {
            return false;
        }

        for r in 0..shape.height {
            let region_row = start_row + r;
            let shape_row_start = r * shape.width;
            let region_row_start = region_row * self.width + start_col;

            for c in 0..shape.width {
                if shape.data[shape_row_start + c] && self.data[region_row_start + c] {
                    return false;
                }
            }
        }
        true
    }

    fn place_shape(&mut self, shape: &Shape, start_row: usize, start_col: usize, place: bool) {
        for r in 0..shape.height {
            let region_row_start = (start_row + r) * self.width + start_col;
            let shape_row_start = r * shape.width;

            for c in 0..shape.width {
                if shape.data[shape_row_start + c] {
                    self.data[region_row_start + c] = place;
                }
            }
        }
    }

    fn largest_empty_component(&self) -> usize {
        let mut visited = vec![false; self.data.len()];
        let mut max_size = 0;

        for idx in 0..self.data.len() {
            if !self.data[idx] && !visited[idx] {
                let size = self.bfs_component(idx, &mut visited);
                max_size = max_size.max(size);
            }
        }
        max_size
    }

    fn bfs_component(&self, start_idx: usize, visited: &mut [bool]) -> usize {
        let mut queue = VecDeque::with_capacity(64);
        let row = start_idx / self.width;
        let col = start_idx % self.width;

        queue.push_back((row, col));
        visited[start_idx] = true;
        let mut size = 1;

        while let Some((row, col)) = queue.pop_front() {
            let neighbors = [
                (row.wrapping_sub(1), col),
                (row + 1, col),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
            ];

            for (nr, nc) in neighbors {
                if nr < self.height && nc < self.width {
                    let idx = nr * self.width + nc;
                    if !self.data[idx] && !visited[idx] {
                        visited[idx] = true;
                        queue.push_back((nr, nc));
                        size += 1;
                    }
                }
            }
        }
        size
    }
}

// ============================================================================
// SHAPES
// ============================================================================

#[derive(Debug, Clone)]
struct ShapeSet {
    orientations: Vec<Shape>,
    packed_orientations: Vec<PackedShape>,
    area: usize,
}

impl ShapeSet {
    fn new(shape: Shape) -> Self {
        let area = shape.area();
        let orientations = shape.all_orientations();
        let packed_orientations = orientations
            .iter()
            .map(|s| PackedShape::from_shape(s))
            .collect();
        Self {
            orientations,
            packed_orientations,
            area,
        }
    }
}

#[derive(Debug, Clone)]
struct PackedShape {
    mask: u64,
    width: usize,
    height: usize,
}

impl PackedShape {
    fn from_shape(shape: &Shape) -> Self {
        let mut mask = 0u64;
        for r in 0..shape.height {
            for c in 0..shape.width {
                if shape.data[r * shape.width + c] {
                    let idx = r * shape.width + c;
                    mask |= 1u64 << idx;
                }
            }
        }
        Self {
            mask,
            width: shape.width,
            height: shape.height,
        }
    }
}

#[derive(Debug, Clone)]
struct Shape {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Shape {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines.first().map(|l| l.len()).unwrap_or(0);
        let height = lines.len();
        let data = lines
            .iter()
            .flat_map(|l| l.chars())
            .map(|c| c == '#')
            .collect();
        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> bool {
        self.data[row * self.width + col]
    }

    fn area(&self) -> usize {
        self.data.iter().filter(|&&b| b).count()
    }

    fn new_empty_rotated(&self) -> Self {
        Self {
            data: vec![false; self.width * self.height],
            width: self.height,
            height: self.width,
        }
    }

    fn flip_horizontal(&self) -> Self {
        let mut new_shape = Self {
            data: vec![false; self.data.len()],
            width: self.width,
            height: self.height,
        };
        for r in 0..self.height {
            for c in 0..self.width {
                new_shape.data[r * self.width + c] = self.get(r, self.width - 1 - c);
            }
        }
        new_shape
    }

    fn rotate_90_clockwise(&self) -> Self {
        let mut new_shape = self.new_empty_rotated();
        for r in 0..self.height {
            for c in 0..self.width {
                let new_r = c;
                let new_c = self.height - 1 - r;
                new_shape.data[new_r * new_shape.width + new_c] = self.get(r, c);
            }
        }
        new_shape
    }

    fn rotate_180(&self) -> Self {
        self.rotate_90_clockwise().rotate_90_clockwise()
    }

    fn rotate_270(&self) -> Self {
        self.rotate_90_clockwise()
            .rotate_90_clockwise()
            .rotate_90_clockwise()
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let orientations = vec![
            self.clone(),
            self.rotate_90_clockwise(),
            self.rotate_180(),
            self.rotate_270(),
            self.flip_horizontal(),
            self.flip_horizontal().rotate_90_clockwise(),
            self.flip_horizontal().rotate_180(),
            self.flip_horizontal().rotate_270(),
        ];

        let mut unique = Vec::new();
        let mut seen = HashSet::new();
        for shape in orientations {
            let canonical = shape.to_canonical_string();
            if seen.insert(canonical) {
                unique.push(shape);
            }
        }
        unique
    }

    fn to_canonical_string(&self) -> String {
        self.data
            .iter()
            .map(|&b| if b { '1' } else { '0' })
            .collect()
    }
}

// ============================================================================
// SOLVER
// ============================================================================

struct Solver {
    shape_sets: Vec<ShapeSet>,
}

impl Solver {
    fn new(shape_sets: Vec<ShapeSet>) -> Self {
        Self { shape_sets }
    }

    fn area_check(&self, counts: &[usize]) -> usize {
        self.shape_sets
            .iter()
            .zip(counts)
            .map(|(ss, &count)| ss.area * count)
            .sum()
    }

    // Dispatch to packed or unpacked solver
    fn solve_region(&self, width: usize, height: usize, counts: Vec<usize>) -> bool {
        let required_area = self.area_check(&counts);
        if required_area > width * height {
            return false;
        }

        if width * height <= 64 {
            // Use bit-packed version for small regions
            let mut region = PackedRegion::new(width, height, counts).unwrap();
            self.solve_packed(&mut region)
        } else {
            // Use standard version for large regions
            let mut region = Region::new(width, height, counts);
            self.solve_standard(&mut region)
        }
    }

    fn solve_packed(&self, region: &mut PackedRegion) -> bool {
        let mut placement_order: Vec<(usize, usize)> = Vec::new();
        for (idx, &count) in region.counts.iter().enumerate() {
            for instance in 0..count {
                placement_order.push((idx, instance));
            }
        }

        placement_order.sort_by_key(|(shape_idx, instance)| {
            (
                std::cmp::Reverse(self.shape_sets[*shape_idx].area),
                *instance,
            )
        });

        self.backtrack_packed(region, &placement_order, 0, &mut Vec::new(), 0)
    }

    fn backtrack_packed(
        &self,
        region: &mut PackedRegion,
        order: &[(usize, usize)],
        depth: usize,
        positions: &mut Vec<(usize, usize, usize)>,
        check_frequency: usize,
    ) -> bool {
        if depth == order.len() {
            return true;
        }

        let (shape_idx, instance) = order[depth];
        let shape_set = &self.shape_sets[shape_idx];

        if check_frequency % 3 == 0 {
            let remaining_shapes: Vec<usize> = order[depth..]
                .iter()
                .map(|(idx, _)| self.shape_sets[*idx].area)
                .collect();

            if let Some(&max_remaining) = remaining_shapes.iter().max() {
                if region.largest_empty_component() < max_remaining {
                    return false;
                }
            }
        }

        let min_pos = if instance > 0 {
            positions
                .iter()
                .rev()
                .find(|(idx, inst, _)| *idx == shape_idx && *inst == instance - 1)
                .map(|(_, _, pos)| *pos)
                .unwrap_or(0)
        } else {
            0
        };

        let region_size = region.width * region.height;

        for packed_shape in &shape_set.packed_orientations {
            for pos in min_pos..region_size {
                let row = pos / region.width;
                let col = pos % region.width;

                if row + packed_shape.height > region.height
                    || col + packed_shape.width > region.width
                {
                    if col + packed_shape.width > region.width {
                        let skip_to = (row + 1) * region.width;
                        if skip_to >= region_size {
                            break;
                        }
                        continue;
                    }
                    continue;
                }

                if region.can_place(
                    packed_shape.mask,
                    row,
                    col,
                    packed_shape.width,
                    packed_shape.height,
                ) {
                    region.place_shape(packed_shape.mask, row, col, true);
                    positions.push((shape_idx, instance, pos));

                    if self.backtrack_packed(
                        region,
                        order,
                        depth + 1,
                        positions,
                        check_frequency + 1,
                    ) {
                        return true;
                    }

                    positions.pop();
                    region.place_shape(packed_shape.mask, row, col, false);
                }
            }
        }

        false
    }

    fn solve_standard(&self, region: &mut Region) -> bool {
        let mut placement_order: Vec<(usize, usize)> = Vec::new();
        for (idx, &count) in region.counts.iter().enumerate() {
            for instance in 0..count {
                placement_order.push((idx, instance));
            }
        }

        placement_order.sort_by_key(|(shape_idx, instance)| {
            (
                std::cmp::Reverse(self.shape_sets[*shape_idx].area),
                *instance,
            )
        });

        self.backtrack_standard(region, &placement_order, 0, &mut Vec::new(), 0)
    }

    fn backtrack_standard(
        &self,
        region: &mut Region,
        order: &[(usize, usize)],
        depth: usize,
        positions: &mut Vec<(usize, usize, usize)>,
        check_frequency: usize,
    ) -> bool {
        if depth == order.len() {
            return true;
        }

        let (shape_idx, instance) = order[depth];
        let shape_set = &self.shape_sets[shape_idx];

        if check_frequency % 3 == 0 {
            let remaining_shapes: Vec<usize> = order[depth..]
                .iter()
                .map(|(idx, _)| self.shape_sets[*idx].area)
                .collect();

            if let Some(&max_remaining) = remaining_shapes.iter().max() {
                if region.largest_empty_component() < max_remaining {
                    return false;
                }
            }
        }

        let min_pos = if instance > 0 {
            positions
                .iter()
                .rev()
                .find(|(idx, inst, _)| *idx == shape_idx && *inst == instance - 1)
                .map(|(_, _, pos)| *pos)
                .unwrap_or(0)
        } else {
            0
        };

        for orientation in &shape_set.orientations {
            let start_search = min_pos.max(
                region.data[min_pos..]
                    .iter()
                    .position(|&x| !x)
                    .map(|p| p + min_pos)
                    .unwrap_or(min_pos),
            );

            for pos in start_search..region.data.len() {
                let row = pos / region.width;
                let col = pos % region.width;

                if row + orientation.height > region.height
                    || col + orientation.width > region.width
                {
                    if col + orientation.width > region.width {
                        let skip_to = (row + 1) * region.width;
                        if skip_to >= region.data.len() {
                            break;
                        }
                        continue;
                    }
                    continue;
                }

                if region.can_place(orientation, row, col) {
                    region.place_shape(orientation, row, col, true);
                    positions.push((shape_idx, instance, pos));

                    if self.backtrack_standard(
                        region,
                        order,
                        depth + 1,
                        positions,
                        check_frequency + 1,
                    ) {
                        return true;
                    }

                    positions.pop();
                    region.place_shape(orientation, row, col, false);
                }
            }
        }

        false
    }
}

// ============================================================================
// MAIN
// ============================================================================

pub fn part_one(input: &str) -> Option<u64> {
    let shapes: Vec<Shape> = input
        .split("\n\n")
        .filter_map(|chunk| {
            let lines: Vec<&str> = chunk
                .lines()
                .filter(|line| !line.contains(':') && !line.contains('x'))
                .collect();

            if lines.is_empty() {
                None
            } else {
                Some(Shape::new(&lines.join("\n")))
            }
        })
        .collect();

    let shape_sets: Vec<ShapeSet> = shapes.iter().map(|s| ShapeSet::new(s.clone())).collect();

    let regions: Vec<(usize, usize, Vec<usize>)> = input
        .lines()
        .rev()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                return None;
            }

            let dims: Vec<&str> = parts[0].split('x').collect();
            if dims.len() != 2 {
                return None;
            }

            let width: usize = dims[0].parse().ok()?;
            let height: usize = dims[1].parse().ok()?;

            let counts: Vec<usize> = parts[1]
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            Some((width, height, counts))
        })
        .collect();

    let solver = Solver::new(shape_sets);

    // PARALLEL PROCESSING OF REGIONS
    let solvable_count: u64 = regions
        .into_par_iter()
        .enumerate()
        .map(|(_, (width, height, counts))| {
            // let size = width * height;
            // let pack_type = if size <= 64 { "packed" } else { "standard" };
            // println!(
            //     "Solving region {} ({}x{}, {} cells, {})...",
            //     idx + 1,
            //     width,
            //     height,
            //     size,
            //     pack_type
            // );
            let result = solver.solve_region(width, height, counts);
            // println!(
            //     "  {} region {}",
            //     if result { "✓ Solved" } else { "✗ Failed" },
            //     idx + 1
            // );
            if result { 1 } else { 0 }
        })
        .sum();

    Some(solvable_count)
}

pub fn part_two(_: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
