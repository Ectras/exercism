pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    // Can use subtraction because square_of_sum > sum_of_squares for n: u32.
    // Otherwise, abs_diff would be necessary to prevent underflows.
    square_of_sum(n) - sum_of_squares(n)
}
