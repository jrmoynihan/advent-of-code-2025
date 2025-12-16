# Optimization Failures & Lessons Learned

Documentation of optimization attempts that didn't work, why they failed, and what we learned.

---

## Overview

Not all optimizations succeed. This document captures failed attempts to help avoid similar mistakes in the future.

**Key Principle:** Measure everything, be willing to revert, learn from failures.

---

## Day 9: Movie Theater - Multiple Failed Attempts

**Baseline:** 98.7ms (custom point-in-polygon implementation)  
**Final:** 52.0ms (geo crate)  
**Failed attempts:** 3 out of 5 optimizations made it slower

---

### ❌ Failure 1: Sorting Pairs by Potential Area

**Attempt:** Sort all 122,760 pairs by descending potential area, then check largest first with early termination.

**Expected:** Early termination would skip most pairs after finding large rectangle.

**Result:** 98.7ms → 107.9ms (**SLOWER by 9.2ms**)

**Why it failed:**
```
Sorting overhead: ~15ms (collecting + sorting 122k pairs)
Early termination benefit: ~5ms saved
Net result: -10ms (SLOWER)
```

**Root cause:**
- Sorting 122,760 pairs is expensive (O(n log n))
- Many pairs have similar potential areas (no dramatic pruning)
- Early termination doesn't save enough to justify sorting cost
- Rectangle validation is already fast (O(1) prefix sum query)

**Lesson:** Sorting has overhead - only worth it if you can skip significant work (>50% of items).

**When sorting helps:**
- ✅ Can skip >50% of items after sorting
- ✅ Sorting cost << work saved
- ✅ Items have very different priorities

**When sorting hurts:**
- ❌ Most items need to be checked anyway
- ❌ Sorting cost > work saved
- ❌ Items have similar priorities

---

### ❌ Failure 2: Rayon Parallelization

**Attempt:** Parallelize pair checking with `par_iter()`.

**Expected:** 2-3x speedup on multi-core CPU.

**Result:** 98.7ms → 109.3ms (**SLOWER by 10.6ms**)

**Why it failed:**
```
Thread spawning overhead: ~10ms
Work per pair: ~0.7µs (too small)
Rectangle validation: O(1) prefix sum query (too fast)
Net result: -11ms (SLOWER)
```

**Root cause:**
- Work per item is too small (~0.7µs)
- Thread overhead (~10ms) dominates
- Rectangle validation is already O(1) and fast
- Collecting pairs adds memory allocation overhead

**Lesson:** Parallelization needs sufficient work per item to overcome overhead.

**When parallelization helps:**
- ✅ >10µs work per item
- ✅ Independent work units
- ✅ No shared mutable state
- ✅ Substantial total work

**When parallelization hurts:**
- ❌ <10µs work per item
- ❌ Thread overhead > work saved
- ❌ Work is already fast (O(1) operations)
- ❌ Memory allocation overhead

**Example of successful parallelization:**
- Day 2: Each range has substantial work (many numbers to check)
- Work per range: ~300-1000µs
- Thread overhead: ~10ms
- Net: 3.9x speedup ✅

---

### ❌ Failure 3: HashMap Caching for point_in_polygon

**Attempt:** Cache point-in-polygon results in HashMap to avoid redundant calls.

**Expected:** Eliminate duplicate polygon checks.

**Result:** 88.4ms → 86.4ms → **reverted** (inconsistent, no clear win)

**Why it failed:**
```
Cache hit rate: only 2.6% (1,624 hits / 61,504 cells)
HashMap lookup overhead: ~0.1µs per lookup
Direct function call: ~1.3µs per call
Net: Marginal savings, inconsistent results
```

**Root cause:**
- Most cells map to unique original points (coordinate compression)
- Cache hit rate too low (2.6%)
- HashMap lookup overhead ≈ direct function call cost
- Grid is only built once, so limited benefit

**Lesson:** Caching only helps with high hit rate (>20-30%).

**When caching helps:**
- ✅ High hit rate (>20-30%)
- ✅ Expensive computation (>10µs)
- ✅ Cache lookup < computation cost
- ✅ Repeated queries

**When caching hurts:**
- ❌ Low hit rate (<10%)
- ❌ Cache overhead ≈ computation cost
- ❌ One-time computation
- ❌ Memory overhead

**Example of successful caching:**
- Memoization in recursive algorithms (fibonacci, etc.)
- Hit rate: >50%
- Computation: >100µs
- Net: 10-100x speedup ✅

---

## Day 9: Profiling Insights

### Time Distribution (88ms total)
```
Build valid grid:    81.0ms  (92%)  ← Dominant bottleneck
  ├─ point_in_polygon calls: ~60,000
  └─ Each checks 496 edges
Rectangle search:     0.4ms   (0.5%) ← Already optimized!
Coordinate compress:  0.01ms  (0.01%)
Build prefix sum:     0.2ms   (0.2%)
Other:                6.4ms   (7%)
```

**Key finding:** 92% of time is in `build_valid_grid`

**Why further optimization was hard:**
- Need to validate 122,760 potential rectangles
- Each validation requires knowing which cells are valid
- Must check every grid cell for validity (no shortcuts for arbitrary polygons)
- Polygon is complex (496 edges) so containment is expensive

**What we can't avoid:**
- O(n²) pair checking (need all pairs)
- O(grid_size × polygon_complexity) grid building
- With current algorithm: ~88ms is near-optimal

---

## General Anti-Patterns

### ❌ Don't: Sort Without Measuring

**Pattern:** Sort items to enable early termination.

**Problem:** Sorting has O(n log n) overhead that must be justified.

**When it fails:**
- Most items need to be checked anyway
- Sorting cost > work saved
- Items have similar priorities

**Example:** Day 9 sorting pairs (failed)

**Solution:** Profile first. Only sort if you can skip >50% of work.

---

### ❌ Don't: Parallelize Small Work

**Pattern:** Use `par_iter()` on small work items.

**Problem:** Thread overhead dominates when work per item is small.

**When it fails:**
- Work per item < 10µs
- Thread overhead > work saved
- Work is already fast (O(1) operations)

**Example:** Day 9 parallelizing pair checks (failed)

**Solution:** Profile first. Need >10µs work per item for parallelization to help.

---

### ❌ Don't: Cache Without Hit Rate Analysis

**Pattern:** Add HashMap caching without analyzing hit rate.

**Problem:** Cache overhead can exceed savings if hit rate is low.

**When it fails:**
- Hit rate < 20%
- Cache overhead ≈ computation cost
- One-time computation

**Example:** Day 9 caching point-in-polygon (failed)

**Solution:** Measure hit rate first. Need >20-30% hit rate to be worthwhile.

---

### ❌ Don't: Optimize Without Profiling

**Pattern:** Optimize code without finding the bottleneck.

**Problem:** Optimizing the wrong part wastes time and may make code worse.

**Example:** Day 9 - could have optimized rectangle search (0.4ms / 98.7ms = 0.4% of runtime)

**Solution:** Profile first. Find the 80-90% bottleneck, optimize that.

---

## Success vs Failure Comparison

### Day 9: What Worked vs What Didn't

| Optimization | Result | Why |
|--------------|--------|-----|
| ✅ Single-pass point_in_polygon | 98.7ms → 88.4ms | Eliminated redundant loop |
| ✅ Geo crate | 98.7ms → 52.0ms | Professional library optimization |
| ❌ Sorting pairs | 98.7ms → 107.9ms | Sorting overhead > work saved |
| ❌ Rayon parallel | 98.7ms → 109.3ms | Thread overhead > work saved |
| ❌ HashMap cache | 88.4ms → 86.4ms | Low hit rate (2.6%) |

**Key insight:** Simple optimizations (single-pass, library) worked. Complex optimizations (sorting, parallelization) failed.

---

## Lessons Learned

### 1. Profile Before Optimizing
- Day 9 profiling revealed 92% time in one function
- Optimizing the 0.5% bottleneck would be waste
- **Tool:** Manual timing with `Instant::now()`

### 2. Not All "Standard" Optimizations Work
- Parallelization needs sufficient work per item
- Sorting has overhead that must be justified
- Caching needs high hit rate (>20-30%)

### 3. Sometimes "Simple" is Best
- Single-pass optimization: 10 lines changed, 10ms saved
- Complex optimizations (sorting, caching): failed
- Best wins come from algorithmic improvements, not tricks

### 4. Know When to Stop
- 88ms is fast (< 0.1 second)
- Further optimization would require algorithm redesign
- Diminishing returns for complex changes
- **Good enough is good enough!**

### 5. Measure Everything
- Every optimization attempt was benchmarked
- Willing to revert when results were worse
- Learned from failures, not just successes

---

## Optimization Workflow (Proven Process)

1. **Measure baseline** with `cargo solve --release`
2. **Profile** to find bottleneck (manual timing)
3. **Choose technique** based on bottleneck type
4. **Implement** one change at a time
5. **Benchmark** to verify improvement
6. **Keep or revert** based on results
7. **Repeat** until diminishing returns

**Critical:** Always measure before and after. Never assume an optimization will help.

---

## Red Flags: When to Avoid an Optimization

### 🚩 Red Flag 1: Work is Too Small
- Work per item < 10µs
- Thread overhead will dominate
- **Solution:** Don't parallelize

### 🚩 Red Flag 2: Most Items Need Checking
- Can't skip >50% of items
- Sorting overhead won't be justified
- **Solution:** Don't sort

### 🚩 Red Flag 3: Low Cache Hit Rate
- Hit rate < 20%
- Cache overhead ≈ computation cost
- **Solution:** Don't cache

### 🚩 Red Flag 4: Optimizing Wrong Part
- Target is <5% of runtime
- Won't make meaningful difference
- **Solution:** Profile first, find 80%+ bottleneck

---

## Conclusion

### What We Learned
- ✅ 3 out of 5 Day 9 optimizations made it slower
- ✅ Simple optimizations (single-pass, library) worked
- ✅ Complex optimizations (sorting, parallelization) failed
- ✅ Profiling is essential before optimizing

### Key Takeaway
**Not all optimizations are good.** The best approach:
1. Profile to find bottleneck
2. Try simple optimizations first
3. Measure every change
4. Be willing to revert
5. Learn from failures

**Remember:** Sometimes the best optimization is knowing when to stop!

