//! # Clifford-Gödel Trait
//!
//! This module defines the `CliffordGodel` trait, which bridges the gap
//! between the Clifford algebra module and the Gödel numbering system.
//!
//! By implementing this trait, a type can have both a multivector representation
//! and a unique numerical identifier, allowing algebraic structures to be
//! encoded, stored, and analyzed as numbers.

use super::clifford_trait::Clifford;
use crate::godel::Godel;

/// A trait that combines the `Clifford` and `Godel` traits.
pub trait CliffordGodel: Clifford + Godel {
    /// Get the Gödel number of this Clifford multivector
    fn clifford_godel_number(&self) -> u64 {
        self.godel_number()
    }
    
    /// Create a Clifford multivector from a Gödel number
    fn from_clifford_godel_number(n: u64) -> Option<Self> where Self: Sized;
    
    /// Compose multiple Clifford multivectors into a single Gödel number
    fn compose_clifford_godel(multivectors: &[Self]) -> u64 where Self: Sized;
    
    /// Decompose a Gödel number back into Clifford multivectors
    fn decompose_clifford_godel(n: u64) -> Vec<Self> where Self: Sized;
}