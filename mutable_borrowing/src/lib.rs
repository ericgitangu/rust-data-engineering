/// Changes the string by appending ", world!" to it.
///
/// # Examples
///
/// ```
/// let mut s = String::from("hello");
/// mutable_borrowing::change_string(&mut s);
/// assert_eq!(s, "hello, world!");
/// ```

pub fn change_string(s: &mut String) {
    s.push_str(", world!");
}
