#![feature(portable_simd)]

use std::simd::{Simd, cmp::SimdPartialEq};

advent_of_code::solution!(6);

// SIMD lane size - typically 16, 32, or 64 bytes depending on CPU
const SIMD_LANE_SIZE: usize = 64;

// SIMD vector type for u8 operations
type SimdU8 = Simd<u8, SIMD_LANE_SIZE>;

// Helper function to create SIMD vector filled with a byte value
#[inline(always)]
fn simd_splat_u8(value: u8) -> SimdU8 {
    Simd::splat(value)
}

/// Represents a parsed math worksheet with precomputed digit values
struct Worksheet {
    bytes: Vec<u8>,
    digits: Vec<u8>, // Precomputed: digit values (0-9) or >9 for non-digits
    line_len: usize,
    operator_line_start: usize,
    operand_lines: usize,
}

impl Worksheet {
    /// Parse input and precompute digit values using SIMD
    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes().to_vec();
        let file_len = bytes.len();

        // Find first newline to determine line length
        let line_len = Self::find_line_length(&bytes);

        let operator_line_start = file_len - line_len;
        let operand_lines = operator_line_start / line_len;

        // Precompute digit table using SIMD for bulk conversion
        let digits = Self::precompute_digits(&bytes, operator_line_start);

        Self {
            bytes,
            digits,
            line_len,
            operator_line_start,
            operand_lines,
        }
    }

    /// Find the first newline using SIMD to determine line length
    fn find_line_length(bytes: &[u8]) -> usize {
        let mut pos = 0;

        // Process in SIMD chunks
        let newlines = simd_splat_u8(b'\n');
        while pos + SIMD_LANE_SIZE <= bytes.len() {
            let lane = SimdU8::from_slice(&bytes[pos..pos + SIMD_LANE_SIZE]);
            let mask = lane.simd_eq(newlines).to_bitmask();

            if mask != 0 {
                pos += mask.trailing_zeros() as usize;
                return pos + 1; // +1 to include the newline
            }

            pos += SIMD_LANE_SIZE;
        }

        // Handle remainder with scalar search
        while pos < bytes.len() {
            if bytes[pos] == b'\n' {
                return pos + 1;
            }
            pos += 1;
        }

        bytes.len() // No newline found, entire input is one line
    }

    /// Precompute digit values for all bytes using SIMD bulk conversion
    /// Returns a vector where digits are 0-9 and non-digits are >9
    fn precompute_digits(bytes: &[u8], len: usize) -> Vec<u8> {
        let mut digits = vec![0u8; len];
        let mut i = 0;

        // Bulk conversion using SIMD
        let ascii_zeroes = simd_splat_u8(b'0');
        while i + SIMD_LANE_SIZE <= len {
            let lane = SimdU8::from_slice(&bytes[i..i + SIMD_LANE_SIZE]);
            let values = lane - ascii_zeroes;

            // Copy SIMD results to output
            digits[i..i + SIMD_LANE_SIZE].copy_from_slice(values.as_array());
            i += SIMD_LANE_SIZE;
        }

        // Handle remainder with scalar conversion
        while i < len {
            digits[i] = bytes[i].wrapping_sub(b'0');
            i += 1;
        }

        digits
    }

    /// Parse a number from a column, reading digits from top to bottom (Part 1)
    fn parse_number_from_column(&self, column_start: usize, column_len: usize) -> u64 {
        let mut number = 0u64;
        let mut seen_digit = false;

        for line in 0..self.operand_lines {
            let start = line * self.line_len + column_start;
            let end = start + column_len;

            for i in start..end {
                let digit = self.digits[i];
                if digit < 10 {
                    number = number * 10 + digit as u64;
                    seen_digit = true;
                } else if seen_digit {
                    break; // End of number
                }
            }
        }

        number
    }

    /// Parse a number from a column, reading digits from bottom to top (Part 2)
    fn parse_number_from_column_reverse(&self, column_start: usize, column: usize) -> u64 {
        let mut number = 0u64;
        let mut seen_digit = false;

        for line in (0..self.operand_lines).rev() {
            let i = line * self.line_len + column_start + column;
            let digit = self.digits[i];

            if digit < 10 {
                number = number * 10 + digit as u64;
                seen_digit = true;
            } else if seen_digit {
                break; // End of number
            }
        }

        number
    }

    /// Get the operator character at a given column position
    fn get_operator(&self, column_start: usize) -> Option<u8> {
        if column_start < self.bytes.len() {
            Some(self.bytes[self.operator_line_start + column_start])
        } else {
            None
        }
    }

    /// Check if we've reached the end of the file
    fn is_end(&self, column_start: usize) -> bool {
        self.operator_line_start + column_start >= self.bytes.len()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let worksheet = Worksheet::parse(input);
    let mut total = 0u64;
    let mut column_start = 0;
    let mut column_len = 1;

    // Process each column of operations
    while !worksheet.is_end(column_start + column_len) {
        // Find the width of this column (until next space or newline)
        while column_start + column_len < worksheet.bytes.len()
            && worksheet.bytes[worksheet.operator_line_start + column_start + column_len] == b' '
        {
            column_len += 1;
        }

        // Include newline if we hit end of line
        if column_start + column_len < worksheet.bytes.len()
            && worksheet.bytes[worksheet.operator_line_start + column_start + column_len] == b'\n'
        {
            column_len += 1;
        }

        // Get operator for this column
        let operator = worksheet.get_operator(column_start)?;

        // Parse and process operands based on operator
        let result = match operator {
            b'+' => {
                // Sum all numbers in the column
                // Note: parse_number_from_column parses the entire column as one number
                // We need to parse each row's number separately
                let mut sum = 0u64;
                for row in 0..worksheet.operand_lines {
                    let start = row * worksheet.line_len + column_start;
                    let end = start + column_len;
                    let mut number = 0u64;
                    let mut seen_digit = false;

                    for i in start..end {
                        let digit = worksheet.digits[i];
                        if digit < 10 {
                            number = number * 10 + digit as u64;
                            seen_digit = true;
                        } else if seen_digit {
                            break;
                        }
                    }
                    sum += number;
                }
                sum
            }
            b'*' => {
                // Multiply all numbers in the column
                let mut product = 1u64;
                for row in 0..worksheet.operand_lines {
                    let start = row * worksheet.line_len + column_start;
                    let end = start + column_len;
                    let mut number = 0u64;
                    let mut seen_digit = false;

                    for i in start..end {
                        let digit = worksheet.digits[i];
                        if digit < 10 {
                            number = number * 10 + digit as u64;
                            seen_digit = true;
                        } else if seen_digit {
                            break;
                        }
                    }
                    product *= number;
                }
                product
            }
            _ => continue, // Skip invalid operators
        };

        total += result;
        column_start += column_len;
        column_len = 1;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let worksheet = Worksheet::parse(input);
    let mut total = 0u64;
    let mut column_start = 0;
    let mut column_len = 1;

    // Process each column of operations
    while !worksheet.is_end(column_start + column_len) {
        // Find the width of this column
        while column_start + column_len < worksheet.bytes.len()
            && worksheet.bytes[worksheet.operator_line_start + column_start + column_len] == b' '
        {
            column_len += 1;
        }

        // Include newline if we hit end of line
        if column_start + column_len < worksheet.bytes.len()
            && worksheet.bytes[worksheet.operator_line_start + column_start + column_len] == b'\n'
        {
            column_len += 1;
        }

        // Get operator for this column
        let operator = worksheet.get_operator(column_start)?;

        // For Part 2, we process columns right-to-left within each problem
        // Numbers are read bottom-to-top (most significant digit at top)
        let mut numbers = Vec::new();

        // Parse numbers from each column position (right to left)
        for column in (0..column_len - 1).rev() {
            let number = worksheet.parse_number_from_column_reverse(column_start, column);
            if number > 0 {
                numbers.push(number);
            }
        }

        // Apply operation to collected numbers
        let result = match operator {
            b'+' => numbers.iter().sum::<u64>(),
            b'*' => numbers.iter().product::<u64>(),
            _ => continue, // Skip invalid operators
        };

        total += result;
        column_start += column_len;
        column_len = 1;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
