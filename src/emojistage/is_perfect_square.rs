/// Checks if a number is a perfect square.
pub fn is_perfect_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
} 