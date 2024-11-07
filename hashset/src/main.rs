use std::collections::HashSet;

/// Demonstrates basic operations on a `HashSet`.
///
/// This function creates a new `HashSet`, inserts elements, checks for
/// membership, iterates over the set, and removes an element.
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
///
/// let mut set: std::collections::HashSet<i32> = HashSet::new();
/// set.insert(1);
/// set.insert(2);
/// set.insert(3);
/// set.insert(3); // Duplicate, will not be added
///
/// assert!(set.contains(&2));
///
/// let mut values: Vec<_> = set.iter().cloned().collect();
/// values.sort();
/// assert_eq!(values, vec![1, 2, 3]);
///
/// set.remove(&1);
/// assert!(!set.contains(&1));
/// ```
fn main() {
    let mut set: std::collections::HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(3); // Duplicate, will not be added

    // Check if a value is in the set
    if set.contains(&2) {
        println!("Set contains 2");
    }

    // Iterate over the set
    for value in &set {
        println!("Value: {}", value);
    }

    // Remove an element
    set.remove(&1);
    println!("Set after removal: {:?}", set);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hashset() {
        main();
    }
    #[test]
    fn test_hashset_remove() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        set.remove(&1);
        assert_eq!(set.contains(&1), false);
    }
    #[test]
    fn test_hashset_contains() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        assert!(set.contains(&1));
    }
    #[test]
    fn test_hashset_iter() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn test_hashset_len() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        assert_eq!(set.len(), 1);
    }
    #[test]
    fn test_hashset_is_empty() {
        let set: std::collections::HashSet<i32> = HashSet::new();
        assert!(set.is_empty());
    }
    #[test]
    fn test_hashset_clear() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        set.clear();
        assert!(set.is_empty());
    }
    #[test]
    fn test_hashset_insert() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        assert!(set.contains(&1));
    }
    #[test]
    fn test_hashset_insert_duplicate() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        set.insert(1);
        assert_eq!(set.len(), 1);
    }
    #[test]
    fn test_hashset_union() {
        let mut set1: std::collections::HashSet<i32> = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2: std::collections::HashSet<i32> = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let union = set1.union(&set2).collect::<Vec<_>>();
        assert_eq!(union.len(), 3);
    }
    #[test]
    fn test_hashset_difference() {
        let mut set1: std::collections::HashSet<i32> = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2: std::collections::HashSet<i32> = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let difference = set1.difference(&set2).collect::<Vec<_>>();
        assert_eq!(difference.len(), 1);
    }
    #[test]
    fn test_hashset_intersection() {
        let mut set1: std::collections::HashSet<i32> = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2: std::collections::HashSet<i32> = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let intersection = set1.intersection(&set2).collect::<Vec<_>>();
        assert_eq!(intersection.len(), 1);
    }
    #[test]
    fn test_hashset_symmetric_difference() {
        let mut set1: std::collections::HashSet<i32> = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2: std::collections::HashSet<i32> = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let symmetric_difference = set1.symmetric_difference(&set2).collect::<Vec<_>>();
        assert_eq!(symmetric_difference.len(), 2);
    }
    #[test]
    fn test_hashset_retain() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.retain(|x| x % 2 == 0);
        assert_eq!(set.len(), 1);
    }
    #[test]
    fn test_hashset_eq() {
        let mut set1: std::collections::HashSet<i32> = HashSet::new();
        set1.insert(1);
        let mut set2: std::collections::HashSet<i32> = HashSet::new();
        set2.insert(1);
        assert_eq!(set1, set2);
    }
    #[test]
    fn test_hashset_ne() {
        let mut set1: std::collections::HashSet<i32> = HashSet::new();
        set1.insert(1);
        let mut set2: std::collections::HashSet<i32> = HashSet::new();
        set2.insert(2);
        assert_ne!(set1, set2);
    }
    #[test]
    fn test_hashset_clone() {
        let mut set: std::collections::HashSet<i32> = HashSet::new();
        set.insert(1);
        let cloned_set = set.clone();
        assert_eq!(set, cloned_set);
    }
}
