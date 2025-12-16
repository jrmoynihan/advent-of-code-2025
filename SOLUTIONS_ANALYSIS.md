# Advent of Code 2025 - Solutions Analysis & Learning Guide

This document provides a comprehensive analysis of all solutions from Advent of Code 2025, focusing on algorithmic techniques, optimization strategies, and key learnings for future problem-solving.

---

## Table of Contents

1. [Day-by-Day Analysis](#day-by-day-analysis)
2. [Algorithmic Techniques Summary](#algorithmic-techniques-summary)
3. [Optimization Patterns](#optimization-patterns)
4. [Key Takeaways](#key-takeaways)

---

## Day-by-Day Analysis

### Day 1: Secret Entrance - Circular Array Simulation

**Problem Summary:**
- Simulate a dial that can rotate left (L) or right (R) with wrapping at boundaries (0-99)
- Part 1: Count how many times the dial lands on 0 after complete rotations
- Part 2: Count how many times the dial passes through 0 during rotations (including mid-rotation)

**Key Aspects:**
- Circular array/modular arithmetic problem
- Requires tracking state transitions
- Part 2 adds complexity by counting intermediate states

**Solution Chosen:**
- Direct simulation using modular arithmetic (`rem_euclid` for proper wrapping)
- Track revolutions by calculating how many times the value crosses 0

**Why This Solution:**
- Simple and direct - no complex data structures needed
- `rem_euclid` handles negative wrapping correctly (unlike `%`)
- O(n) time complexity where n is number of instructions
- Revolution counting requires careful boundary condition handling

**Optimizations:**
- Inline functions for hot path operations
- Direct calculation of revolutions rather than iteration
- Careful handling of edge case: `dial != DIAL_MIN && new <= DIAL_MIN` prevents double-counting

**Key Learning:**
- Use `rem_euclid` for true modular arithmetic (not `%` which can return negative)
- Revolution counting formula: `(new / DIAL_MAX).abs()` with boundary adjustments

---

### Day 2: Gift Shop - Pattern Matching in Number Ranges

**Problem Summary:**
- Find "invalid" product IDs within ranges where numbers match specific digit patterns
- Part 1: Numbers with digits repeated exactly twice (e.g., 1234**1234**)
- Part 2: Numbers with digits repeated at least twice (any repeating pattern)

**Key Aspects:**
- Iterate through potentially large numeric ranges
- Pattern detection in digit sequences
- Avoid string conversions for performance

**Solution Chosen:**
- Extract digits into fixed-size array using division/modulo
- Part 1: Check if first half equals second half
- Part 2: Check all divisors of length for repeating chunks

**Why This Solution:**
- Avoids expensive string allocations
- Fixed-size array (20 digits max for u64) is stack-allocated
- Direct digit manipulation is faster than string operations
- Reverse order storage simplifies palindrome checks

**Optimizations:**
- Stack-allocated fixed-size arrays instead of Vec
- Early continue for numbers with odd digit count (Part 1)
- Iterate divisors in reverse order (Part 2) to find largest pattern first
- Direct digit extraction: `digits[len] = (temp % 10) as u8`

**Key Learning:**
- For numeric pattern matching, digit arrays beat strings
- When checking repeating patterns, iterate chunk sizes from largest to smallest
- Stack allocation matters for tight loops

---

### Day 3: Lobby - Greedy Digit Selection

**Problem Summary:**
- Extract maximum value from lines of digits
- Part 1: Choose any 2 digits to form the largest 2-digit number
- Part 2: Choose exactly 12 digits to form the largest 12-digit number

**Key Aspects:**
- Greedy selection problem
- Must preserve order (can't rearrange)
- Part 2 requires careful window calculation to ensure enough digits remain

**Solution Chosen:**
- **Part 1**: Suffix maximum array for O(n) solution
  - Precompute maximum digit from each position forward
  - For each digit, pair with best future digit
- **Part 2**: Greedy window selection with look-ahead
  - Calculate dynamic search window ensuring enough digits remain
  - Select maximum digit in valid window, advance past it

**Why This Solution:**
- Part 1: Suffix maximum eliminates O(n²) nested loop
- Part 2: Greedy approach with window constraints guarantees optimality
- Both avoid backtracking

**Optimizations:**
- Work directly with bytes (`line.as_bytes()`) instead of chars
- Suffix maximum array enables O(n) lookup
- Early termination when 99 is found (Part 1)
- Parallel processing with Rayon (Part 2)
- Window calculation: `remaining_len - (cells_needed - 1)`

**Key Learning:**
- Precomputed suffix/prefix arrays transform O(n²) to O(n)
- Greedy selection with look-ahead: calculate minimum elements to reserve
- Rayon's `par_lines()` provides easy parallelization for independent lines

---

### Day 4: Printing Department - Grid Simulation with Cascading Updates

**Problem Summary:**
- Grid of paper rolls (`@`), count/remove accessible ones
- Part 1: Count rolls with < 4 neighbors in 8 directions
- Part 2: Repeatedly remove accessible rolls until none remain (cascading effect)

**Key Aspects:**
- 2D grid traversal with neighbor counting
- Part 2 requires iterative updates as removals expose new removables
- Order of removal matters for Part 2

**Solution Chosen:**
- Grid with border padding to avoid bounds checking
- Part 1: Single pass with neighbor counting
- Part 2: Row-by-row backtracking algorithm
  - Process row left-to-right, remove accessible rolls
  - If any removed, backtrack one row
  - Continue until no changes

**Why This Solution:**
- Border padding eliminates expensive boundary checks
- Bool grid (not u8) reduces memory and cache pressure
- Row-by-row with backtracking ensures all cascading removals found
- Simpler than HashSet work queue approach

**Optimizations:**
- Padded grid: add border of false values
- Slice-based neighbor counting: `grid[row-1..=row+1]`
- Inline neighbor checks in 3x3 window
- Row backtracking instead of full HashSet traversal
- Bool instead of u8 saves memory

**Key Learning:**
- Border padding is a classic optimization for grid problems
- Row-by-row with backtracking can be simpler than work queue
- Cascading updates require revisiting modified regions

---

### Day 5: Cafeteria - Range Merging and Binary Search

**Problem Summary:**
- Manage ingredient ID ranges to determine freshness
- Part 1: Count how many specific IDs fall within ranges
- Part 2: Count total IDs covered by ranges

**Key Aspects:**
- Overlapping interval merging
- Point-in-range queries
- Binary search optimization

**Solution Chosen:**
- Sort ranges by start point
- Merge overlapping/adjacent ranges
- Part 1: Binary search to find containing range for each ID
- Part 2: Sum widths of merged ranges

**Why This Solution:**
- Merging reduces redundant checks
- Sorted, non-overlapping ranges enable binary search
- O(n log n) sort + O(m log n) queries vs O(n*m) brute force

**Optimizations:**
- Sort once before merging
- Merge overlapping ranges: `start <= last_end + 1`
- Binary search with careful indexing for point queries
- Direct range width calculation for Part 2

**Key Learning:**
- Always merge overlapping intervals before querying
- Binary search on intervals: `binary_search_by_key` then check neighbors
- Adjacent ranges (end+1 == next.start) should be merged

---

### Day 6: Trash Compactor - SIMD-Accelerated Parsing

**Problem Summary:**
- Parse unusual column-based math worksheets
- Part 1: Read problems top-to-bottom, left-to-right
- Part 2: Read problems bottom-to-top, right-to-left within columns

**Key Aspects:**
- Complex 2D parsing problem
- Vertical number reading
- SIMD opportunities in preprocessing

**Solution Chosen:**
- Precompute all digit values using SIMD
- Use SIMD to find newlines and structure
- Store worksheet structure (line length, operator positions)
- Parse numbers from precomputed digit array

**Why This Solution:**
- SIMD bulk conversion of ASCII to digits
- Precomputation amortizes cost across many operations
- Avoids repeated character classification

**Optimizations:**
- **SIMD operations:**
  - `simd_splat_u8(b'0')` for vectorized subtraction
  - Lane-wide newline search: `lane.simd_eq(newlines)`
  - 64-wide lanes for throughput
- Precompute digit table: `digits[i] = bytes[i] - b'0'`
- Single-pass column parsing with bounds checking
- Portable SIMD (`#![feature(portable_simd)]`)

**Key Learning:**
- SIMD shines in bulk character processing
- Precomputation trades memory for speed
- Portable SIMD provides cross-platform acceleration
- Column-oriented data benefits from transposed representation

---

### Day 7: Laboratories - Beam Splitting Simulation

**Problem Summary:**
- Simulate tachyon beams through splitters
- Part 1: Count total beam splits (classical)
- Part 2: Count unique end states (quantum - multiplicative paths)

**Key Aspects:**
- State propagation through grid
- Part 1: Count events
- Part 2: Count combinatorial outcomes

**Solution Chosen:**
- Track beam positions as boolean array (Part 1) or counts (Part 2)
- Single pass through grid, updating beam state
- Part 1: Increment split counter when beam hits splitter
- Part 2: When beam splits, duplicate its count to neighbors

**Why This Solution:**
- O(rows * cols) time complexity
- Space: O(cols) for beam tracking
- No backtracking or recursion needed
- Part 2's counting approach elegantly handles exponential path explosion

**Optimizations:**
- Boolean array for Part 1 (minimal memory)
- Integer count array for Part 2 (tracks all possibilities)
- In-place updates as we scan downward
- Direct array indexing, no HashMap needed

**Key Learning:**
- Sometimes counting outcomes is easier than enumerating them
- Part 2's solution: track number of ways to reach each position
- Multiplicative path counting avoids exponential complexity

---

### Day 8: Playground - Minimum Spanning Tree (Kruskal's Algorithm)

**Problem Summary:**
- Connect 3D junction boxes with minimum cable
- Part 1: After 1000 connections, find 3 largest circuits
- Part 2: Find last connection needed to unite all boxes

**Key Aspects:**
- Classic graph connectivity problem
- Minimum Spanning Tree (MST) with early termination
- Union-Find data structure

**Solution Chosen:**
- **Kruskal's Algorithm:**
  1. Generate all pairs and compute distances
  2. Sort edges by distance
  3. Use Union-Find to track connected components
- Part 1: Stop after 1000 edges, count component sizes
- Part 2: Stop when all connected (MST complete)

**Why This Solution:**
- Kruskal's is optimal for MST
- Union-Find provides efficient connectivity queries
- No need for Prim's or Dijkstra's here

**Optimizations:**
- Skip actual square root in Euclidean distance (compare squared distances)
- Union-Find with:
  - Path compression: flatten tree on find
  - Union by size: attach smaller tree to larger
- Distance-first sort with coordinate tie-breaker for determinism
- Early termination when edges_connected == n-1

**Key Learning:**
- MST problems: Kruskal's (sort edges) vs Prim's (grow from node)
- Union-Find is essential for dynamic connectivity
- Squared distances suffice for comparisons (avoid expensive sqrt)
- Path compression + union by size → nearly O(1) operations

---

### Day 9: Movie Theater - Computational Geometry

**Problem Summary:**
- Find largest rectangle with red tiles at opposite corners
- Part 1: Any rectangle between any two red tiles
- Part 2: Rectangle must only contain red/green tiles (polygon constraint)

**Key Aspects:**
- Computational geometry
- Part 1: Brute force all pairs
- Part 2: Point-in-polygon, coordinate compression, 2D prefix sums

**Solution Chosen:**
- **Part 1**: Try all pairs of red tiles, compute Manhattan rectangle area
  - Area = `(|x2-x1| + 1) * (|y2-y1| + 1)`
- **Part 2**: Complex multi-step approach
  1. Connect red tiles with green tiles (on path between consecutive reds)
  2. Use ray-casting for point-in-polygon
  3. Coordinate compression to reduce search space
  4. 2D prefix sum for fast rectangle validation
  5. Brute force with early pruning

**Why This Solution:**
- Part 1: O(n²) is acceptable for reasonable n
- Part 2: Coordinate compression + prefix sums enable O(n²) validation
- Point-in-polygon ensures we only check valid regions

**Optimizations:**
- **Coordinate compression**: Map sparse coordinates to dense indices
- **2D prefix sum array**: Count invalid cells in O(1)
  - `prefix[y2+1][x2+1] - prefix[y1][x2+1] - prefix[y2+1][x1] + prefix[y1][x1]`
- **Early pruning**: Skip if potential area ≤ current max
- **Ray casting**: Simplified for axis-aligned polygon
- Edge caching and sorted iteration

**Key Learning:**
- Coordinate compression transforms sparse problems to dense
- 2D prefix sums enable O(1) rectangle sum queries
- Ray casting for point-in-polygon: count boundary crossings
- Early pruning in brute force can provide huge speedups

---

### Day 10: Factory - Linear Systems Over Different Domains

**Problem Summary:**
- Configure machines via button presses
- Part 1: Toggle indicator lights (boolean, XOR) - system over GF(2)
- Part 2: Increment counters (integers, addition) - system over integers

**Key Aspects:**
- Part 1: System of linear equations over GF(2) (binary field)
- Part 2: Optimal button presses to reach target counts
- Fundamentally different mathematical structures

**Solution Chosen:**
- **Part 1**: Brute force (2^m combinations) for small m
  - Try all button press combinations (0 or 1 times each)
  - Each button toggles specific lights (XOR operation)
- **Part 2**: Recursive parity-based dynamic programming
  - Key insight: Pressing buttons twice returns to same state (even/odd parity)
  - Precompute all single-press outcomes grouped by parity
  - Recursively: `solve(target) = min(presses + 2*solve((target - joltages)/2))`
  - Memoization prevents recomputation

**Why This Solution:**
- Part 1: Small button count makes brute force viable
  - Gaussian elimination over GF(2) was implemented but brute force is simpler
- Part 2: Parity insight reduces infinite search space
  - Only need to press each button 0 or 1 times, then recursively double
  - Memoization makes this tractable

**Optimizations:**
- Bit manipulation for Part 1 state representation
- HashMap memoization for Part 2
- Parity grouping dramatically prunes search space
- Pattern precomputation amortizes cost

**Alternative Approaches Considered:**
- Part 1: Gaussian elimination (RREF) over GF(2) - implemented but unused
- Part 2: BFS, A*, direct search - all too slow without parity insight

**Key Learning:**
- GF(2) systems: every element is its own inverse (x XOR x = 0)
- Parity-based recursion: powerful technique for optimization problems
- Even/odd analysis can transform infinite problems to finite
- Memoization + mathematical insight beats blind search

---

### Day 11: Reactor - Path Counting in Directed Graphs

**Problem Summary:**
- Count paths through directed device network
- Part 1: All simple paths from "you" to "out"
- Part 2: Paths from "svr" to "out" passing through both "dac" and "fft"

**Key Aspects:**
- Graph traversal with path counting
- Part 2 adds mandatory waypoint constraints
- No cycles (DAG assumption enables memoization)

**Solution Chosen:**
- **Part 1**: DFS with backtracking
  - Track visited nodes per path
  - Clone visited set for each branch (backtracking)
  - Count paths reaching destination
- **Part 2**: Memoized DFS on DAG
  - Assume graph is DAG (no cycles)
  - Precompute path counts between waypoints
  - Multiply counts for different orderings: (svr→dac→fft→out) + (svr→fft→dac→out)

**Why This Solution:**
- Part 1: Backtracking ensures all paths found
- Part 2: DAG assumption enables memoization
  - Without cycles, each node has fixed path count to destination
  - Split problem by required visit order

**Optimizations:**
- Visited set cloning for clean backtracking
- Memoization map for DAG traversal (Part 2)
- Path multiplication instead of enumeration (Part 2)
- HashSet operations for visited tracking

**Key Learning:**
- DFS with visited cloning: clean backtracking pattern
- DAG → memoized path counting is efficient
- Waypoint constraints: split by visit order, multiply counts
- In Rust: `visited.clone()` for each branch gives automatic backtracking

---

### Day 12: Christmas Tree Farm - 2D Bin Packing with Backtracking

**Problem Summary:**
- Fit oddly-shaped presents into rectangular regions
- Presents can be rotated/flipped
- NP-complete problem

**Key Aspects:**
- 2D bin packing (NP-complete)
- Multiple shapes, multiple instances
- Rotation and reflection symmetries

**Solution Chosen:**
- Comprehensive backtracking with heavy optimizations:
  1. **Precompute shape orientations**: All 8 rotations/flips, deduplicate
  2. **Sort shapes by area**: Place larger shapes first
  3. **Canonical ordering**: Place multiple instances of same shape left-to-right
  4. **Connected component pruning**: If largest remaining shape > largest empty region, backtrack
  5. **Bit-packed representation**: For regions ≤64 cells, use u64 bitmask
  6. **Parallel processing**: Rayon for independent regions

**Why This Solution:**
- Backtracking is necessary for NP-complete problems
- Heavy pruning makes it practical
- Bit-packing provides major speedup for small regions

**Optimizations:**
- **Bit-packing**: Region ≤64 cells → u64 bitmask
  - Single bit operations instead of array operations
  - BFS on bitmask for connected components
- **Shape ordering**: Largest first reduces backtracking
- **Instance ordering**: `min_pos` constraint prevents duplicate placements
- **Connected component check**: Every 3rd placement (tuned frequency)
- **Parallel regions**: `regions.into_par_iter()`
- **Early first-fit skip**: Jump to next row when shape won't fit

**Key Learning:**
- NP-complete → need intelligent pruning
- Bit-packing can provide 10x+ speedup for small grids
- Connected component analysis is powerful pruning heuristic
- Sort by constraint difficulty (largest shapes first)
- Rayon makes parallelization trivial for independent subproblems

---

## Algorithmic Techniques Summary

### 1. Number Theory & Arithmetic
- **Modular arithmetic**: Day 1 (circular dial)
  - Use `rem_euclid` not `%` for proper wrapping
- **Digit manipulation**: Day 2 (pattern matching), Day 3 (greedy selection)
  - Extract digits via division/modulo into fixed array
  - Faster than string conversion
- **Pattern detection**: Day 2 (repeating digits)
  - Check divisors of length for chunk repetition

### 2. Array/String Processing
- **Suffix arrays**: Day 3 Part 1 (O(n²) → O(n))
  - Precompute maximum/minimum from each position
- **SIMD vectorization**: Day 6 (bulk digit conversion)
  - Portable SIMD for cross-platform acceleration
  - Best for homogeneous operations on large arrays
- **Prefix sums**: Day 9 Part 2 (2D rectangle queries)
  - Enable O(1) range sum queries after O(n²) preprocessing

### 3. Graph Algorithms
- **Union-Find (Disjoint Set Union)**: Day 8 (MST)
  - Path compression + union by size → amortized O(α(n)) ≈ O(1)
  - Essential for dynamic connectivity
- **Kruskal's Algorithm**: Day 8 (Minimum Spanning Tree)
  - Sort edges + Union-Find
  - Good when edges are easily enumerable
- **DFS with backtracking**: Day 11 (path counting)
  - Clone visited set for each branch
  - Count all simple paths
- **Memoized DFS on DAG**: Day 11 Part 2 (path counting with constraints)
  - Assumption: no cycles enables memoization
  - DP on graph structure

### 4. Greedy Algorithms
- **Greedy selection with look-ahead**: Day 3 Part 2 (12-digit extraction)
  - Calculate minimum elements to reserve
  - Select maximum in valid window
- **Interval merging**: Day 5 (range consolidation)
  - Sort + single pass merge
  - Critical for range queries

### 5. Computational Geometry
- **Point-in-polygon (ray casting)**: Day 9 Part 2
  - Count boundary crossings to determine interior
  - Simplified for axis-aligned polygons
- **Coordinate compression**: Day 9 Part 2
  - Map sparse coordinates to dense indices
  - Enables tractable brute force
- **Rectangle area calculation**: Day 9
  - Manhattan distance for axis-aligned rectangles

### 6. Simulation & State Space
- **Grid simulation with borders**: Day 4
  - Pad grid with sentinel values
  - Eliminates bounds checking
- **Cascading updates with backtracking**: Day 4 Part 2
  - Row-by-row with backtrack on changes
- **Beam/particle simulation**: Day 7
  - Track state counts instead of enumerating paths
  - Multiplicative path counting

### 7. Optimization & Search
- **Backtracking with pruning**: Day 12 (2D bin packing)
  - Connected component check
  - Largest-first ordering
  - Canonical placement constraints
- **Brute force with early termination**: Day 10 Part 1
  - Enumerate all 2^m combinations for small m
- **Dynamic programming with memoization**: Day 10 Part 2
  - Parity-based recursive DP
  - Pattern precomputation + memoization

### 8. Linear Algebra
- **Systems over GF(2)**: Day 10 Part 1
  - Gaussian elimination for XOR systems
  - Every element is its own inverse
- **Integer optimization**: Day 10 Part 2
  - Parity analysis transforms infinite to finite

---

## Optimization Patterns

### Memory & Cache Optimization
1. **Stack vs Heap**
   - Fixed-size arrays when bounds known (Day 2)
   - Bool vectors over u8 for boolean data (Day 4, Day 7)
2. **Bit-packing**
   - u64 bitmasks for ≤64 elements (Day 12)
   - Massive speedup for small state spaces
3. **Spatial locality**
   - Row-major traversal (Day 4)
   - Border padding eliminates branches (Day 4)

### Algorithmic Optimization
1. **Precomputation**
   - Suffix/prefix arrays (Day 3)
   - Digit tables (Day 6)
   - Shape orientations (Day 12)
2. **Early termination**
   - Found maximum value → break (Day 3)
   - Area pruning (Day 9)
   - Connected component check (Day 12)
3. **Avoiding recomputation**
   - Memoization (Day 10 Part 2, Day 11 Part 2)
   - Pattern grouping (Day 10 Part 2)

### Parallelization
- **Rayon parallel iterators**: Day 3 Part 2, Day 12
  - `par_lines()` for line-independent processing
  - `into_par_iter()` for independent subproblems
- **When to parallelize**: No shared mutable state, substantial work per item

### Mathematical Insights
1. **Parity analysis**: Day 10 Part 2
   - Reduce infinite search to finite
2. **Squared distances**: Day 8
   - Avoid expensive sqrt, preserves ordering
3. **Modular arithmetic**: Day 1
   - `rem_euclid` for proper wrapping

### Data Structure Selection
1. **HashMap vs Vec**
   - Vec when indices dense/predictable
   - HashMap for sparse/string keys
2. **HashSet for visited tracking**: Day 11
   - O(1) lookups, cloneable for backtracking
3. **VecDeque for BFS**: Day 9, Day 12
   - Efficient push_back/pop_front

---

## Key Takeaways

### Problem-Solving Strategies
1. **Identify the core problem type**
   - Simulation, graph, geometry, optimization, number theory
2. **Consider mathematical properties**
   - Parity, modular arithmetic, symmetry
3. **Look for standard algorithms**
   - MST, shortest path, dynamic programming
4. **Start simple, optimize later**
   - Brute force first, then identify bottlenecks

### Rust-Specific Patterns
1. **Ownership for backtracking**
   - Clone data for each branch
   - Automatic cleanup on return
2. **Type system for correctness**
   - NewTypes, enums for state machines
3. **Iterator chaining**
   - Functional style often clearer
4. **Inline hints for hot paths**
   - `#[inline]` for small, frequently-called functions

### Optimization Workflow
1. **Measure first**: Don't guess bottlenecks
2. **Algorithmic before micro**: O(n²) → O(n log n) beats cache tweaks
3. **Profile-guided**: Use real input for profiling
4. **Parallel when trivial**: Rayon makes it easy, use when applicable

### Common Pitfalls
1. **Modulo with negative numbers**: Use `rem_euclid` not `%`
2. **Off-by-one in ranges**: Inclusive vs exclusive endpoints
3. **Floating point comparisons**: Use `partial_cmp` carefully
4. **Premature optimization**: Get correctness first

### Problem Categories & Approaches
- **Simulation**: Model directly, optimize data structures
- **Graph**: Identify connectivity/path/flow problem, apply standard algorithms
- **Geometry**: Look for coordinate compression, sweep line, or brute force with pruning
- **Optimization**: Backtracking with pruning, DP, greedy if applicable
- **Number theory**: Look for mathematical properties (parity, modular, divisibility)

---

## Conclusion

This year's Advent of Code required a diverse toolkit:
- **Days 1-3**: Simulation and greedy algorithms
- **Days 4-5**: Grid processing and interval management
- **Days 6-7**: SIMD optimization and state counting
- **Days 8-9**: Graph theory and computational geometry
- **Days 10-11**: Linear systems and path counting
- **Day 12**: NP-complete problem with heavy optimization

**Most impactful techniques:**
1. Memoization / DP for avoiding recomputation
2. Bit-packing for small state spaces
3. Precomputation (suffix arrays, lookup tables)
4. Parallelization with Rayon
5. Mathematical insight (parity, modular arithmetic)

**For future years:**
- Build a library of common algorithms (Union-Find, DFS templates, etc.)
- Practice identifying problem types quickly
- Remember: correctness first, optimization second
- Keep a reference of Rust optimization patterns


