# Day 4: Approach Comparison

## Current Approach (Work Queue)

### Data Structure
- `Grid<u8>` abstraction
- Returns `Option<&u8>` for every access
- Requires bounds checking on every neighbor access

### Algorithm (Part 2)
- **Work Queue**: `HashSet` tracks cells that need re-checking
- Start with all '@' cells
- After removing cells, only add their neighbors to the queue
- Iterate until queue is empty

### Pros
- ✅ Only checks cells that might have changed
- ✅ Theoretically minimal work (only changed cells)
- ✅ Good for sparse changes

### Cons
- ❌ **Cache-unfriendly**: HashSet iteration order is non-sequential
- ❌ **Option unwrapping overhead**: Every `grid.get()` returns `Option`
- ❌ **Bounds checking**: Every neighbor access checks bounds
- ❌ **HashSet overhead**: Hash computation and collision handling
- ❌ **Non-sequential memory access**: Random access pattern hurts CPU cache

---

## Alternative Approach (Row-by-Row Backtracking)

### Data Structure
- `Vec<Vec<bool>>` with border padding
- Direct array indexing: `grid[row][col]`
- No bounds checking needed (border handles it)

### Algorithm (Part 2)
- **Sequential sweep**: Process rows top-to-bottom
- **Backtracking**: If any cell removed in a row, go back one row
- Continue until bottom row processed with no changes

### Pros
- ✅ **Cache-friendly**: Sequential row-by-row access
- ✅ **No bounds checking**: Border padding eliminates all checks
- ✅ **No Option overhead**: Direct bool access
- ✅ **Simple neighbor counting**: Can use slice operations
- ✅ **Predictable access pattern**: CPU prefetcher can help

### Cons
- ❌ Might check more cells than necessary (but sequential access compensates)
- ❌ Backtracking might revisit rows multiple times

---

## Performance Analysis

### Memory Access Patterns

**Current (Work Queue)**:
```
Access pattern: Random (HashSet order)
Cache misses: High (non-sequential)
CPU prefetching: Poor
```

**Alternative (Row-by-Row)**:
```
Access pattern: Sequential (row-by-row)
Cache misses: Low (sequential)
CPU prefetching: Excellent
```

### Overhead Comparison

**Current Approach**:
- Option unwrapping: ~1-2 cycles per access
- Bounds checking: ~1-2 cycles per neighbor
- HashSet operations: ~5-10 cycles per insert/lookup
- Random memory access: Cache miss penalty (~100-300 cycles)

**Alternative Approach**:
- Direct array access: ~1 cycle
- No bounds checking: 0 cycles
- No Option overhead: 0 cycles
- Sequential access: Cache hit (~1-3 cycles)

### Expected Performance

For **Part 1** (single pass):
- Current: ~260µs
- Alternative: **~150-200µs** (30-40% faster)
  - Eliminates Option/bounds overhead
  - Better cache locality

For **Part 2** (iterative removal):
- Current: ~5.1ms
- Alternative: **~2-3ms** (40-60% faster)
  - Sequential access pattern is huge win
  - Even though it might check more cells, sequential access is much faster

---

## Key Insight

**Cache locality trumps algorithmic efficiency** for this problem size.

The work queue approach is theoretically more efficient (fewer checks), but:
1. The overhead of random memory access kills performance
2. CPU cache misses are expensive (100-300 cycles)
3. Sequential access allows CPU prefetching to work

The alternative approach trades some redundant checks for:
- Sequential memory access (cache-friendly)
- No abstraction overhead
- Better CPU pipeline utilization

---

## Recommendation

**Use the alternative approach** (Vec<Vec<bool>> with row-by-row backtracking) because:

1. **Cache performance dominates**: Sequential access is 10-100x faster than random access
2. **Simpler code**: Less abstraction, easier to understand
3. **Better for this problem size**: Grid is small enough that sequential scanning is fast
4. **CPU-friendly**: Modern CPUs are optimized for sequential access patterns

The work queue approach would be better for:
- Very large grids where most cells don't change
- Sparse updates where only a tiny fraction changes per iteration
- Cases where cache locality is less important

But for AoC-sized problems, **sequential access wins**.

