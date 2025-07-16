//! # Dynamic Gödel Trait
//!
//! This module provides a `dyn`-safe version of the `Godel` trait.
//! It is designed for scenarios where you need to work with Gödel-numbered
//! objects through trait objects, which requires a simplified, object-safe interface.

/// Dyn-compatible interface for Godel operations
/// This trait provides a simplified interface that can be used with trait objects
pub trait GodelDyn {
    /// Get the Gödel number for this item
    fn godel_number(&self) -> u64;
    
    /// Check if this item is a Gödel prime
    fn is_godel_prime(&self) -> bool;
    
    /// Get the Gödel factors of this item
    fn godel_factors(&self) -> Vec<u64>;
    
    /// Get a string representation of this item
    fn to_string(&self) -> String;
    
    /// Create a new Godel item from a number
    fn from_number(n: u64) -> Self where Self: Sized;
    
    /// Compose multiple numbers into a single Gödel number
    fn compose_numbers(&self, numbers: &[u64]) -> u64;
    
    /// Decompose a Gödel number back into numbers
    fn decompose_number(&self, n: u64) -> Vec<u64>;
}