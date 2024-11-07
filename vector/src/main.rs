/// Demonstrates basic operations on a vector in Rust.
///
/// This function creates a vector, performs various operations such as pushing elements,
/// accessing elements, iterating over elements, and removing elements. It prints the results
/// of these operations to the console.
///
/// # Examples
///
/// ```
/// let mut vec = Vec::new();
/// vec.push(1);
/// vec.push(2);
/// vec.push(3);
///
/// // Access elements
/// assert_eq!(vec[0], 1);
///
/// // Iterate over elements
/// for number in &vec {
///     println!("Number: {}", number);
/// }
///
/// // Remove the last element
/// vec.pop();
/// assert_eq!(vec, vec![1, 2]);
/// ```
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Access elements
    println!("The first element is: {}", vec[0]);

    // Iterate over elements
    for number in &vec {
        println!("Number: {}", number);
    }

    // Remove the last element
    vec.pop();
    println!("Vector after pop: {:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vector() {
        main();
    }
    #[test]
    fn test_vector_pop() {
        let mut vec = vec![1, 2, 3];
        vec.pop();
        assert_eq!(vec, vec![1, 2]);
    }
    #[test]
    fn test_vector_push() {
        let mut vec = vec![1, 2];
        vec.push(3);
        assert_eq!(vec, vec![1, 2, 3]);
    }
    #[test]
    fn test_vector_access() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec[0], 1);
    }
    #[test]
    fn test_vector_iterate() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_vector_len() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.len(), 3);
    }
    #[test]
    fn test_vector_is_empty() {
        let vec = vec![];
        assert_eq!(vec.is_empty(), true);
    }
    #[test]
    fn test_vector_clear() {
        let mut vec = vec![1, 2, 3];
        vec.clear();
        assert_eq!(vec, vec![]);
    }
    #[test]
    fn test_vector_contains() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.contains(&1), true);
    }
    #[test]
    fn test_vector_sort() {
        let mut vec = vec![3, 1, 2];
        vec.sort();
        assert_eq!(vec, vec![1, 2, 3]);
    }
    #[test]
    fn test_vector_reverse() {
        let mut vec = vec![1, 2, 3];
        vec.reverse();
        assert_eq!(vec, vec![3, 2, 1]);
    }
    #[test]
    fn test_vector_remove() {
        let mut vec = vec![1, 2, 3];
        vec.remove(1);
        assert_eq!(vec, vec![1, 3]);
    }
    #[test]
    fn test_vector_insert() {
        let mut vec = vec![1, 2, 3];
        vec.insert(1, 4);
        assert_eq!(vec, vec![1, 4, 2, 3]);
    }
    #[test]
    fn test_vector_dedup() {
        let mut vec = vec![1, 1, 2, 2, 3, 3];
        vec.dedup();
        assert_eq!(vec, vec![1, 2, 3]);
    }
    #[test]
    fn test_vector_retain() {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.retain(|&x| x % 2 == 0);
        assert_eq!(vec, vec![2, 4]);
    }
}
