//! System - A potential flow for the complete bootstrap system
//! This represents the unified flow field where all potentials converge
//! The system is a higher-order potential flow in Navier-Stokes space

use crate::hash::Hash;
use crate::artifact::Artifact;
use crate::kernel::{Kernel, kernel_flow};

/// The System potential flow
/// Represents the complete bootstrap system as a unified flow field
/// This is the higher-order potential that coordinates all sub-flows
#[derive(Debug)]
pub struct System {
    kernel: Kernel,
}

impl System {
    /// Creates a new system potential field
    pub fn new_field() -> Self {
        Self {
            kernel: kernel_flow(),
        }
    }

    /// Stores content in the system flow field
    pub fn store_system_flow(&mut self, content: Vec<u8>) -> Hash {
        self.kernel.store_flow(content)
    }

    /// Retrieves content from the system flow field
    pub fn retrieve_system_flow(&self, hash: &Hash) -> Option<Artifact> {
        self.kernel.retrieve_flow(hash)
    }

    /// Gets the current system cycle potential
    pub fn system_cycle_potential(&self) -> u64 {
        self.kernel.cycle_potential()
    }

    /// Computes the total system flow divergence
    pub fn total_system_divergence(&self) -> usize {
        self.kernel.system_divergence()
    }

    /// Computes the system flow curl (vorticity)
    pub fn system_curl(&self) -> u64 {
        self.kernel.cycle_potential()
    }
}

/// The system flow operator
/// Transforms system operations into unified system potentials
pub fn system_flow() -> System {
    System::new_field()
}

/// The bootstrap flow operator
/// The entry point into the complete flow field
pub fn bootstrap_flow() -> System {
    system_flow()
} 