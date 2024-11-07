//! A module for merging overlapping intervals.
//!
//! This module provides a function `merge_intervals` that takes a vector of intervals
//! and merges all overlapping intervals into a single interval.
//!
//! # Examples
//!
//! ```
//! use merge_intervals::merge_intervals;
//!
//! let intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
//! let merged = merge_intervals(intervals);
//! assert_eq!(merged, vec![(1, 6), (8, 10), (15, 18)]);
//! ```

/// Merges overlapping intervals.
///
/// This function takes a vector of tuples, where each tuple represents an interval
/// with a start and end. It returns a new vector of intervals where all overlapping
/// intervals have been merged.
///
/// # Arguments
///
/// * `intervals` - A vector of tuples representing the intervals to be merged.
///
/// # Returns
///
/// A vector of tuples representing the merged intervals.
///
/// # Examples
///
/// ```
/// let intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
/// let merged = merge_intervals(intervals);
/// assert_eq!(merged, vec![(1, 6), (8, 10), (15, 18)]);
/// ```
fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if intervals.is_empty() {
        return vec![];
    }

    intervals.sort_by_key(|interval| interval.0);
    let mut merged = vec![intervals[0]];

    for interval in intervals.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if interval.0 <= last.1 {
            last.1 = last.1.max(interval.1);
        } else {
            merged.push(interval);
        }
    }
    merged
}

fn main() {
    let intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
    println!("{:?}", merge_intervals(intervals)); // [(1, 6), (8, 10), (15, 18)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        assert_eq!(
            merge_intervals(vec![(1, 3), (2, 6), (8, 10), (15, 18)]),
            vec![(1, 6), (8, 10), (15, 18)]
        );
    }
}
