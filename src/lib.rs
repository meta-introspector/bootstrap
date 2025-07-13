//! # Bootstrap - Minimal Self-Contained Microkernel
//!
//! A truly minimal, dependency-free bootstrap system that provides the essential
//! building blocks for content-addressable storage and dynamic extension.
//!
//! ## Core Principles
//!
//! - **Zero Dependencies**: No external crates, pure Rust std/core
//! - **Self-Contained**: Complete functionality within this crate
//! - **Content-Addressable**: All data identified by content hash
//! - **Extensible**: Pluggable components via traits
//! - **Minimal**: Only essential functionality for bootstrap

pub mod artifact;
pub mod hash;
pub mod kernel;
pub mod storage;

pub use artifact::Artifact;
pub use hash::{Hash, Hasher, SimpleHasher};
pub use kernel::Kernel;
pub use storage::MemoryStore;

/// The core bootstrap system
pub struct Bootstrap {
    kernel: Kernel,
}

impl Bootstrap {
    /// Creates a new bootstrap system with default components
    pub fn new() -> Self {
        Self {
            kernel: Kernel::new(
                Box::new(SimpleHasher),
                Box::new(MemoryStore::new()),
            )
        }
    }

    /// Stores content and returns its hash
    pub fn store(&mut self, content: Vec<u8>) -> Hash {
        self.kernel.store_artifact(content)
    }

    /// Retrieves content by hash
    pub fn retrieve(&self, hash: &Hash) -> Option<&Artifact> {
        self.kernel.get_artifact(hash)
    }

    /// Gets the current cycle step (0-41)
    pub fn cycle_step(&self) -> u64 {
        self.kernel.cycle_step()
    }
}

impl Default for Bootstrap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bootstrap_creation() {
        let bootstrap = Bootstrap::new();
        assert_eq!(bootstrap.cycle_step(), 0);
    }

    #[test]
    fn test_store_and_retrieve() {
        let mut bootstrap = Bootstrap::new();
        let content = b"Hello, Bootstrap!".to_vec();
        
        let hash = bootstrap.store(content.clone());
        let retrieved = bootstrap.retrieve(&hash);
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, content);
    }

    #[test]
    fn test_cycle_advancement() {
        let mut bootstrap = Bootstrap::new();
        let initial_step = bootstrap.cycle_step();
        
        // Store something to advance cycle
        bootstrap.store(b"test".to_vec());
        
        assert_eq!(bootstrap.cycle_step(), (initial_step + 1) % 42);
    }
} 