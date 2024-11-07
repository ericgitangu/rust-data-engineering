use kadane_max_subarray::max_subarray;

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{:?}", max_subarray(nums)); // (6, 3, 6)
}
