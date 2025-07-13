//! Duality concepts and operations

use bootstrap::{Bootstrap, Hash, Artifact};

/// Represents the duality phase of the system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualPhase {
    /// Primary phase - operating on the main system
    Primary,
    /// Dual phase - operating on the complementary system
    Dual,
}

/// Types of dual operations
#[derive(Debug, Clone, PartialEq)]
pub enum DualOperation {
    /// Store operation in both systems
    Store { content: Vec<u8> },
    /// Retrieve operation from current phase
    Retrieve { hash: Hash },
    /// Phase transition
    Transition { from: DualPhase, to: DualPhase },
}

/// A dual bootstrap system that maintains two complementary instances
pub struct DualBootstrap {
    /// The primary bootstrap system
    primary: Bootstrap,
    /// The dual bootstrap system
    dual: Bootstrap,
    /// Current phase
    phase: DualPhase,
    /// Operation history
    history: Vec<DualOperation>,
}

impl DualBootstrap {
    /// Creates a new dual bootstrap system
    pub fn new() -> Self {
        Self {
            primary: Bootstrap::new(),
            dual: Bootstrap::new(),
            phase: DualPhase::Primary,
            history: Vec::new(),
        }
    }

    /// Performs a dual store operation
    pub fn dual_store(&mut self, content: Vec<u8>) -> (Hash, Hash) {
        // Store in primary system
        let primary_hash = self.primary.store(content.clone());
        
        // Store in dual system
        let dual_hash = self.dual.store(content);
        
        // Record operation
        self.history.push(DualOperation::Store { content });
        
        // Advance phase
        self.advance_phase();
        
        (primary_hash, dual_hash)
    }

    /// Retrieves from the current phase's system
    pub fn retrieve(&self, hash: &Hash) -> Option<&Artifact> {
        let operation = DualOperation::Retrieve { hash: hash.clone() };
        
        match self.phase {
            DualPhase::Primary => self.primary.retrieve(hash),
            DualPhase::Dual => self.dual.retrieve(hash),
        }
    }

    /// Gets the current phase
    pub fn phase(&self) -> DualPhase {
        self.phase
    }

    /// Gets the combined cycle step
    pub fn combined_cycle(&self) -> u64 {
        self.primary.cycle_step() + self.dual.cycle_step()
    }

    /// Gets the operation history
    pub fn history(&self) -> &[DualOperation] {
        &self.history
    }

    /// Advances to the next phase
    fn advance_phase(&mut self) {
        let from = self.phase;
        self.phase = match self.phase {
            DualPhase::Primary => DualPhase::Dual,
            DualPhase::Dual => DualPhase::Primary,
        };
        
        self.history.push(DualOperation::Transition { from, to: self.phase });
    }

    /// Gets the primary system
    pub fn primary(&self) -> &Bootstrap {
        &self.primary
    }

    /// Gets the dual system
    pub fn dual(&self) -> &Bootstrap {
        &self.dual
    }

    /// Checks if both systems are in sync (same cycle step)
    pub fn is_synchronized(&self) -> bool {
        self.primary.cycle_step() == self.dual.cycle_step()
    }

    /// Forces synchronization by advancing the lagging system
    pub fn synchronize(&mut self) {
        let primary_step = self.primary.cycle_step();
        let dual_step = self.dual.cycle_step();
        
        if primary_step < dual_step {
            // Advance primary to match dual
            for _ in 0..(dual_step - primary_step) {
                self.primary.store(b"sync".to_vec());
            }
        } else if dual_step < primary_step {
            // Advance dual to match primary
            for _ in 0..(primary_step - dual_step) {
                self.dual.store(b"sync".to_vec());
            }
        }
    }
}

impl Default for DualBootstrap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_bootstrap_creation() {
        let dual = DualBootstrap::new();
        assert_eq!(dual.phase(), DualPhase::Primary);
        assert_eq!(dual.combined_cycle(), 0);
        assert!(dual.is_synchronized());
    }

    #[test]
    fn test_dual_store() {
        let mut dual = DualBootstrap::new();
        let content = b"test content".to_vec();
        
        let (primary_hash, dual_hash) = dual.dual_store(content.clone());
        
        // Hashes should be different (different systems)
        assert_ne!(primary_hash, dual_hash);
        
        // Both should be retrievable
        assert!(dual.primary().retrieve(&primary_hash).is_some());
        assert!(dual.dual().retrieve(&dual_hash).is_some());
        
        // Phase should have advanced
        assert_eq!(dual.phase(), DualPhase::Dual);
    }

    #[test]
    fn test_phase_cycling() {
        let mut dual = DualBootstrap::new();
        
        // Start in primary
        assert_eq!(dual.phase(), DualPhase::Primary);
        
        // First operation -> dual
        dual.dual_store(b"test1".to_vec());
        assert_eq!(dual.phase(), DualPhase::Dual);
        
        // Second operation -> primary
        dual.dual_store(b"test2".to_vec());
        assert_eq!(dual.phase(), DualPhase::Primary);
    }

    #[test]
    fn test_synchronization() {
        let mut dual = DualBootstrap::new();
        
        // Store in primary only
        dual.primary.store(b"primary only".to_vec());
        assert!(!dual.is_synchronized());
        
        // Synchronize
        dual.synchronize();
        assert!(dual.is_synchronized());
    }

    #[test]
    fn test_history_tracking() {
        let mut dual = DualBootstrap::new();
        
        dual.dual_store(b"test".to_vec());
        
        let history = dual.history();
        assert_eq!(history.len(), 2); // Store + Transition
        
        if let DualOperation::Store { content } = &history[0] {
            assert_eq!(content, b"test");
        } else {
            panic!("Expected Store operation");
        }
    }
} 