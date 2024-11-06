// cargo test

mod error_handling_result_option;

#[test]
fn test_read_file() {
    let base_dir = std::env::current_dir().unwrap();
    let content =
        error_handling_result_option::read_file(base_dir.join("example.text").to_str().unwrap());
    assert_eq!(content.is_ok(), true);
}

#[test]
fn test_read_file_error() {
    let base_dir = std::env::current_dir().unwrap();
    let content = error_handling_result_option::read_file(
        base_dir.join("non_existent.text").to_str().unwrap(),
    );
    assert_eq!(content.is_err(), true);
}
