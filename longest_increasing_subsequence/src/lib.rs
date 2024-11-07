/// Calculates the length of the longest increasing subsequence in a vector of integers.
///
/// This function uses a dynamic programming approach with binary search to efficiently
/// determine the length of the longest increasing subsequence (LIS) in the given vector.
///
/// # Arguments
///
/// * `nums` - A vector of integers for which the LIS length is to be calculated.
///
/// # Returns
///
/// The length of the longest increasing subsequence.
///
/// # Examples
///
/// ```
/// let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
/// assert_eq!(longest_increasing_subsequence::length_of_lis(nums), 4);
/// ```
///
/// ```
/// let nums = vec![1, 2, 3, 4, 5];
/// assert_eq!(longest_increasing_subsequence::length_of_lis(nums), 5);
/// ```
///
/// ```
/// let nums = vec![5, 4, 3, 2, 1];
/// assert_eq!(longest_increasing_subsequence::length_of_lis(nums), 1);
/// ```
pub fn length_of_lis(nums: Vec<i32>) -> usize {
    let mut lis = vec![];

    for num in nums {
        // Perform a binary search on the `lis` vector to find the position where `num` should be inserted.
        // If `num` is not found, `unwrap_or_else` returns the index where `num` can be inserted to maintain sorted order.
        let pos = lis.binary_search(&num).unwrap_or_else(|x| x);

        // If the position is equal to or greater than the length of `lis`, it means `num` is greater than all elements in `lis`.
        // Hence, we push `num` to the end of the `lis` vector.
        if pos >= lis.len() {
            lis.push(num);
        } else {
            // Otherwise, replace the element at the found position with `num` to maintain the smallest possible values in `lis`.
            lis[pos] = num;
        }
    }
    // Return the length of the `lis` vector, which represents the length of the longest increasing subsequence.
    lis.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_lis() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
    #[test]
    fn test_length_of_lis_empty() {
        assert_eq!(length_of_lis(vec![]), 0);
    }
    #[test]
    fn test_length_of_lis_single() {
        assert_eq!(length_of_lis(vec![1]), 1);
    }
    #[test]
    fn test_length_of_lis_sorted() {
        assert_eq!(length_of_lis(vec![1, 2, 3, 4, 5]), 5);
    }
    #[test]
    fn test_length_of_lis_decreasing() {
        assert_eq!(length_of_lis(vec![5, 4, 3, 2, 1]), 1);
    }
    #[test]
    fn test_length_of_lis_duplicate() {
        assert_eq!(length_of_lis(vec![1, 1, 1, 1, 1]), 1);
    }
    #[test]
    fn test_length_of_lis_large() {
        assert_eq!(length_of_lis(vec![1, 3, 5, 4, 7]), 4);
    }
}
