# Day 6: SIMD vs Current Approach Comparison

## Current Approach

### Data Structure
- **Part 1**: `Grid<&str>` - stores string slices
- **Part 2**: `Grid<u8>` - stores raw bytes
- Uses `grid` crate abstraction with iterators

### Algorithm Flow

**Part 1**:
1. Parse input into `Grid<&str>` (splits on whitespace)
2. Iterate columns using `iter_cols()`
3. For each column:
   - Extract operator from last element
   - Parse remaining elements as `u64` using `parse::<u64>()`
   - Apply operation (sum or product)
4. Sum all results

**Part 2**:
1. Parse input into `Grid<u8>` (raw bytes)
2. Iterate columns in reverse using `iter_cols().rev()`
3. For each column:
   - Parse digits manually from bottom to top
   - Accumulate numbers until operator found
   - Apply operation when operator encountered
4. Sum all results

### Pros
- ✅ Clean, idiomatic Rust
- ✅ Uses standard library parsing
- ✅ Easy to understand and maintain
- ✅ Safe (no unsafe code)

### Cons
- ❌ **String allocation overhead**: Part 1 creates many string slices
- ❌ **Parse overhead**: `parse::<u64>()` is general-purpose but slower
- ❌ **Grid abstraction overhead**: Iterator chains, Option unwrapping
- ❌ **Multiple passes**: Grid construction + column iteration
- ❌ **Memory overhead**: Stores entire grid structure

---

## SIMD Approach

### Data Structure
- Works directly on `input.as_bytes()` - raw byte slice
- Precomputes digit table: `Vec<u8>` mapping ASCII to digit values
- No grid structure - direct byte array access

### Algorithm Flow

**Both Parts**:
1. **Find line length**: Use SIMD to find first newline
2. **Precompute digit table**: 
   - Use SIMD to bulk-convert ASCII bytes to digit values
   - `bytes[i] - b'0'` for all bytes in parallel
   - Store in `digits` array (0-9 for digits, >9 for non-digits)
3. **Process columns**:
   - Iterate through columns directly from byte array
   - Parse numbers using precomputed digit table
   - Apply operations inline
4. **Single pass**: Everything computed in one function

### Key Optimizations

1. **SIMD Digit Conversion**:
   ```rust
   let lane = Simd::from_slice(&bytes[i..i + LANE_SIZE]);
   let values = lane - ASCII_ZEROES;  // Parallel subtraction
   ```
   - Processes 16-64 bytes at once (depending on SIMD width)
   - Eliminates per-byte loop overhead

2. **Precomputed Digit Table**:
   - Converts all ASCII to numeric values upfront
   - Fast lookup: `digits[i]` instead of `bytes[i] - b'0'`
   - Non-digits become values >9, easy to filter

3. **Direct Byte Access**:
   - No string allocation
   - No parsing overhead
   - Direct pointer arithmetic (unsafe but fast)

4. **Unsafe Optimizations**:
   - `unsafe` pointer operations for bounds-checked but known-safe accesses
   - Eliminates bounds checking overhead

### Pros
- ✅ **Massive speedup**: SIMD processes 16-64 bytes at once
- ✅ **No allocation**: Works directly on input bytes
- ✅ **No parsing overhead**: Precomputed digit table
- ✅ **Single pass**: Everything in one function
- ✅ **Cache-friendly**: Sequential byte access

### Cons
- ❌ **Complex code**: Harder to understand and maintain
- ❌ **Unsafe code**: Requires careful reasoning about safety
- ❌ **Platform-specific**: SIMD width varies by CPU
- ❌ **Less idiomatic**: Doesn't use standard Rust patterns

---

## Performance Analysis

### Current Approach Overhead

| Operation         | Cost           | Frequency   |
| ----------------- | -------------- | ----------- |
| String allocation | ~10-50 cycles  | Per token   |
| `parse::<u64>()`  | ~50-200 cycles | Per number  |
| Grid iterator     | ~5-10 cycles   | Per element |
| Option unwrapping | ~1-2 cycles    | Per access  |
| Bounds checking   | ~1-2 cycles    | Per access  |

**Total overhead**: ~100-300 cycles per number parsed

### SIMD Approach Overhead

| Operation             | Cost         | Frequency       |
| --------------------- | ------------ | --------------- |
| SIMD digit conversion | ~5-10 cycles | Per 16-64 bytes |
| Digit table lookup    | ~1 cycle     | Per byte        |
| Unsafe pointer access | ~1 cycle     | Per access      |
| Manual parsing        | ~5-10 cycles | Per number      |

**Total overhead**: ~10-20 cycles per number parsed

### Expected Performance Improvement

**Part 1**:
- Current: ~85µs (from benchmarks)
- SIMD: **~15-25µs** (3-5x faster)
  - Eliminates string parsing overhead
  - SIMD bulk conversion is 10-20x faster

**Part 2**:
- Current: ~190µs (from benchmarks)
- SIMD: **~20-35µs** (5-9x faster)
  - Precomputed digit table eliminates per-byte conversion
  - Direct byte access is much faster than Grid iteration

---

## Key Differences

### 1. **Digit Conversion**

**Current**:
```rust
// Part 1: String parsing
operand.parse::<u64>().ok()

// Part 2: Manual conversion
if b >= b'0' && b <= b'9' {
    Some((b - b'0') as u64)
}
```

**SIMD**:
```rust
// Bulk conversion upfront
let lane = Simd::from_slice(&bytes[i..i + LANE_SIZE]);
let values = lane - ASCII_ZEROES;  // 16-64 bytes at once!
```

**Impact**: SIMD processes 16-64 bytes in the time it takes to process 1 byte

### 2. **Memory Access Pattern**

**Current**:
- Grid abstraction with iterators
- Multiple allocations (strings, grid structure)
- Indirect access through abstraction layers

**SIMD**:
- Direct byte array access
- Sequential memory access (cache-friendly)
- Single allocation (digit table)

### 3. **Parsing Strategy**

**Current**:
- General-purpose `parse::<u64>()` for Part 1
- Manual digit-by-digit parsing for Part 2
- Multiple passes through data

**SIMD**:
- Precomputed digit table (one pass)
- Fast digit extraction using table lookup
- Single pass through entire input

---

## When SIMD Wins

SIMD is particularly effective here because:

1. **Bulk operations**: Converting many ASCII bytes to digits is perfect for SIMD
2. **Simple arithmetic**: `byte - b'0'` is trivial to vectorize
3. **Sequential data**: Input is contiguous, perfect for SIMD loads
4. **High throughput**: Processing 16-64 bytes at once vs 1 byte at a time

---

## Trade-offs

### Current Approach
- ✅ **Readability**: Easy to understand
- ✅ **Maintainability**: Standard Rust patterns
- ✅ **Safety**: No unsafe code
- ❌ **Performance**: 3-9x slower

### SIMD Approach
- ✅ **Performance**: 3-9x faster
- ✅ **Memory**: Less allocation
- ❌ **Complexity**: Harder to understand
- ❌ **Safety**: Requires unsafe code
- ❌ **Portability**: SIMD width varies

---

## Recommendation

**For AoC/competitive programming**: Use SIMD approach
- Performance matters
- Code is used once
- 3-9x speedup is significant

**For production code**: Consider hybrid approach
- Use SIMD for hot paths (digit conversion)
- Keep safe abstractions elsewhere
- Add comprehensive tests for unsafe code

**For learning**: Current approach is fine
- More idiomatic Rust
- Easier to understand
- Good enough for AoC (85µs vs 190µs is still very fast)

---

## Implementation Notes

The SIMD approach requires:
1. `std::simd` (portable_simd) - available in nightly or via `portable-simd` crate
2. Careful bounds checking (even with unsafe)
3. Handling remainder after SIMD chunks
4. Platform-specific SIMD width (16, 32, 64 bytes)

The code shown uses:
- `Simd::from_slice()` - load SIMD vector
- `simd_eq()` - parallel comparison
- `to_bitmask()` - convert comparison to bitmask
- `trailing_zeros()` - find first match

