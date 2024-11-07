/// Finds the contiguous subarray with the maximum sum using Kadane’s Algorithm.
///
/// Given an integer array, this function finds the contiguous subarray with the maximum sum.
/// Kadane’s Algorithm keeps track of the maximum sum ending at each position.
///
/// # Arguments
///
/// * `nums` - A vector of integers.
///
/// # Returns
///
/// The maximum sum of the contiguous subarray.
///
/// # Examples
///
/// ```
/// let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
/// use kadane_max_subarray::max_subarray;
/// assert_eq!(kadane_max_subarray::max_subarray(nums), (6, 3, 6));
/// ```
pub fn max_subarray(nums: Vec<i32>) -> (i32, usize, usize) {
    let mut max_current = nums[0];
    let mut max_global = nums[0];
    let mut start = 0;
    let mut end = 0;
    let mut temp_start = 0;

    // Iterate over the elements of the vector starting from the second element
    for (i, &num) in nums.iter().enumerate().skip(1) {
        if num > max_current + num {
            max_current = num;
            temp_start = i;
        } else {
            max_current += num;
        }

        if max_current > max_global {
            max_global = max_current;
            start = temp_start;
            end = i;
        }
    }
    // Return the maximum sum found along with the start and end indices
    (max_global, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray() {
        assert_eq!(max_subarray(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), (6, 3, 6));
    }
}
