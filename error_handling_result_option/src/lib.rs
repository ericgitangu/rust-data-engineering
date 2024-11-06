use std::fs::File;
use std::io::{self, Read};

/// Reads the contents of a file into a string.
///
/// # Examples
///
/// ```
/// let content = error_handling_result_option::read_file(std::env::current_dir().unwrap().join("example.text").to_str().unwrap());
/// assert_eq!(content.is_ok(), true);
/// ```
pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
