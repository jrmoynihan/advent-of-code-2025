# Inline Optimization Analysis

## When `#[inline]` Helps

1. **Small functions called in hot loops** - Eliminates function call overhead
2. **Functions called many times** - Reduces call overhead accumulation
3. **Functions that enable further optimizations** - Allows compiler to optimize across call boundaries
4. **Generic/trait methods** - Compiler might not inline automatically

## When `#[inline]` Doesn't Help

1. **Large functions** - Code bloat, instruction cache misses
2. **Functions called once** - No benefit
3. **Functions already inlined by compiler** - Redundant hint
4. **Cross-crate calls** - `#[inline]` only suggests, doesn't force

## Analysis by Day

### Day 1: ✅ Good Candidates

**`Direction::from_char`** - Called once per line in hot loop
- Small function (match statement)
- Called in `part_one` and `part_two` loops
- **Recommendation**: `#[inline]`

**`Direction::spin`** - Called once per line in hot loop  
- Small function (match + arithmetic)
- Called in `part_one` and `part_two` loops
- **Recommendation**: `#[inline]`

### Day 2: ❌ Not Needed

All code is already inline in the main functions. No separate helper functions called in loops.

### Day 3: ✅ Good Candidates

**`find_max_joltage`** - Called once per line
- Medium-sized function
- Called in `part_one` for every line
- **Recommendation**: `#[inline]` (might help, but compiler likely already inlines)

**`max_of_window`** - Called multiple times per line in `find_12_cell_joltage`
- Small function
- Called in hot loop (up to 12 times per line)
- **Recommendation**: `#[inline]`

**`find_12_cell_joltage`** - Called once per line
- Medium-sized function
- Called in `part_two` for every line
- **Recommendation**: `#[inline]` (might help)

### Day 4: ❌ Not Needed

Most code is commented out. The active code is already inline.

### Day 5: ⚠️ Marginal Benefit

**`sort_id_ranges`** - Called once per part
- Small function (just a sort)
- Called only once, not in a loop
- **Recommendation**: Probably not needed

**`merge_overlapping_ranges`** - Called once per part
- Medium function
- Called only once, not in a loop
- **Recommendation**: Probably not needed

**`capture_id_ranges_and_ingredient_ids`** - Called once per part
- Medium function
- Called only once, not in a loop
- **Recommendation**: Probably not needed

### Day 6: ✅ Already Optimized

**`simd_splat_u8`** - Already has `#[inline(always)]` ✅

**`Worksheet::find_line_length`** - Called once
- **Recommendation**: `#[inline]` (might help, but called only once)

**`Worksheet::precompute_digits`** - Called once
- **Recommendation**: `#[inline]` (might help, but called only once)

**`Worksheet::parse_number_from_column`** - Called many times
- Called in hot loops for Part 1
- **Recommendation**: `#[inline]`

**`Worksheet::parse_number_from_column_reverse`** - Called many times
- Called in hot loops for Part 2
- **Recommendation**: `#[inline]`

**`Worksheet::get_operator`** - Called many times
- Called in hot loops
- **Recommendation**: `#[inline]`

## Expected Performance Impact

| Day   | Function                    | Expected Impact                 |
| ----- | --------------------------- | ------------------------------- |
| Day 1 | `Direction::from_char`      | 1-3% faster                     |
| Day 1 | `Direction::spin`           | 1-3% faster                     |
| Day 3 | `max_of_window`             | 2-5% faster (called many times) |
| Day 3 | `find_max_joltage`          | 1-2% faster                     |
| Day 3 | `find_12_cell_joltage`      | 1-2% faster                     |
| Day 6 | `parse_number_from_column*` | 2-4% faster                     |
| Day 6 | `get_operator`              | 1-2% faster                     |

**Total expected improvement**: 5-15% across all days

## Recommendation Priority

### High Priority (Definite Benefit)
1. Day 1: `Direction::from_char`, `Direction::spin`
2. Day 3: `max_of_window` (called many times per line)
3. Day 6: `parse_number_from_column*`, `get_operator`

### Medium Priority (Likely Benefit)
1. Day 3: `find_max_joltage`, `find_12_cell_joltage`
2. Day 6: Helper functions called once but might enable optimizations

### Low Priority (Unlikely Benefit)
1. Day 5: Functions called only once
2. Day 2: Already inline
3. Day 4: Already inline

