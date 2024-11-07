use std::collections::HashMap;

/// Finds two numbers in the given vector that add up to the target value.
///
/// This function takes a vector of integers and a target integer. It returns a tuple of indices
/// of the two numbers such that they add up to the target. If no such numbers are found, it returns `None`.
///
/// # Arguments
///
/// * `nums` - A vector of integers.
/// * `target` - The target integer.
///
/// # Returns
///
/// An `Option` containing a tuple of indices of the two numbers that add up to the target, or `None` if no such numbers are found.
///
/// # Examples
///
/// ```
/// let nums = vec![2, 7, 11, 15];
/// let target = 9;
/// assert_eq!(two_sum::two_sum(nums, target), Some((0, 1)));
/// ```
fn two_sum(nums: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return Some((nums[index], nums[i]));
        }
        map.insert(num, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), Some((2, 7)));
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", two_sum(nums, target)); // Some((0, 1))
}
