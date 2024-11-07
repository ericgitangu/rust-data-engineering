use std::collections::VecDeque;

/// This is the main function that demonstrates the usage of `VecDeque`.
///
/// A `VecDeque` is a double-ended queue implemented using a `Vec`, providing efficient
/// insertion and deletion of elements at both ends.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
///
/// let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
/// deque.push_back(1);
/// deque.push_front(0);
/// deque.push_back(2);
///
/// // Access elements
/// assert_eq!(deque.front(), Some(&0));
/// assert_eq!(deque.back(), Some(&2));
///
/// // Iterate over elements
/// let mut iter = deque.iter();
/// assert_eq!(iter.next(), Some(&0));
/// assert_eq!(iter.next(), Some(&1));
/// assert_eq!(iter.next(), Some(&2));
///
/// // Remove elements from both ends
/// deque.pop_front();
/// deque.pop_back();
/// assert_eq!(deque, vec![1]);
/// ```
fn main() {
    let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_front(0);
    deque.push_back(2);

    // Access elements
    println!("Front element: {:?}", deque.front());
    println!("Back element: {:?}", deque.back());

    // Iterate over elements
    for element in &deque {
        println!("Element: {}", element);
    }

    // Remove elements from both ends
    deque.pop_front();
    deque.pop_back();
    println!("Deque after popping: {:?}", deque);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vecdeque() {
        main();
    }

    #[test]
    fn test_vecdeque_front() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_front(1);
        assert_eq!(deque.front(), Some(&1));
    }

    #[test]
    fn test_vecdeque_back() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        assert_eq!(deque.back(), Some(&1));
    }

    #[test]
    fn test_vecdeque_iter() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        let mut iter = deque.iter();
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_vecdeque_pop_front() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        assert_eq!(deque.pop_front(), Some(1));
    }

    #[test]
    fn test_vecdeque_pop_back() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        assert_eq!(deque.pop_back(), Some(1));
    }

    #[test]
    fn test_vecdeque_len() {
        let deque: std::collections::VecDeque<i32> = VecDeque::new();
        assert_eq!(deque.len(), 0);
    }

    #[test]
    fn test_vecdeque_is_empty() {
        let deque: std::collections::VecDeque<i32> = VecDeque::new();
        assert!(deque.is_empty());
    }

    #[test]
    fn test_vecdeque_clear() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.clear();
        assert!(deque.is_empty());
    }

    #[test]
    fn test_vecdeque_contains() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        assert!(deque.contains(&1));
    }

    #[test]
    fn test_vecdeque_reserve() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.reserve(10);
        assert_eq!(deque.capacity(), 10);
    }

    #[test]
    fn test_vecdeque_shrink_to_fit() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.shrink_to_fit();
        assert_eq!(deque.capacity(), 1);
    }

    #[test]
    fn test_vecdeque_resize() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.resize(10, 0);
        assert_eq!(deque.len(), 10);
    }

    #[test]
    fn test_vecdeque_truncate() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.truncate(0);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_vecdeque_retain() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_back(4);
        deque.push_back(5);
        deque.push_back(6);
        deque.retain(|x| x % 2 == 0);
        assert_eq!(deque, vec![2, 4, 6]);
    }

    #[test]
    fn test_vecdeque_rotate_left() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.rotate_left(1);
        assert_eq!(deque, vec![2, 3, 1]);
    }

    #[test]
    fn test_vecdeque_rotate_right() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.rotate_right(1);
        assert_eq!(deque, vec![3, 1, 2]);
    }

    #[test]
    fn test_vecdeque_into_iter() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        let mut iter = deque.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_vecdeque_iter_mut() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        let mut iter = deque.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn test_vecdeque_drain() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        let drained = deque.drain(..).collect::<Vec<_>>();
        assert_eq!(drained, vec![1, 2, 3]);
    }

    #[test]
    fn test_vecdeque_eq() {
        let mut deque1: std::collections::VecDeque<i32> = VecDeque::new();
        deque1.push_back(1);
        let mut deque2: std::collections::VecDeque<i32> = VecDeque::new();
        deque2.push_back(1);
        assert_eq!(deque1, deque2);
    }

    #[test]
    fn test_vecdeque_ne() {
        let mut deque1: std::collections::VecDeque<i32> = VecDeque::new();
        deque1.push_back(1);
        let mut deque2: std::collections::VecDeque<i32> = VecDeque::new();
        deque2.push_back(2);
        assert_ne!(deque1, deque2);
    }

    #[test]
    fn test_vecdeque_fmt() {
        let mut deque: std::collections::VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert_eq!(format!("{:?}", deque), "[1, 2, 3]");
    }
}
