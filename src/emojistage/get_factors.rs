/// Returns a vector of factors for a given number.
pub fn get_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
    }
    factors
} 