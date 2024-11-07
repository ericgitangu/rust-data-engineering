use std::collections::HashMap;

/// Demonstrates the usage of `HashMap` and handling potential errors with `Result`.
///
/// This function creates a `HashMap`, inserts a key-value pair, and attempts to retrieve
/// the value associated with a key. If the key is found, it prints the value; otherwise,
/// it prints an error message.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// let mut map: HashMap<String, i32> = HashMap::new();
/// map.insert(String::from("a"), 10);
///
/// // Using Result to handle potential errors
/// let result = map.get("a").ok_or("Key not found");
///
/// match result {
///     Ok(value) => println!("Found: {}", value),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("a"), 10);

    // Using Result to handle potential errors
    let result = map.get("a").ok_or("Key not found");

    match result {
        Ok(value) => println!("Found: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
