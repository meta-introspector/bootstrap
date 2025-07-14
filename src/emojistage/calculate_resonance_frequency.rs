pub fn calculate_resonance_frequency(n: u64) -> f64 {
    let base_freq = n as f64;
    let prime_bonus = if crate::emojistage::is_prime(n) { 1.5 } else { 1.0 };
    let fibonacci_bonus = if crate::emojistage::is_fibonacci(n) { 1.3 } else { 1.0 };
    let square_bonus = if crate::emojistage::is_perfect_square(n) { 1.2 } else { 1.0 };
    base_freq * prime_bonus * fibonacci_bonus * square_bonus
} 