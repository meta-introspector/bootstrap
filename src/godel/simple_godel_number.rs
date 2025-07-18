//! # Simple Gödel Number
//!
//! This module provides a basic, concrete implementation of the `GodelDyn` trait.
//! It is used as a straightforward way to represent a Gödel number when a simple,
//! no-frills implementation is needed.

/// Simple concrete type that implements GodelDyn
#[derive(Debug, Clone)]
pub struct SimpleGodelNumber {
    /// The numerical value of the Gödel number.
    pub value: u64,
}

impl SimpleGodelNumber {
    /// Creates a new SimpleGodelNumber.
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

impl Default for SimpleGodelNumber {
    fn default() -> Self {
        Self { value: 1 }
    }
}

impl super::godel_dyn_trait::GodelDyn for SimpleGodelNumber {
    fn godel_number(&self) -> u64 {
        self.value
    }
    
    fn is_godel_prime(&self) -> bool {
        // Simplified - in practice this would check if the number is prime
        self.value > 1
    }
    
    fn godel_factors(&self) -> Vec<u64> {
        // Simplified - in practice this would factor the number
        vec![self.value]
    }
    
    fn to_string(&self) -> String {
        format!("Gödel({})", self.value)
    }
    
    fn from_number(n: u64) -> Self {
        Self::new(n)
    }
    
    fn compose_numbers(&self, numbers: &[u64]) -> u64 {
        numbers.iter().product()
    }
    
    fn decompose_number(&self, n: u64) -> Vec<u64> {
        let mut factors = Vec::new();
        let mut remaining = n;
        let mut divisor = 2;
        
        while remaining > 1 {
            while remaining % divisor == 0 {
                factors.push(divisor);
                remaining /= divisor;
            }
            divisor += 1;
        }
        
        factors
    }
}