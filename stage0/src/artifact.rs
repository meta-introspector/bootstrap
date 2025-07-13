//! Artifact - A potential flow for content storage
//! This represents the flow of content through storage space
//! Each artifact is a potential in the content storage field

use crate::hash::Hash;

/// The Artifact potential flow
/// Represents a point in storage space where content materializes
#[derive(Debug, Clone)]
pub struct Artifact {
    pub hash: Hash,
    pub content: Vec<u8>,
}

impl Artifact {
    /// Creates a new artifact potential from content flow
    pub fn from_content_flow(content: Vec<u8>) -> Self {
        let hash = crate::hash::hash_flow(&content);
        Self { hash, content }
    }

    /// Returns the content flow at this potential
    pub fn content_flow(&self) -> &[u8] {
        &self.content
    }

    /// Computes the divergence of this artifact potential
    pub fn divergence(&self) -> usize {
        self.content.len()
    }
}

/// The artifact flow operator
/// Transforms content into artifact potential
pub fn artifact_flow(content: Vec<u8>) -> Artifact {
    Artifact::from_content_flow(content)
} 