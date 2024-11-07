use primes_iterator_range::PrimeIterator;

fn main() {
    let primes_up_to_20: Vec<u32> = PrimeIterator::new(20).collect();
    println!("Primes up to 20: {:?}", primes_up_to_20);
}
