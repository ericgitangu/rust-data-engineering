use std::sync::{Arc, Mutex};
use std::thread;

/// This example demonstrates how to use `Arc` and `Mutex` to safely share and modify data across multiple threads.
///
/// In the provided Rust code, the `*num` is a mutable reference to the integer inside the `Mutex`, which is wrapped in an `Arc`.
/// Here's how the value gets updated and eventually results in the final value of 10:
///
/// 1. **Mutex Locking**:
///    - Each thread locks the `Mutex` using `counter.lock().unwrap()`. This call returns a `MutexGuard`, which is a smart pointer that provides mutable access to the data inside the `Mutex`.
///
/// 2. **Increment Operation**:
///    - The line `*num += 1;` increments the value inside the `Mutex`. Here, `*num` dereferences the `MutexGuard` to access the integer, and the `+= 1` operation modifies it.
///
/// 3. **Automatic Unlocking**:
///    - When the `MutexGuard` goes out of scope (at the end of the closure), it automatically releases the lock on the `Mutex`. This is due to the `Drop` trait implementation for `MutexGuard`, which ensures the lock is released when the guard is dropped.
///
/// 4. **Shared Ownership with Arc**:
///    - The `Arc` (Atomic Reference Counted) allows multiple threads to own the `Mutex` concurrently. Each thread gets a clone of the `Arc`, which safely manages the reference count across threads.
///
/// 5. **Final Result**:
///    - After all threads have completed their execution and incremented the counter, the main thread locks the `Mutex` one last time to read the final value with `*counter.lock().unwrap()`. Since each of the 10 threads increments the counter by 1, the final value is 10.
///
/// The key to understanding this is recognizing that the `Mutex` ensures that only one thread can modify the counter at a time, and the `Arc` allows the `Mutex` to be shared safely across threads. Each thread's increment operation is applied to the same integer inside the `Mutex`, and the `MutexGuard` ensures that these operations are performed safely without data races.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, Mutex};
/// use std::thread;
///
/// let counter = Arc::new(Mutex::new(0));
/// let mut handles = vec![];
///
/// for _ in 1..=10 {
///     let counter = Arc::clone(&counter);
///     let handle = thread::spawn(move || {
///         let mut num = counter.lock().unwrap();
///         *num += 1;
///     });
///     handles.push(handle);
/// }
///
/// for handle in handles {
///     handle.join().unwrap();
/// }
///
/// assert_eq!(*counter.lock().unwrap(), 10);
/// ```

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
