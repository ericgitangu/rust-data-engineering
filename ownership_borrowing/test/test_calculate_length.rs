// cargo test

#[cfg(test)]
mod tests {
    use ownership_borrowing::calculate_length;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        let len = calculate_length(&s);
        assert_eq(len, 5, "Length of 'hello' should be 5");
    }
}
