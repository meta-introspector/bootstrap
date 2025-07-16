//! # Resonance Frequency Calculator
//!
//! This module provides a function to calculate the "resonance frequency" of a
//! given stage number. The frequency is a heuristic value derived from the
//! number's mathematical properties (primality, Fibonacci status, etc.).

/// Calculates a "resonance frequency" for a number based on its properties.
///
/// The calculation starts with the number itself and applies multipliers
/// if the number is prime, a Fibonacci number, or a perfect square. This value
/// is used to represent the stage's unique "vibe."
pub fn calculate_resonance_frequency(n: u64) -> f64 {
    let base_freq = n as f64;
    let prime_bonus = if crate::emojistage::is_prime(n) { 1.5 } else { 1.0 };
    let fibonacci_bonus = if crate::emojistage::is_fibonacci(n) { 1.3 } else { 1.0 };
    let square_bonus = if crate::emojistage::is_perfect_square(n) { 1.2 } else { 1.0 };
    base_freq * prime_bonus * fibonacci_bonus * square_bonus
}