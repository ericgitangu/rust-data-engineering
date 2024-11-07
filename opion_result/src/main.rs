/// This example demonstrates how to use a `Vec` collection containing `Option<i32>`
/// values. It iterates over the vector and prints each value if it exists, or indicates
/// when a `None` is found.
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
