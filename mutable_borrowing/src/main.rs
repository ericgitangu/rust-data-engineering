use mutable_borrowing;

fn main() {
    let mut s = String::from("hello");
    mutable_borrowing::change_string(&mut s);
    println!("{}", s);
}
