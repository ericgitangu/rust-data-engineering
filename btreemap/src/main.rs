use std::collections::BTreeMap;

/// Demonstrates basic usage of `BTreeMap` including insertion, iteration, and access.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
///
/// let mut map = BTreeMap::new();
/// map.insert("c", 3);
/// map.insert("a", 1);
/// map.insert("b", 2);
///
/// // Iterate in sorted order of keys
/// for (key, value) in &map {
///     println!("{}: {}", key, value);
/// }
///
/// // Access a specific value
/// if let Some(value) = map.get("a") {
///     println!("The value for 'a' is: {}", value);
/// }
/// ```
fn main() {
    let mut map = BTreeMap::new();
    map.insert("c", 3);
    map.insert("a", 1);
    map.insert("b", 2);

    // Iterate in sorted order of keys
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Access a specific value
    if let Some(value) = map.get("a") {
        println!("The value for 'a' is: {}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_btreemap() {
        main();
    }
    #[test]
    fn test_btreemap_get() {
        let mut map = BTreeMap::new();
        map.insert("c", 3);
        map.insert("a", 1);
        map.insert("b", 2);
        assert_eq!(map.get("a"), Some(&1));
    }
    #[test]
    fn test_btreemap_iter() {
        let mut map = BTreeMap::new();
        map.insert("c", 3);
        map.insert("a", 1);
        map.insert("b", 2);
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&"a", &1)));
        assert_eq!(iter.next(), Some((&"b", &2)));
        assert_eq!(iter.next(), Some((&"c", &3)));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_btreemap_eq() {
        let mut map1 = BTreeMap::new();
        map1.insert("c", 3);
        map1.insert("a", 1);
        map1.insert("b", 2);
        let mut map2 = BTreeMap::new();
        map2.insert("a", 1);
        map2.insert("b", 2);
        map2.insert("c", 3);
        assert_eq!(map1, map2);
    }
    #[test]
    fn test_btreemap_ne() {
        let mut map1 = BTreeMap::new();
        map1.insert("c", 3);
        map1.insert("a", 1);
        map1.insert("b", 2);
        let mut map2 = BTreeMap::new();
        map2.insert("a", 1);
        map2.insert("b", 2);
        assert_ne!(map1, map2);
    }
    #[test]
    fn test_btreemap_clone() {
        let mut map = BTreeMap::new();
        map.insert("c", 3);
        map.insert("a", 1);
        map.insert("b", 2);
        let cloned_map = map.clone();
        assert_eq!(map, cloned_map);
    }
}
