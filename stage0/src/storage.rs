//! Storage - A potential flow for storage operations
//! This represents the flow of artifacts through storage space
//! Each storage operation is a potential in the storage field

use std::collections::HashMap;
use crate::hash::Hash;
use crate::artifact::Artifact;

/// The Storage potential flow
/// Represents a field of storage potentials where artifacts converge
#[derive(Debug, Clone)]
pub struct Storage {
    field: HashMap<Hash, Artifact>,
}

impl Storage {
    /// Creates a new storage potential field
    pub fn new_field() -> Self {
        Self {
            field: HashMap::new(),
        }
    }

    /// Stores an artifact in the storage field
    pub fn store_flow(&mut self, artifact: Artifact) -> Result<(), StorageFlowError> {
        self.field.insert(artifact.hash.clone(), artifact);
        Ok(())
    }

    /// Retrieves an artifact from the storage field
    pub fn retrieve_flow(&self, hash: &Hash) -> Option<Artifact> {
        self.field.get(hash).cloned()
    }

    /// Computes the curl of the storage field
    pub fn field_curl(&self) -> usize {
        self.field.len()
    }
}

/// Storage flow error potential
#[derive(Debug, thiserror::Error)]
pub enum StorageFlowError {
    #[error("Storage flow operation failed")]
    FlowFailed,
    #[error("Hash potential not found in field")]
    NotFound,
}

/// The storage flow operator
/// Transforms storage operations into storage potentials
pub fn storage_flow() -> Storage {
    Storage::new_field()
} 