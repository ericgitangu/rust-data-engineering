use std::collections::{HashMap, HashSet};

/// Functions to find duplicates and their frequencies in a vector.
///
/// This module provides two functions:
/// - `find_duplicates`: Finds all numbers that appear more than once in the input vector.
/// - `find_duplicates_frequency`: Returns a frequency map of all numbers in the input vector.
///
/// # Examples
///
/// ```
/// let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
///
/// // Find duplicates
/// let duplicates = find_duplicates(&nums);
/// assert_eq!(duplicates, vec![2, 3]);
///
/// // Find frequency of each number
/// let frequency = find_duplicates_frequency(nums);
/// assert_eq!(frequency, HashMap::from([(4, 1), (3, 2), (2, 2), (7, 1), (8, 1), (1, 1)]));
/// ```
fn find_duplicates(nums: &Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = vec![];

    for &num in nums {
        if !seen.insert(num) {
            duplicates.push(num);
        }
    }
    duplicates
}

fn find_duplicates_frequency(nums: Vec<i32>) -> HashMap<i32, i32> {
    let mut frequency = HashMap::new();

    for num in nums {
        *frequency.entry(num).or_insert(0) += 1;
    }
    frequency
}

fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("{:?}", find_duplicates(&nums)); // [2, 3]
    println!("{:?}", find_duplicates_frequency(nums.clone())); // {4: 1, 3: 2, 2: 2, 7: 1, 8: 1, 1: 1}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        assert_eq!(find_duplicates(&vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_frequency() {
        assert_eq!(
            find_duplicates_frequency(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            HashMap::from([(4, 1), (3, 2), (2, 2), (7, 1), (8, 1), (1, 1)])
        );
    }
}
