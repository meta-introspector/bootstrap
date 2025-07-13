//! Bootstrap Stage0 - Flow-Based Microkernel
//! 
//! A system constructed as flows where each type is a potential flow
//! in a Navier-Stokes equation. Each module is a vector field,
//! each function is a flow operation, and the system is a higher-order
//! potential flow.

pub mod hash;
pub mod artifact;
pub mod storage;
pub mod kernel;
pub mod system;

// Re-export the main flow types
pub use hash::{Hash, hash_flow};
pub use artifact::{Artifact, artifact_flow};
pub use storage::{Storage, storage_flow, StorageFlowError};
pub use kernel::{Kernel, kernel_flow};
pub use system::{System, system_flow, bootstrap_flow};

/// The Bootstrap type - the main entry point into the flow system
/// This represents the complete flow field where all potentials converge
pub struct Bootstrap {
    system: System,
}

impl Bootstrap {
    /// Creates a new bootstrap flow field
    pub fn new() -> Self {
        Self {
            system: bootstrap_flow(),
        }
    }

    /// Stores content in the bootstrap flow
    pub fn store(&mut self, content: Vec<u8>) -> Hash {
        self.system.store_system_flow(content)
    }

    /// Retrieves content from the bootstrap flow
    pub fn retrieve(&self, hash: &Hash) -> Option<Artifact> {
        self.system.retrieve_system_flow(hash)
    }

    /// Gets the current cycle potential
    pub fn cycle_step(&self) -> u64 {
        self.system.system_cycle_potential()
    }

    /// Computes the total system divergence
    pub fn total_divergence(&self) -> usize {
        self.system.total_system_divergence()
    }

    /// Computes the system curl (vorticity)
    pub fn curl(&self) -> u64 {
        self.system.system_curl()
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
    fn test_bootstrap_flow_creation() {
        let bootstrap = Bootstrap::new();
        assert_eq!(bootstrap.cycle_step(), 0);
    }

    #[test]
    fn test_flow_store_and_retrieve() {
        let mut bootstrap = Bootstrap::new();
        let content = b"Hello, Flow World!".to_vec();
        
        let hash = bootstrap.store(content.clone());
        let retrieved = bootstrap.retrieve(&hash);
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content_flow(), content.as_slice());
    }

    #[test]
    fn test_flow_cycle_advancement() {
        let mut bootstrap = Bootstrap::new();
        let initial_step = bootstrap.cycle_step();
        
        // Store something to advance cycle flow
        bootstrap.store(b"test flow".to_vec());
        
        assert_eq!(bootstrap.cycle_step(), (initial_step + 1) % 42);
    }

    #[test]
    fn test_flow_divergence() {
        let mut bootstrap = Bootstrap::new();
        assert_eq!(bootstrap.total_divergence(), 0);
        
        bootstrap.store(b"content 1".to_vec());
        bootstrap.store(b"content 2".to_vec());
        
        assert_eq!(bootstrap.total_divergence(), 2);
    }
} 