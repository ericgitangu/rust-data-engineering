use error_handling_result_option;

fn main() {
    let base_dir = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", base_dir);
    match error_handling_result_option::read_file(base_dir.join("example.text").to_str().unwrap()) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
