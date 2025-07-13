//!
//! # Bootstrap Stage0 - Flow-Based Microkernel
//!
//! ## Philosophy
//!
//! Bootstrap Stage0 is a minimal, self-contained microkernel constructed as **flows** where each type is a **potential flow** in a Navier-Stokes equation. Each module is a **vector field**, each function is a **flow operation**, and the system is a **higher-order potential flow**.
//!
//! - **Each type = a potential flow** in mathematical space
//! - **Each file = a vector field** with defined operations
//! - **Each function = a flow operation** that transforms data
//! - **Each module = a flow component** with specific responsibilities
//! - **The system = a higher-order potential flow** that coordinates all sub-flows
//!
//! ## Mathematical Foundations
//!
//! The system is built on fluid dynamics principles:
//! - **Gradient**: Rate of change in potential
//! - **Divergence**: Flow source/sink strength
//! - **Curl**: Vorticity of the flow field
//! - **Potential**: Scalar field from which flow derives
//!
//! ### 42-Step Cycle
//! The system operates on a 42-step cycle, representing:
//! - **Flow periodicity**: System returns to initial state every 42 operations
//! - **Potential cycling**: Each operation advances the cycle potential
//! - **Field evolution**: Flow field evolves through 42 distinct states
//!
//! ## Architecture
//!
//! ### File Structure
//! ```
//! src/
//! ├── hash.rs         // Hash potential flow
//! ├── artifact.rs     // Artifact potential flow  
//! ├── storage.rs      // Storage potential flow
//! ├── kernel.rs       // Kernel potential flow
//! ├── system.rs       // System potential flow
//! └── lib.rs          // Unified flow field
//! ```
//!
//! ## Flow Types
//!
//! ### 1. Hash Flow (`hash.rs`)
//! **Purpose**: Content identification through hash space
//!
//! - **Flow Field**: Hash space where content converges
//! - **Flow Operator**: `hash_flow(data) -> Hash`
//! - **Mathematical**: Represents a potential in content identification field
//! - **Properties**: Gradient, flow field, potential convergence
//!
//! ### 2. Artifact Flow (`artifact.rs`)
//! **Purpose**: Content storage and materialization
//!
//! - **Flow Field**: Storage space where content materializes
//! - **Flow Operator**: `artifact_flow(content) -> Artifact`
//! - **Mathematical**: Represents a potential in content storage field
//! - **Properties**: Divergence, content flow, materialization
//!
//! ### 3. Storage Flow (`storage.rs`)
//! **Purpose**: Storage field operations
//!
//! - **Flow Field**: Field of storage potentials where artifacts converge
//! - **Flow Operator**: `storage_flow() -> Storage`
//! - **Mathematical**: Represents a field of storage potentials
//! - **Properties**: Curl, field operations, potential mapping
//!
//! ### 4. Kernel Flow (`kernel.rs`)
//! **Purpose**: System coordination and cycle management
//!
//! - **Flow Field**: Central coordination field where all flows converge
//! - **Flow Operator**: `kernel_flow() -> Kernel`
//! - **Mathematical**: Represents the central coordination potential
//! - **Properties**: System divergence, cycle management, flow coordination
//!
//! ### 5. System Flow (`system.rs`)
//! **Purpose**: Complete bootstrap system as unified flow field
//!
//! - **Flow Field**: Unified flow field where all potentials converge
//! - **Flow Operator**: `system_flow() -> System`
//! - **Mathematical**: Higher-order potential that coordinates all sub-flows
//! - **Properties**: System curl, total divergence, unified coordination
//!
//! ## API Reference
//!
//! ### Main Interface
//!
//! ```rust
//! pub struct Bootstrap {
//!     system: System,
//! }
//!
//! impl Bootstrap {
//!     pub fn new() -> Self
//!     pub fn store(&mut self, content: Vec<u8>) -> Hash
//!     pub fn retrieve(&self, hash: &Hash) -> Option<Artifact>
//!     pub fn cycle_step(&self) -> u64
//!     pub fn total_divergence(&self) -> usize
//!     pub fn curl(&self) -> u64
//! }
//! ```
//!
//! ### Usage Example
//!
//! ```rust
//! use bootstrap::Bootstrap;
//!
//! let mut bootstrap = Bootstrap::new();
//! let content = b"Hello, Flow World!".to_vec();
//! let hash = bootstrap.store(content);
//! let retrieved = bootstrap.retrieve(&hash);
//! let divergence = bootstrap.total_divergence();
//! let curl = bootstrap.curl();
//! let cycle = bootstrap.cycle_step();
//! ```
//!
//! ## Mathematical Properties
//!
//! - **Hash**: Potential in content identification field
//! - **Artifact**: Potential in content storage field
//! - **Storage**: Field of storage potentials
//! - **Kernel**: Central coordination potential
//! - **System**: Higher-order potential flow
//!
//! ### Flow Properties
//! - **Gradient**: Rate of change in potential (Hash)
//! - **Divergence**: Flow source/sink strength (Artifact, Kernel, System)
//! - **Curl**: Vorticity of the flow field (Storage, System)
//! - **Potential**: Scalar field from which flow derives (all types)
//!
//! ## Design Principles
//!
//! - **Minimalism**: Zero dependencies, only essential functionality
//! - **Flow-Based Design**: Mathematical consistency, composable operations
//! - **Content-Addressable Storage**: All data identified by content hash
//! - **Extensibility**: Pluggable components, modular flow architecture
//!
//! ## Testing
//!
//! Run `cargo test` to verify flow behavior and mathematical consistency.
//!
//! ## License
//!
//! MIT License - see LICENSE file for details.

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