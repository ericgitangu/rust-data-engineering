use binary_search_impl;

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    match binary_search_impl::binary_search(&arr, target) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
