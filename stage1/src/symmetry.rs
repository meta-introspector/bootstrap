//! Symmetry cycles and kernel operations

use bootstrap::{Bootstrap, Hash, Artifact};
use std::collections::HashMap;

/// A symmetry cycle that operates in dual phases
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SymmetryCycle {
    /// Current step in the symmetry cycle (0-83, representing dual 42-step cycles)
    step: u64,
    /// Current phase (0 = primary, 1 = dual)
    phase: u8,
}

impl SymmetryCycle {
    /// Creates a new symmetry cycle
    pub fn new() -> Self {
        Self { step: 0, phase: 0 }
    }

    /// Advances the cycle by one step
    pub fn advance(&mut self) {
        self.step = (self.step + 1) % 84; // 2 * 42 = 84 total steps
        self.phase = (self.step / 42) as u8; // 0 for first 42 steps, 1 for second 42 steps
    }

    /// Gets the current step
    pub fn step(&self) -> u64 {
        self.step
    }

    /// Gets the current phase (0 = primary, 1 = dual)
    pub fn phase(&self) -> u8 {
        self.phase
    }

    /// Gets the relative step within the current phase (0-41)
    pub fn phase_step(&self) -> u64 {
        self.step % 42
    }

    /// Checks if we're in the primary phase
    pub fn is_primary(&self) -> bool {
        self.phase == 0
    }

    /// Checks if we're in the dual phase
    pub fn is_dual(&self) -> bool {
        self.phase == 1
    }

    /// Gets the symmetry ratio (balance between phases)
    pub fn symmetry_ratio(&self) -> f64 {
        if self.step == 0 {
            0.5 // Perfect balance at start
        } else {
            let primary_steps = if self.phase == 0 { self.phase_step() } else { 42 };
            let dual_steps = if self.phase == 1 { self.phase_step() } else { 0 };
            primary_steps as f64 / (primary_steps + dual_steps) as f64
        }
    }
}

impl Default for SymmetryCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// A symmetry kernel that maintains dual bootstrap systems
pub struct SymmetryKernel {
    /// Primary bootstrap system
    primary: Bootstrap,
    /// Dual bootstrap system
    dual: Bootstrap,
    /// Symmetry cycle
    cycle: SymmetryCycle,
    /// Symmetry operations history
    history: Vec<SymmetryOperation>,
}

/// Types of symmetry operations
#[derive(Debug, Clone)]
pub enum SymmetryOperation {
    /// Store operation in current phase
    Store { content: Vec<u8>, phase: u8 },
    /// Retrieve operation from current phase
    Retrieve { hash: Hash, phase: u8 },
    /// Phase transition
    PhaseTransition { from: u8, to: u8 },
    /// Symmetry balance operation
    Balance { ratio: f64 },
}

impl SymmetryKernel {
    /// Creates a new symmetry kernel
    pub fn new() -> Self {
        Self {
            primary: Bootstrap::new(),
            dual: Bootstrap::new(),
            cycle: SymmetryCycle::new(),
            history: Vec::new(),
        }
    }

    /// Stores content in the current phase's system
    pub fn store(&mut self, content: Vec<u8>) -> Hash {
        let phase = self.cycle.phase();
        let hash = match phase {
            0 => self.primary.store(content.clone()),
            1 => self.dual.store(content.clone()),
            _ => panic!("Invalid phase"),
        };

        self.history.push(SymmetryOperation::Store { content, phase });
        self.cycle.advance();
        hash
    }

    /// Retrieves content from the current phase's system
    pub fn retrieve(&self, hash: &Hash) -> Option<&Artifact> {
        let phase = self.cycle.phase();
        match phase {
            0 => self.primary.retrieve(hash),
            1 => self.dual.retrieve(hash),
            _ => None,
        }
    }

    /// Performs a dual store operation (stores in both systems)
    pub fn dual_store(&mut self, content: Vec<u8>) -> (Hash, Hash) {
        let primary_hash = self.primary.store(content.clone());
        let dual_hash = self.dual.store(content.clone());

        // Record operations for both phases
        self.history.push(SymmetryOperation::Store { content: content.clone(), phase: 0 });
        self.history.push(SymmetryOperation::Store { content, phase: 1 });

        self.cycle.advance();
        (primary_hash, dual_hash)
    }

    /// Gets the current cycle
    pub fn cycle(&self) -> SymmetryCycle {
        self.cycle
    }

    /// Gets the symmetry ratio
    pub fn symmetry_ratio(&self) -> f64 {
        self.cycle.symmetry_ratio()
    }

    /// Gets the operation history
    pub fn history(&self) -> &[SymmetryOperation] {
        &self.history
    }

    /// Gets the primary system
    pub fn primary(&self) -> &Bootstrap {
        &self.primary
    }

    /// Gets the dual system
    pub fn dual(&self) -> &Bootstrap {
        &self.dual
    }

    /// Checks if both systems are balanced
    pub fn is_balanced(&self) -> bool {
        let ratio = self.symmetry_ratio();
        (ratio - 0.5).abs() < 0.1 // Within 10% of perfect balance
    }

    /// Forces balance by advancing the lagging system
    pub fn balance(&mut self) {
        let primary_step = self.primary.cycle_step();
        let dual_step = self.dual.cycle_step();

        if primary_step < dual_step {
            // Advance primary to match dual
            for _ in 0..(dual_step - primary_step) {
                self.primary.store(b"balance".to_vec());
            }
        } else if dual_step < primary_step {
            // Advance dual to match primary
            for _ in 0..(primary_step - dual_step) {
                self.dual.store(b"balance".to_vec());
            }
        }

        let ratio = self.symmetry_ratio();
        self.history.push(SymmetryOperation::Balance { ratio });
    }

    /// Gets symmetry statistics
    pub fn stats(&self) -> SymmetryStats {
        SymmetryStats {
            total_operations: self.history.len(),
            primary_operations: self.history.iter().filter(|op| matches!(op, SymmetryOperation::Store { phase: 0, .. } | SymmetryOperation::Retrieve { phase: 0, .. })).count(),
            dual_operations: self.history.iter().filter(|op| matches!(op, SymmetryOperation::Store { phase: 1, .. } | SymmetryOperation::Retrieve { phase: 1, .. })).count(),
            phase_transitions: self.history.iter().filter(|op| matches!(op, SymmetryOperation::PhaseTransition { .. })).count(),
            symmetry_ratio: self.symmetry_ratio(),
            balanced: self.is_balanced(),
        }
    }
}

impl Default for SymmetryKernel {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about symmetry operations
#[derive(Debug)]
pub struct SymmetryStats {
    /// Total number of operations
    pub total_operations: usize,
    /// Number of primary phase operations
    pub primary_operations: usize,
    /// Number of dual phase operations
    pub dual_operations: usize,
    /// Number of phase transitions
    pub phase_transitions: usize,
    /// Current symmetry ratio
    pub symmetry_ratio: f64,
    /// Whether systems are balanced
    pub balanced: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry_cycle() {
        let mut cycle = SymmetryCycle::new();
        assert_eq!(cycle.step(), 0);
        assert_eq!(cycle.phase(), 0);
        assert!(cycle.is_primary());
        assert!(!cycle.is_dual());

        // Advance to step 41 (still in primary phase)
        for _ in 0..41 {
            cycle.advance();
        }
        assert_eq!(cycle.phase_step(), 41);
        assert!(cycle.is_primary());

        // Advance to step 42 (enters dual phase)
        cycle.advance();
        assert_eq!(cycle.phase(), 1);
        assert!(cycle.is_dual());
        assert_eq!(cycle.phase_step(), 0);
    }

    #[test]
    fn test_symmetry_kernel() {
        let mut kernel = SymmetryKernel::new();
        assert_eq!(kernel.cycle().step(), 0);
        assert!(kernel.cycle().is_primary());

        // Store in primary phase
        let hash = kernel.store(b"test".to_vec());
        assert_eq!(kernel.cycle().step(), 1);
        assert!(kernel.cycle().is_primary());

        // Retrieve from primary phase
        let retrieved = kernel.retrieve(&hash);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_dual_store() {
        let mut kernel = SymmetryKernel::new();
        let (primary_hash, dual_hash) = kernel.dual_store(b"dual test".to_vec());

        // Both hashes should be different (different systems)
        assert_ne!(primary_hash, dual_hash);

        // Both should be retrievable from their respective systems
        assert!(kernel.primary().retrieve(&primary_hash).is_some());
        assert!(kernel.dual().retrieve(&dual_hash).is_some());
    }

    #[test]
    fn test_balance() {
        let mut kernel = SymmetryKernel::new();
        
        // Store in primary only
        kernel.primary.store(b"primary only".to_vec());
        assert!(!kernel.is_balanced());

        // Balance the systems
        kernel.balance();
        assert!(kernel.is_balanced());
    }

    #[test]
    fn test_symmetry_stats() {
        let mut kernel = SymmetryKernel::new();
        
        kernel.store(b"test1".to_vec());
        kernel.store(b"test2".to_vec());
        kernel.dual_store(b"dual test".to_vec());

        let stats = kernel.stats();
        assert_eq!(stats.total_operations, 4); // 2 single stores + 2 dual stores
        assert_eq!(stats.primary_operations, 3); // 2 single + 1 dual
        assert_eq!(stats.dual_operations, 1); // 1 dual
    }
} 