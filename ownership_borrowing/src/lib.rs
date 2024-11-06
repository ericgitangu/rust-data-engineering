/// Calculates the length of a given string.
///
/// # Examples
///
/// ```
/// let s = String::from("hello");
/// let len = ownership_borrowing::calculate_length(&s);
/// assert_eq!(len, 5);
/// ```
pub fn calculate_length(s: &String) -> usize {
    s.len() // Only read access
}
