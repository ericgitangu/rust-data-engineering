use std::collections::BTreeSet;

/// Demonstrates basic usage of `BTreeSet` including insertion, iteration, and membership check.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeSet;
///
/// let mut set: BTreeSet<i32> = BTreeSet::new();
/// set.insert(3);
/// set.insert(1);
/// set.insert(2);
/// set.insert(2); // Duplicate, will not be added
///
/// // Iterate over elements in sorted order
/// for value in &set {
///     println!("Value: {}", value);
/// }
///
/// // Check if a value exists
/// if set.contains(&2) {
///     println!("Set contains 2");
/// }
/// ```
fn main() {
    let mut set: BTreeSet<i32> = BTreeSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);
    set.insert(2); // Duplicate, will not be added

    // Iterate over elements in sorted order
    for value in &set {
        println!("Value: {}", value);
    }

    // Check if a value exists
    if set.contains(&2) {
        println!("Set contains 2");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn test_btreeset_contains() {
        let mut set: BTreeSet<i32> = BTreeSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);
        assert!(set.contains(&2));
    }

    #[test]
    fn test_btreeset_iter() {
        let mut set: BTreeSet<i32> = BTreeSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_btreeset_eq() {
        let mut set1: BTreeSet<i32> = BTreeSet::new();
        set1.insert(3);
        set1.insert(1);
        set1.insert(2);
        let mut set2: BTreeSet<i32> = BTreeSet::new();
        set2.insert(1);
        set2.insert(2);
        set2.insert(3);
        assert_eq!(set1, set2);
    }

    #[test]
    fn test_btreeset_ne() {
        let mut set1: BTreeSet<i32> = BTreeSet::new();
        set1.insert(3);
        set1.insert(1);
        let mut set2: BTreeSet<i32> = BTreeSet::new();
        set2.insert(1);
        assert_ne!(set1, set2);
    }

    #[test]
    fn test_btreeset_clone() {
        let mut set: BTreeSet<i32> = BTreeSet::new();
        set.insert(3);
        set.insert(1);
        let cloned_set = set.clone();
        assert_eq!(set, cloned_set);
    }
}
