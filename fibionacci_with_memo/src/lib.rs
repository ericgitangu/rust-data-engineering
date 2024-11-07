use std::collections::HashMap;

/// Calculate the nth Fibonacci number using memoization.
///
/// This function uses a hash map to store previously computed Fibonacci numbers to avoid redundant calculations.
///
/// # Examples
///
/// ```
/// let n = 10;
/// use std::collections::HashMap;
/// use fibionacci_with_memo::fibonacci;
/// let result = fibionacci_with_memo::fibonacci(n, &mut HashMap::new());
/// assert_eq!(result, 55);
/// ```
pub fn fibonacci(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, result);
    result
}
