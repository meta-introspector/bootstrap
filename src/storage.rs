//! Storage backends for artifacts

use crate::artifact::Artifact;
use crate::hash::Hash;
use std::collections::HashMap;

/// Trait for artifact storage backends
pub trait Store {
    /// Stores an artifact and returns its hash
    fn store(&mut self, artifact: Artifact) -> Hash;
    
    /// Retrieves an artifact by hash
    fn get(&self, hash: &Hash) -> Option<&Artifact>;
    
    /// Checks if an artifact exists
    fn contains(&self, hash: &Hash) -> bool;
    
    /// Returns the number of stored artifacts
    fn len(&self) -> usize;
    
    /// Returns true if the store is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A simple in-memory storage implementation
///
/// This is the most basic storage backend, suitable for bootstrap
/// and testing. All data is lost when the process terminates.
#[derive(Debug, Default)]
pub struct MemoryStore {
    artifacts: HashMap<Hash, Artifact>,
}

impl MemoryStore {
    /// Creates a new empty memory store
    pub fn new() -> Self {
        Self {
            artifacts: HashMap::new(),
        }
    }

    /// Returns an iterator over all stored artifacts
    pub fn iter(&self) -> impl Iterator<Item = (&Hash, &Artifact)> {
        self.artifacts.iter()
    }
}

impl Store for MemoryStore {
    fn store(&mut self, artifact: Artifact) -> Hash {
        let hash = artifact.hash.clone();
        self.artifacts.insert(hash.clone(), artifact);
        hash
    }

    fn get(&self, hash: &Hash) -> Option<&Artifact> {
        self.artifacts.get(hash)
    }

    fn contains(&self, hash: &Hash) -> bool {
        self.artifacts.contains_key(hash)
    }

    fn len(&self) -> usize {
        self.artifacts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::SimpleHasher;

    #[test]
    fn test_memory_store_creation() {
        let store = MemoryStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_store_and_retrieve() {
        let mut store = MemoryStore::new();
        let hasher = SimpleHasher;
        let content = b"test content".to_vec();
        let artifact = Artifact::new(content.clone(), &hasher);

        let hash = store.store(artifact);
        
        assert!(store.contains(&hash));
        assert_eq!(store.len(), 1);
        
        let retrieved = store.get(&hash);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, content);
    }

    #[test]
    fn test_store_iteration() {
        let mut store = MemoryStore::new();
        let hasher = SimpleHasher;
        
        let artifact1 = Artifact::new(b"content1".to_vec(), &hasher);
        let artifact2 = Artifact::new(b"content2".to_vec(), &hasher);
        
        store.store(artifact1);
        store.store(artifact2);
        
        let artifacts: Vec<_> = store.iter().collect();
        assert_eq!(artifacts.len(), 2);
    }
} 