//! Hash - A potential flow for content identification
//! This represents the flow of content through hash space
//! Each hash is a potential in the content identification field

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher as StdHasher};

/// The Hash potential flow
/// Represents a point in hash space where content converges
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Creates a new hash potential from flow field
    pub fn from_flow(data: &[u8]) -> Self {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let flow_value = hasher.finish();
        Self(flow_value.to_be_bytes().into())
    }

    /// Returns the flow field at this potential
    pub fn flow_field(&self) -> &[u8; 32] {
        &self.0
    }

    /// Computes the gradient of this hash potential
    pub fn gradient(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// The hash flow operator
/// Transforms content flow into hash potential
pub fn hash_flow(data: &[u8]) -> Hash {
    Hash::from_flow(data)
} 