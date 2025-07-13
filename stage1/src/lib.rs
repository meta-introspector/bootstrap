//! # Bootstrap Stage 1 - Duality and Paired Systems
//!
//! Stage 1 introduces duality concepts, building upon the minimal bootstrap foundation.
//! This stage explores paired systems, dual operations, and complementary structures.
//!
//! ## Duality Principles
//!
//! - **Paired Operations**: Every operation has a dual counterpart
//! - **Complementary Storage**: Dual storage backends working together
//! - **Mirror Structures**: Symmetric data organization
//! - **Dual Hashing**: Paired hash algorithms for verification
//! - **Cyclic Duality**: 42-step cycle with dual phases

pub mod dual;
pub mod mirror;
pub mod paired;
pub mod symmetry;

pub use dual::{DualBootstrap, DualOperation, DualPhase};
pub use mirror::{MirrorStore, MirrorArtifact};
pub use paired::{PairedHasher, PairedStorage};
pub use symmetry::{SymmetryKernel, SymmetryCycle};

/// The dual bootstrap system that operates in complementary pairs
pub struct Stage1Bootstrap {
    primary: bootstrap::Bootstrap,
    dual: bootstrap::Bootstrap,
    phase: DualPhase,
}

impl Stage1Bootstrap {
    /// Creates a new dual bootstrap system
    pub fn new() -> Self {
        Self {
            primary: bootstrap::Bootstrap::new(),
            dual: bootstrap::Bootstrap::new(),
            phase: DualPhase::Primary,
        }
    }

    /// Performs a dual operation (stores in both systems)
    pub fn dual_store(&mut self, content: Vec<u8>) -> (bootstrap::Hash, bootstrap::Hash) {
        let primary_hash = self.primary.store(content.clone());
        let dual_hash = self.dual.store(content);
        
        self.advance_phase();
        (primary_hash, dual_hash)
    }

    /// Retrieves from the current phase's system
    pub fn retrieve(&self, hash: &bootstrap::Hash) -> Option<&bootstrap::Artifact> {
        match self.phase {
            DualPhase::Primary => self.primary.retrieve(hash),
            DualPhase::Dual => self.dual.retrieve(hash),
        }
    }

    /// Gets the current duality phase
    pub fn phase(&self) -> DualPhase {
        self.phase
    }

    /// Gets the combined cycle step (sum of both systems)
    pub fn combined_cycle(&self) -> u64 {
        self.primary.cycle_step() + self.dual.cycle_step()
    }

    /// Advances to the next duality phase
    fn advance_phase(&mut self) {
        self.phase = match self.phase {
            DualPhase::Primary => DualPhase::Dual,
            DualPhase::Dual => DualPhase::Primary,
        };
    }
}

impl Default for Stage1Bootstrap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1_creation() {
        let stage1 = Stage1Bootstrap::new();
        assert_eq!(stage1.phase(), DualPhase::Primary);
        assert_eq!(stage1.combined_cycle(), 0);
    }

    #[test]
    fn test_dual_store() {
        let mut stage1 = Stage1Bootstrap::new();
        let content = b"dual content".to_vec();
        
        let (primary_hash, dual_hash) = stage1.dual_store(content.clone());
        
        // Both hashes should be different (different systems)
        assert_ne!(primary_hash, dual_hash);
        
        // Both should be retrievable
        assert!(stage1.primary.retrieve(&primary_hash).is_some());
        assert!(stage1.dual.retrieve(&dual_hash).is_some());
    }

    #[test]
    fn test_phase_advancement() {
        let mut stage1 = Stage1Bootstrap::new();
        assert_eq!(stage1.phase(), DualPhase::Primary);
        
        stage1.dual_store(b"test".to_vec());
        assert_eq!(stage1.phase(), DualPhase::Dual);
        
        stage1.dual_store(b"test2".to_vec());
        assert_eq!(stage1.phase(), DualPhase::Primary);
    }
} 