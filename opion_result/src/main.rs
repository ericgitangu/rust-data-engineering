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

    #[test]
    fn test_main_output() {
        // Capture the output of the main function
        let output = {
            let mut buf = Vec::new();
            let mut writer = Box::new(&mut buf);
            let _ = std::io::set_output_capture(Some(writer));
            main();
            std::io::set_output_capture(None);
            String::from_utf8(buf).unwrap()
        };
        assert_eq!(output, "Value: 1\nNone found\nValue: 3\n");
    }
}
