use fibionacci_with_memo::fibonacci;
use std::collections::HashMap;

fn main() {
    let mut memo = HashMap::new();
    let n = 40;
    println!("Fibonacci({}) = {}", n, fibonacci(n, &mut memo));
}
