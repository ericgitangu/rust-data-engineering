/// This example demonstrates how to use a `Vec` collection containing `Option<i32>`
/// values. It iterates over the vector and prints each value if it exists, or indicates
/// when a `None` is found.
///
/// The `Option` type is a powerful feature in Rust that represents a value that can either be
/// `Some(T)` (containing a value of type `T`) or `None` (indicating the absence of a value).
/// It is commonly used for safe handling of optional values, avoiding null pointer exceptions.
///
/// The `Result` type is another fundamental feature in Rust, used for error handling. It represents
/// either a success (`Ok(T)`, containing a value of type `T`) or an error (`Err(E)`, containing an error
/// value of type `E`). This allows for robust error handling and propagation.
///
/// # Examples
///
/// ```
/// let vec = vec![Some(1), None, Some(3)];
///
/// for value in vec {
///     match value {
///         Some(v) => println!("Value: {}", v),
///         None => println!("None found"),
///     }
/// }
/// ```
///
/// The output will be:
/// ```text
/// Value: 1
/// None found
/// Value: 3
/// ```
fn main() {
    let vec = vec![Some(1), None, Some(3)];

    for value in vec {
        match value {
            Some(v) => println!("Value: {}", v),
            None => println!("None found"),
        }
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
