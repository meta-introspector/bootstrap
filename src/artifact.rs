//! Content-addressable data units

use crate::hash::{Hash, Hasher};

/// A content-addressable data unit
///
/// Artifacts are immutable data units whose identity is determined
/// by their content through cryptographic hashing.
#[derive(Debug, Clone, PartialEq)]
pub struct Artifact {
    /// The raw binary content
    pub content: Vec<u8>,
    /// The hash that identifies this content
    pub hash: Hash,
}

impl Artifact {
    /// Creates a new artifact from content using the provided hasher
    pub fn new(content: Vec<u8>, hasher: &dyn Hasher) -> Self {
        let hash = hasher.hash(&content);
        Self { content, hash }
    }

    /// Returns the content as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.content
    }

    /// Returns the hash that identifies this artifact
    pub fn id(&self) -> &Hash {
        &self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::SimpleHasher;

    #[test]
    fn test_artifact_creation() {
        let hasher = SimpleHasher;
        let content = b"test content".to_vec();
        let artifact = Artifact::new(content.clone(), &hasher);

        assert_eq!(artifact.content, content);
        assert_eq!(artifact.hash, hasher.hash(&content));
    }

    #[test]
    fn test_artifact_identity() {
        let hasher = SimpleHasher;
        let content1 = b"same content".to_vec();
        let content2 = b"same content".to_vec();
        let content3 = b"different content".to_vec();

        let artifact1 = Artifact::new(content1, &hasher);
        let artifact2 = Artifact::new(content2, &hasher);
        let artifact3 = Artifact::new(content3, &hasher);

        // Same content should have same hash
        assert_eq!(artifact1.hash, artifact2.hash);
        
        // Different content should have different hash
        assert_ne!(artifact1.hash, artifact3.hash);
    }
} 