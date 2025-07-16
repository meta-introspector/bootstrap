//! # Stage Properties
//!
//! This module defines the `StageProperties` struct, which holds the set of
//! calculated mathematical and conceptual properties for a given stage.

/// A container for the mathematical and physical properties of a stage.
#[derive(Debug, Clone)]
pub struct StageProperties {
    /// The number of the stage (e.g., 1 to 42).
    pub stage_number: u8,
    /// True if the stage number is a prime number.
    pub is_prime: bool,
    /// True if the stage number is a Fibonacci number.
    pub is_fibonacci: bool,
    /// True if the stage number is a perfect square.
    pub is_perfect_square: bool,
    /// A vector containing the prime factors of the stage number.
    pub factors: Vec<u64>,
    /// The calculated "resonance frequency" of the stage, based on its properties.
    pub resonance_frequency: f64,
}