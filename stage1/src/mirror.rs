//! Mirror structures and symmetric data organization

use bootstrap::{Artifact, Hash, Hasher};
use std::collections::HashMap;

/// A mirror artifact that maintains symmetric relationships
#[derive(Debug, Clone)]
pub struct MirrorArtifact {
    /// The original artifact
    original: Artifact,
    /// The mirrored artifact (reflected content)
    mirror: Artifact,
    /// Symmetric metadata
    symmetry_metadata: SymmetryMetadata,
}

/// Metadata about the symmetry relationship
#[derive(Debug, Clone)]
pub struct SymmetryMetadata {
    /// Whether this is a perfect mirror (identical content)
    perfect_mirror: bool,
    /// Symmetry axis or transformation applied
    symmetry_type: SymmetryType,
    /// Relationship strength (0.0 to 1.0)
    relationship_strength: f64,
}

/// Types of symmetry transformations
#[derive(Debug, Clone, PartialEq)]
pub enum SymmetryType {
    /// Perfect reflection (identical content)
    Perfect,
    /// Byte reversal
    ByteReversed,
    /// Bit complement
    BitComplement,
    /// Custom transformation
    Custom(String),
}

impl MirrorArtifact {
    /// Creates a new mirror artifact
    pub fn new(original: Artifact, mirror: Artifact, symmetry_type: SymmetryType) -> Self {
        let perfect_mirror = original.content == mirror.content;
        let relationship_strength = if perfect_mirror { 1.0 } else { 0.5 };
        
        Self {
            original,
            mirror,
            symmetry_metadata: SymmetryMetadata {
                perfect_mirror,
                symmetry_type,
                relationship_strength,
            },
        }
    }

    /// Creates a perfect mirror (identical content)
    pub fn perfect_mirror(original: Artifact) -> Self {
        let mirror = Artifact::new(original.content.clone(), &bootstrap::SimpleHasher);
        Self::new(original, mirror, SymmetryType::Perfect)
    }

    /// Creates a byte-reversed mirror
    pub fn byte_reversed_mirror(original: Artifact) -> Self {
        let reversed_content: Vec<u8> = original.content.iter().rev().cloned().collect();
        let mirror = Artifact::new(reversed_content, &bootstrap::SimpleHasher);
        Self::new(original, mirror, SymmetryType::ByteReversed)
    }

    /// Creates a bit-complement mirror
    pub fn bit_complement_mirror(original: Artifact) -> Self {
        let complemented_content: Vec<u8> = original.content.iter().map(|&b| !b).collect();
        let mirror = Artifact::new(complemented_content, &bootstrap::SimpleHasher);
        Self::new(original, mirror, SymmetryType::BitComplement)
    }

    /// Gets the original artifact
    pub fn original(&self) -> &Artifact {
        &self.original
    }

    /// Gets the mirrored artifact
    pub fn mirror(&self) -> &Artifact {
        &self.mirror
    }

    /// Gets the symmetry metadata
    pub fn symmetry_metadata(&self) -> &SymmetryMetadata {
        &self.symmetry_metadata
    }

    /// Checks if this is a perfect mirror
    pub fn is_perfect_mirror(&self) -> bool {
        self.symmetry_metadata.perfect_mirror
    }

    /// Gets the relationship strength
    pub fn relationship_strength(&self) -> f64 {
        self.symmetry_metadata.relationship_strength
    }

    /// Gets both hashes as a tuple
    pub fn hashes(&self) -> (&Hash, &Hash) {
        (&self.original.hash, &self.mirror.hash)
    }
}

/// A storage system that maintains mirror relationships
pub struct MirrorStore {
    /// Primary storage mapping
    primary: HashMap<Hash, MirrorArtifact>,
    /// Mirror storage mapping (reverse lookup)
    mirror: HashMap<Hash, Hash>,
    /// Symmetry statistics
    stats: MirrorStats,
}

/// Statistics about mirror relationships
#[derive(Debug, Default)]
pub struct MirrorStats {
    /// Total number of mirror artifacts
    total_mirrors: usize,
    /// Number of perfect mirrors
    perfect_mirrors: usize,
    /// Number of byte-reversed mirrors
    byte_reversed: usize,
    /// Number of bit-complement mirrors
    bit_complement: usize,
}

impl MirrorStore {
    /// Creates a new mirror store
    pub fn new() -> Self {
        Self {
            primary: HashMap::new(),
            mirror: HashMap::new(),
            stats: MirrorStats::default(),
        }
    }

    /// Stores a mirror artifact
    pub fn store_mirror(&mut self, mirror_artifact: MirrorArtifact) {
        let (original_hash, mirror_hash) = mirror_artifact.hashes();
        
        // Store in primary mapping
        self.primary.insert(original_hash.clone(), mirror_artifact.clone());
        
        // Store reverse mapping
        self.mirror.insert(mirror_hash.clone(), original_hash.clone());
        
        // Update statistics
        self.update_stats(&mirror_artifact);
    }

    /// Retrieves a mirror artifact by hash
    pub fn get_mirror(&self, hash: &Hash) -> Option<&MirrorArtifact> {
        // Try direct lookup
        if let Some(mirror) = self.primary.get(hash) {
            return Some(mirror);
        }
        
        // Try reverse lookup
        if let Some(original_hash) = self.mirror.get(hash) {
            return self.primary.get(original_hash);
        }
        
        None
    }

    /// Gets the mirror hash for a given hash
    pub fn get_mirror_hash(&self, hash: &Hash) -> Option<&Hash> {
        if let Some(mirror) = self.primary.get(hash) {
            return Some(&mirror.mirror.hash);
        }
        
        if let Some(original_hash) = self.mirror.get(hash) {
            return Some(original_hash);
        }
        
        None
    }

    /// Checks if a hash has a mirror
    pub fn has_mirror(&self, hash: &Hash) -> bool {
        self.primary.contains_key(hash) || self.mirror.contains_key(hash)
    }

    /// Gets mirror statistics
    pub fn stats(&self) -> &MirrorStats {
        &self.stats
    }

    /// Updates statistics when storing a mirror
    fn update_stats(&mut self, mirror: &MirrorArtifact) {
        self.stats.total_mirrors += 1;
        
        match mirror.symmetry_metadata.symmetry_type {
            SymmetryType::Perfect => self.stats.perfect_mirrors += 1,
            SymmetryType::ByteReversed => self.stats.byte_reversed += 1,
            SymmetryType::BitComplement => self.stats.bit_complement += 1,
            SymmetryType::Custom(_) => {}, // Custom types don't affect stats
        }
    }

    /// Gets all mirror artifacts
    pub fn all_mirrors(&self) -> impl Iterator<Item = &MirrorArtifact> {
        self.primary.values()
    }

    /// Finds mirrors by symmetry type
    pub fn find_by_symmetry_type(&self, symmetry_type: &SymmetryType) -> Vec<&MirrorArtifact> {
        self.primary
            .values()
            .filter(|mirror| mirror.symmetry_metadata.symmetry_type == *symmetry_type)
            .collect()
    }
}

impl Default for MirrorStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_mirror_creation() {
        let original = bootstrap::Artifact::new(b"test".to_vec(), &bootstrap::SimpleHasher);
        let mirror_artifact = MirrorArtifact::perfect_mirror(original.clone());
        
        assert!(mirror_artifact.is_perfect_mirror());
        assert_eq!(mirror_artifact.relationship_strength(), 1.0);
        assert_eq!(mirror_artifact.original().content, mirror_artifact.mirror().content);
    }

    #[test]
    fn test_byte_reversed_mirror() {
        let original = bootstrap::Artifact::new(b"hello".to_vec(), &bootstrap::SimpleHasher);
        let mirror_artifact = MirrorArtifact::byte_reversed_mirror(original.clone());
        
        assert!(!mirror_artifact.is_perfect_mirror());
        assert_eq!(mirror_artifact.original().content, b"hello");
        assert_eq!(mirror_artifact.mirror().content, b"olleh");
    }

    #[test]
    fn test_mirror_store() {
        let mut store = MirrorStore::new();
        let original = bootstrap::Artifact::new(b"test".to_vec(), &bootstrap::SimpleHasher);
        let mirror_artifact = MirrorArtifact::perfect_mirror(original);
        
        store.store_mirror(mirror_artifact);
        
        assert_eq!(store.stats().total_mirrors, 1);
        assert_eq!(store.stats().perfect_mirrors, 1);
        assert!(store.has_mirror(&original.hash));
    }

    #[test]
    fn test_mirror_lookup() {
        let mut store = MirrorStore::new();
        let original = bootstrap::Artifact::new(b"test".to_vec(), &bootstrap::SimpleHasher);
        let mirror_artifact = MirrorArtifact::perfect_mirror(original.clone());
        
        store.store_mirror(mirror_artifact.clone());
        
        // Lookup by original hash
        let found = store.get_mirror(&original.hash);
        assert!(found.is_some());
        
        // Lookup by mirror hash
        let found = store.get_mirror(&mirror_artifact.mirror.hash);
        assert!(found.is_some());
    }
} 