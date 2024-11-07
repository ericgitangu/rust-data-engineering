//! This program demonstrates the use of the `PrimeIterator` to generate prime numbers up to a specified limit.
//!
//! The `PrimeIterator` struct is used to create an iterator that yields prime numbers starting from 2 up to the given limit.
//! In this example, we generate all prime numbers up to 20 and print them.

use iterator_range_of_primes::PrimeIterator;

fn main() {
    let primes_up_to_20: Vec<u32> = PrimeIterator::new(20).collect();
    println!("Primes up to 20: {:?}", primes_up_to_20);
}
