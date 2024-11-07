/// Binary search implementation in Rust.
///
/// This function takes a sorted array and a target value, and returns the index of the target value in the array.
/// If the target value is not found, it returns `None`.
///
/// # Examples
///
/// ```
/// let arr = [1, 2, 3, 4, 5];
/// let target = 3;
/// assert_eq!(binary_search_impl::binary_search(&arr, target), Some(2));
/// ```
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as isize - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_val = arr[mid as usize];

        if mid_val == target {
            return Some(mid as usize);
        } else if mid_val < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 2, 3, 4, 5];
        let target = 3;
        assert_eq!(binary_search(&arr, target), Some(2));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [1, 2, 3, 4, 5];
        let target = 6;
        assert_eq!(binary_search(&arr, target), None);
    }
}
