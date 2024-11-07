/// An iterator over prime numbers up to a specified limit.
///
/// The `PrimeIterator` struct allows iteration over prime numbers starting from 2 up to a given limit.
/// It implements the `Iterator` trait, providing a `next` method to retrieve the next prime number.
///
/// # Examples
///
/// ```
/// use primes_iterator_range::PrimeIterator;
///
/// let primes: Vec<u32> = PrimeIterator::new(10).collect();
/// assert_eq!(primes, vec![2, 3, 5, 7]);
/// ```
pub struct PrimeIterator {
    current: u32,
    limit: u32,
}

impl PrimeIterator {
    /// Creates a new `PrimeIterator` with a specified upper limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - The upper limit up to which prime numbers will be generated.
    ///
    /// # Returns
    ///
    /// A `PrimeIterator` instance.
    pub fn new(limit: u32) -> Self {
        PrimeIterator { current: 2, limit }
    }

    /// Determines if a given number is prime.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to check for primality.
    ///
    /// # Returns
    ///
    /// `true` if the number is prime, `false` otherwise.
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    /// Returns the next prime number in the sequence, or `None` if the limit is reached.
    fn next(&mut self) -> Option<Self::Item> {
        while self.current <= self.limit {
            let n = self.current;
            self.current += 1;
            if PrimeIterator::is_prime(n) {
                return Some(n);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_iterator() {
        let primes: Vec<u32> = PrimeIterator::new(10).collect();
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }
}
