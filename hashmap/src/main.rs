//! Demonstrates basic operations on a `HashMap` in Rust.
//!
//! This example shows how to create a `HashMap`, insert key-value pairs, access values by key,
//! iterate over key-value pairs, and remove a key-value pair. It also includes tests to verify
//! the functionality of these operations.
//!
//! A `HashMap` is a collection that stores key-value pairs, where each key is unique and associated
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! map.insert("a", 1);
//! map.insert("b", 2);
//! map.insert("c", 3);
//!
//! // Access a value by key
//! if let Some(value) = map.get("b") {
//!     assert_eq!(*value, 2);
//! }
//!
//! // Iterate over key-value pairs
//! for (key, value) in &map {
//!     println!("{}: {}", key, value);
//! }
//!
//! // Remove a key-value pair
//! map.remove("a");
//! assert_eq!(map.get("a"), None);
//! ```

use std::collections::HashMap;

fn main() {
    let mut map: std::collections::HashMap<String, i32> = HashMap::new();
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    map.insert(String::from("c"), 3);

    // Access a value by key
    if let Some(value) = map.get("b") {
        println!("The value for 'b' is: {}", value);
    }

    // Iterate over key-value pairs
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Remove a key-value pair
    map.remove("a");
    println!("Map after removal: {:?}", map);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hashmap() {
        main();
    }
    #[test]
    fn test_hashmap_remove() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.insert(String::from("a"), 1);
        map.remove("a");
        assert_eq!(map.get("a"), None);
    }
    #[test]
    fn test_hashmap_get() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.insert(String::from("a"), 1);
        assert_eq!(map.get("a"), Some(&1));
    }
    #[test]
    fn test_hashmap_iter() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.insert(String::from("a"), 1);
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&String::from("a"), &1)));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_hashmap_insert() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.insert(String::from("a"), 1);
        assert_eq!(map.get("a"), Some(&1));
    }
    #[test]
    fn test_hashmap_contains_key() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.insert(String::from("a"), 1);
        assert!(map.contains_key(&String::from("a")));
    }
    #[test]
    fn test_hashmap_clear() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.insert(String::from("a"), 1);
        map.clear();
        assert!(map.is_empty());
    }
    #[test]
    fn test_hashmap_len() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.insert(String::from("a"), 1);
        assert_eq!(map.len(), 1);
    }
    #[test]
    fn test_hashmap_is_empty() {
        let map: std::collections::HashMap<String, i32> = HashMap::new();
        assert!(map.is_empty());
    }
    #[test]
    fn test_hashmap_entry() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.entry(String::from("a")).or_insert(1);
        assert_eq!(map.get("a"), Some(&1));
    }
    #[test]
    fn test_hashmap_entry_or_insert_with() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.entry(String::from("a")).or_insert_with(|| 1);
        assert_eq!(map.get("a"), Some(&1));
    }
    #[test]
    fn test_hashmap_entry_or_insert() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.entry(String::from("a")).or_insert(1);
        assert_eq!(map.get("a"), Some(&1));
    }
    #[test]
    fn test_hashmap_entry_or_insert_with_closure() {
        let mut map: std::collections::HashMap<String, i32> = HashMap::new();
        map.entry(String::from("a")).or_insert_with(|| 1);
        assert_eq!(map.get("a"), Some(&1));
    }
}
