use std::collections::LinkedList;

/// Demonstrates basic operations on a `LinkedList` including a custom frequency function.
///
/// This function creates a new `LinkedList`, adds some elements to it,
/// and performs various operations such as iterating, popping elements,
/// and calculating the frequency of elements.
///
/// A `LinkedList` is a doubly linked list, which provides efficient insertion and deletion
/// of elements at both ends, but less efficient access to elements by index compared to a `Vec`.
///
/// # Examples
///
/// ```
/// use std::collections::LinkedList;
///
/// let mut list = LinkedList::new();
/// list.push_back(1);
/// list.push_back(2);
/// list.push_front(0);
///
/// // Iterate over elements
/// for element in &list {
///     println!("Element: {}", element);
/// }
///
/// // Remove elements from the front and back
/// list.pop_front();
/// list.pop_back();
/// println!("List after popping: {:?}", list);
///
/// // Calculate frequency of elements
/// let freq = frequency(&list, &1);
/// assert_eq!(freq, 1);
/// ```

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);

    // Iterate over elements
    for element in &list {
        println!("Element: {}", element);
    }

    // Remove elements from the front and back
    list.pop_front();
    list.pop_back();
    println!("List after popping: {:?}", list);

    // Calculate frequency of elements
    let freq = frequency(&list, &1);
    println!("Frequency of 1: {}", freq);
}

/// Calculates the frequency of a given element in the `LinkedList`.
///
/// # Arguments
///
/// * `list` - A reference to the `LinkedList` to search.
/// * `value` - The value to count in the list.
///
/// # Returns
///
/// The frequency of the value in the list.
fn frequency<T: PartialEq>(list: &LinkedList<T>, value: &T) -> usize {
    list.iter().filter(|x| *x == value).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linkedlist() {
        main();
    }

    #[test]
    fn test_linkedlist_pop_front() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.pop_front();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_linkedlist_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.pop_back();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_linkedlist_iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_linkedlist_push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_linkedlist_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_linkedlist_clear() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.clear();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_linkedlist_contains() {
        let mut list = LinkedList::new();
        list.push_back(1);
        assert!(list.contains(&1));
    }

    // #[test]
    // fn test_linkedlist_remove() {
    //     let mut list = LinkedList::new();
    //     list.push_back(1);
    //     list.remove(&1).unwrap();
    //     assert!(!list.contains(&1));
    // }
    #[test]
    fn test_linkedlist_len() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_linkedlist_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_linkedlist_peek_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        assert_eq!(list.front(), Some(&1));
    }

    #[test]
    fn test_linkedlist_peek_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        assert_eq!(list.back(), Some(&1));
    }

    // #[test]
    // fn test_linkedlist_insert() {
    //     let mut list = LinkedList::new();
    //     list.push_back(1);
    //     list.push_back(3);
    //     list.insert(1, 2); // Assuming insert method takes index and value
    //     assert_eq!(list.len(), 3);
    //     assert_eq!(list.front(), Some(&1));
    //     assert_eq!(list.back(), Some(&3));
    //     assert_eq!(list.at(1), Some(&2)); // Assuming get method retrieves value at index
    // }

    #[test]
    fn test_linkedlist_frequency() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(1);
        let freq = frequency(&list, &1);
        assert_eq!(freq, 2);
    }
}
