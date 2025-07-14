use super::clifford_trait::Clifford;
use crate::godel::Godel;

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