use ownership_borrowing;

fn main() {
    let s = String::from("hello");
    let len = ownership_borrowing::calculate_length(&s); // Immutable borrow
    println!("The length of '{}' is {}.", s, len);
}
