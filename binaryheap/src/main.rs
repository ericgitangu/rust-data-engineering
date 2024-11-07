use std::collections::BinaryHeap;

/// Demonstrates basic operations on a `BinaryHeap`.
///
/// This function creates a new `BinaryHeap`, adds some elements to it,
/// and performs various operations such as peeking, popping, and iterating
/// over the elements.
///
/// A `BTreeSet` is a set based on a balanced tree structure, which maintains
/// its elements in sorted order. It provides efficient methods for insertion,
/// deletion, and membership checking, with all operations having a time complexity
/// of O(log n). Unlike a `BinaryHeap`, which is a priority queue, a `BTreeSet` is
/// used when you need a collection of unique elements that are always sorted.
///
/// # Examples
///
/// ```
/// use std::collections::BinaryHeap;
///
/// let mut heap = BinaryHeap::new();
/// heap.push(1);
/// heap.push(5);
/// heap.push(3);
///
/// // Peek at the largest element
/// assert_eq!(heap.peek(), Some(&5));
///
/// // Remove the largest element
/// assert_eq!(heap.pop(), Some(5));
///
/// // Convert to a sorted vector
/// let sorted_vec = heap.clone().into_sorted_vec();
/// assert_eq!(sorted_vec, vec![3, 1]);
/// ```
fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(5);
    heap.push(3);

    // Peek at the largest element
    println!("Top element in max-heap is: {:?}", heap.peek().unwrap());

    // Remove the largest element
    println!("Popped element in max-heap: {:?}", heap.pop().unwrap());

    // Iterate over elements (Note: elements are not in sorted order)
    for element in heap.clone().into_sorted_vec() {
        println!("Heap element in max-heap: {}", element);
    }
    // Create a min-heap by using a BinaryHeap with Reverse
    let mut min_heap = BinaryHeap::new();
    min_heap.push(std::cmp::Reverse(1));
    min_heap.push(std::cmp::Reverse(5));
    min_heap.push(std::cmp::Reverse(3));

    // Peek at the smallest element
    println!(
        "Top element in min-heap is: {:?}",
        min_heap.peek().map(|x| x.0).unwrap()
    );

    // Remove the smallest element
    println!(
        "Popped element from min-heap: {:?}",
        min_heap.pop().map(|x| x.0).unwrap()
    );

    // Iterate over elements in the min-heap (Note: elements are not in sorted order)
    for element in min_heap.clone().into_sorted_vec() {
        println!("Min-heap element in min-heap: {}", element.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binaryheap() {
        main();
    }
    #[test]
    fn test_binaryheap_pop() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(5);
        heap.push(3);
        assert_eq!(heap.pop(), Some(5));
    }
    #[test]
    fn test_binaryheap_peek() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(5);
        heap.push(3);
        assert_eq!(heap.peek(), Some(&5));
    }
    #[test]
    fn test_binaryheap_into_sorted_vec() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(5);
        heap.push(3);
        let sorted_vec = heap.clone().into_sorted_vec();
        assert_eq!(sorted_vec, vec![1, 3, 5]);
    }
    #[test]
    fn test_binaryheap_eq() {
        let mut heap1 = BinaryHeap::new();
        heap1.push(1);
        let mut heap2 = BinaryHeap::new();
        heap2.push(1);
        let heap_vec: Vec<_> = heap1.clone().into_sorted_vec();
        let heap2_vec: Vec<_> = heap2.clone().into_sorted_vec();
        assert_eq!(heap_vec, heap2_vec);
    }
    #[test]
    fn test_binaryheap_ne() {
        let mut heap1 = BinaryHeap::new();
        heap1.push(1);
        let mut heap2 = BinaryHeap::new();
        heap2.push(2);
        let heap_vec: Vec<_> = heap1.clone().into_sorted_vec();
        let heap2_vec: Vec<_> = heap2.clone().into_sorted_vec();
        assert_ne!(heap_vec, heap2_vec);
    }
    #[test]
    fn test_binaryheap_clone() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        let cloned_heap = heap.clone();
        let heap_vec: Vec<_> = heap.clone().into_sorted_vec();
        let cloned_heap_vec: Vec<_> = cloned_heap.clone().into_sorted_vec();
        assert_eq!(heap_vec, cloned_heap_vec);
    }
}
