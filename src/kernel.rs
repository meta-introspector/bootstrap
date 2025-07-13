//! The core kernel that orchestrates the bootstrap system

use crate::artifact::Artifact;
use crate::hash::{Hash, Hasher};
use crate::storage::Store;

/// The core kernel that manages artifacts and system state
///
/// The kernel maintains a 42-step cycle that advances with each
/// operation, providing a sense of system evolution and renewal.
pub struct Kernel {
    /// Current step in the 42-step cycle (0-41)
    cycle_step: u64,
    /// The hasher used for content identification
    hasher: Box<dyn Hasher>,
    /// The storage backend for artifacts
    store: Box<dyn Store>,
}

impl Kernel {
    /// Creates a new kernel with the specified components
    pub fn new(hasher: Box<dyn Hasher>, store: Box<dyn Store>) -> Self {
        Self {
            cycle_step: 0,
            hasher,
            store,
        }
    }

    /// Stores content and returns its hash
    ///
    /// This operation advances the cycle by one step.
    pub fn store_artifact(&mut self, content: Vec<u8>) -> Hash {
        let artifact = Artifact::new(content, self.hasher.as_ref());
        let hash = self.store.store(artifact);
        self.advance_cycle();
        hash
    }

    /// Retrieves an artifact by hash
    pub fn get_artifact(&self, hash: &Hash) -> Option<&Artifact> {
        self.store.get(hash)
    }

    /// Checks if an artifact exists
    pub fn has_artifact(&self, hash: &Hash) -> bool {
        self.store.contains(hash)
    }

    /// Returns the current cycle step (0-41)
    pub fn cycle_step(&self) -> u64 {
        self.cycle_step
    }

    /// Returns the number of stored artifacts
    pub fn artifact_count(&self) -> usize {
        self.store.len()
    }

    /// Advances the cycle by one step, wrapping at 42
    fn advance_cycle(&mut self) {
        self.cycle_step = (self.cycle_step + 1) % 42;
    }

    /// Replaces the hasher component
    ///
    /// This operation advances the cycle by one step.
    pub fn replace_hasher(&mut self, new_hasher: Box<dyn Hasher>) {
        self.hasher = new_hasher;
        self.advance_cycle();
    }

    /// Replaces the storage backend
    ///
    /// This operation advances the cycle by one step.
    pub fn replace_store(&mut self, new_store: Box<dyn Store>) {
        self.store = new_store;
        self.advance_cycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::SimpleHasher;
    use crate::storage::MemoryStore;

    fn create_test_kernel() -> Kernel {
        Kernel::new(
            Box::new(SimpleHasher),
            Box::new(MemoryStore::new()),
        )
    }

    #[test]
    fn test_kernel_creation() {
        let kernel = create_test_kernel();
        assert_eq!(kernel.cycle_step(), 0);
        assert_eq!(kernel.artifact_count(), 0);
    }

    #[test]
    fn test_store_and_retrieve() {
        let mut kernel = create_test_kernel();
        let content = b"test content".to_vec();
        
        let hash = kernel.store_artifact(content.clone());
        
        assert!(kernel.has_artifact(&hash));
        assert_eq!(kernel.artifact_count(), 1);
        
        let retrieved = kernel.get_artifact(&hash);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, content);
    }

    #[test]
    fn test_cycle_advancement() {
        let mut kernel = create_test_kernel();
        assert_eq!(kernel.cycle_step(), 0);
        
        // Store something to advance cycle
        kernel.store_artifact(b"test".to_vec());
        assert_eq!(kernel.cycle_step(), 1);
        
        // Store more to advance further
        kernel.store_artifact(b"test2".to_vec());
        assert_eq!(kernel.cycle_step(), 2);
    }

    #[test]
    fn test_cycle_wrapping() {
        let mut kernel = create_test_kernel();
        
        // Advance to step 41
        for _ in 0..41 {
            kernel.store_artifact(b"test".to_vec());
        }
        assert_eq!(kernel.cycle_step(), 41);
        
        // Next operation should wrap to 0
        kernel.store_artifact(b"test".to_vec());
        assert_eq!(kernel.cycle_step(), 0);
    }

    #[test]
    fn test_component_replacement() {
        let mut kernel = create_test_kernel();
        assert_eq!(kernel.cycle_step(), 0);
        
        // Replace hasher
        kernel.replace_hasher(Box::new(SimpleHasher));
        assert_eq!(kernel.cycle_step(), 1);
        
        // Replace store
        kernel.replace_store(Box::new(MemoryStore::new()));
        assert_eq!(kernel.cycle_step(), 2);
    }
} 