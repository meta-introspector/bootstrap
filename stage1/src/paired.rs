//! Paired systems for dual verification and storage

use bootstrap::{Hash, Hasher, Artifact, Store};
use std::collections::HashMap;

/// A paired hasher that uses two different hash algorithms
pub struct PairedHasher {
    /// Primary hasher
    primary: Box<dyn Hasher>,
    /// Secondary hasher
    secondary: Box<dyn Hasher>,
}

impl PairedHasher {
    /// Creates a new paired hasher
    pub fn new(primary: Box<dyn Hasher>, secondary: Box<dyn Hasher>) -> Self {
        Self { primary, secondary }
    }

    /// Computes both hashes for the given data
    pub fn dual_hash(&self, data: &[u8]) -> (Hash, Hash) {
        let primary_hash = self.primary.hash(data);
        let secondary_hash = self.secondary.hash(data);
        (primary_hash, secondary_hash)
    }

    /// Verifies that both hashes match for the same data
    pub fn verify_dual(&self, data: &[u8], primary_hash: &Hash, secondary_hash: &Hash) -> bool {
        let (computed_primary, computed_secondary) = self.dual_hash(data);
        computed_primary == *primary_hash && computed_secondary == *secondary_hash
    }

    /// Gets the primary hasher
    pub fn primary(&self) -> &dyn Hasher {
        self.primary.as_ref()
    }

    /// Gets the secondary hasher
    pub fn secondary(&self) -> &dyn Hasher {
        self.secondary.as_ref()
    }
}

/// A paired storage system that maintains two storage backends
pub struct PairedStorage {
    /// Primary storage
    primary: Box<dyn Store>,
    /// Secondary storage
    secondary: Box<dyn Store>,
    /// Verification mode
    verification_mode: VerificationMode,
}

/// Modes for storage verification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerificationMode {
    /// Require both storages to succeed
    Strict,
    /// Allow either storage to succeed
    Lenient,
    /// Use primary only, verify with secondary
    PrimaryWithVerification,
}

impl PairedStorage {
    /// Creates a new paired storage system
    pub fn new(primary: Box<dyn Store>, secondary: Box<dyn Store>) -> Self {
        Self {
            primary,
            secondary,
            verification_mode: VerificationMode::Strict,
        }
    }

    /// Sets the verification mode
    pub fn set_verification_mode(&mut self, mode: VerificationMode) {
        self.verification_mode = mode;
    }

    /// Stores an artifact in both storage systems
    pub fn dual_store(&mut self, artifact: Artifact) -> Result<(Hash, Hash), &'static str> {
        let primary_hash = self.primary.store(artifact.clone());
        let secondary_hash = self.secondary.store(artifact);

        match self.verification_mode {
            VerificationMode::Strict => {
                // Both must succeed (they should return the same hash)
                if primary_hash == secondary_hash {
                    Ok((primary_hash, secondary_hash))
                } else {
                    Err("Storage verification failed - hashes don't match")
                }
            }
            VerificationMode::Lenient => {
                // Either can succeed
                Ok((primary_hash, secondary_hash))
            }
            VerificationMode::PrimaryWithVerification => {
                // Primary is authoritative, secondary is for verification
                if primary_hash == secondary_hash {
                    Ok((primary_hash, secondary_hash))
                } else {
                    Err("Secondary storage verification failed")
                }
            }
        }
    }

    /// Retrieves from the primary storage
    pub fn get_primary(&self, hash: &Hash) -> Option<&Artifact> {
        self.primary.get(hash)
    }

    /// Retrieves from the secondary storage
    pub fn get_secondary(&self, hash: &Hash) -> Option<&Artifact> {
        self.secondary.get(hash)
    }

    /// Retrieves from both storages and verifies consistency
    pub fn get_verified(&self, hash: &Hash) -> Result<Option<&Artifact>, &'static str> {
        let primary_artifact = self.primary.get(hash);
        let secondary_artifact = self.secondary.get(hash);

        match (primary_artifact, secondary_artifact) {
            (Some(primary), Some(secondary)) => {
                // Both have the artifact - verify they're the same
                if primary.content == secondary.content {
                    Ok(Some(primary))
                } else {
                    Err("Storage inconsistency detected")
                }
            }
            (Some(primary), None) => {
                // Only primary has it
                match self.verification_mode {
                    VerificationMode::Lenient => Ok(Some(primary)),
                    _ => Err("Secondary storage missing artifact"),
                }
            }
            (None, Some(secondary)) => {
                // Only secondary has it
                match self.verification_mode {
                    VerificationMode::Lenient => Ok(Some(secondary)),
                    _ => Err("Primary storage missing artifact"),
                }
            }
            (None, None) => Ok(None),
        }
    }

    /// Checks if both storages contain the same artifacts
    pub fn is_consistent(&self) -> bool {
        // This is a simplified check - in practice you'd want to compare all artifacts
        self.primary.len() == self.secondary.len()
    }

    /// Gets storage statistics
    pub fn stats(&self) -> PairedStorageStats {
        PairedStorageStats {
            primary_count: self.primary.len(),
            secondary_count: self.secondary.len(),
            verification_mode: self.verification_mode,
            consistent: self.is_consistent(),
        }
    }
}

/// Statistics about paired storage
#[derive(Debug)]
pub struct PairedStorageStats {
    /// Number of artifacts in primary storage
    pub primary_count: usize,
    /// Number of artifacts in secondary storage
    pub secondary_count: usize,
    /// Current verification mode
    pub verification_mode: VerificationMode,
    /// Whether storages are consistent
    pub consistent: bool,
}

/// A paired artifact that maintains dual hashes
#[derive(Debug, Clone)]
pub struct PairedArtifact {
    /// The original artifact
    artifact: Artifact,
    /// Primary hash
    primary_hash: Hash,
    /// Secondary hash
    secondary_hash: Hash,
    /// Whether hashes are verified
    verified: bool,
}

impl PairedArtifact {
    /// Creates a new paired artifact
    pub fn new(artifact: Artifact, primary_hash: Hash, secondary_hash: Hash) -> Self {
        let verified = primary_hash == secondary_hash;
        Self {
            artifact,
            primary_hash,
            secondary_hash,
            verified,
        }
    }

    /// Gets the underlying artifact
    pub fn artifact(&self) -> &Artifact {
        &self.artifact
    }

    /// Gets the primary hash
    pub fn primary_hash(&self) -> &Hash {
        &self.primary_hash
    }

    /// Gets the secondary hash
    pub fn secondary_hash(&self) -> &Hash {
        &self.secondary_hash
    }

    /// Checks if the hashes are verified (match)
    pub fn is_verified(&self) -> bool {
        self.verified
    }

    /// Gets both hashes as a tuple
    pub fn hashes(&self) -> (&Hash, &Hash) {
        (&self.primary_hash, &self.secondary_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bootstrap::{SimpleHasher, ChecksumHasher, MemoryStore};

    #[test]
    fn test_paired_hasher() {
        let paired = PairedHasher::new(
            Box::new(SimpleHasher),
            Box::new(ChecksumHasher),
        );

        let data = b"test data";
        let (primary_hash, secondary_hash) = paired.dual_hash(data);

        // Different hashers should produce different hashes
        assert_ne!(primary_hash, secondary_hash);

        // Verification should pass
        assert!(paired.verify_dual(data, &primary_hash, &secondary_hash));
    }

    #[test]
    fn test_paired_storage_strict() {
        let paired = PairedStorage::new(
            Box::new(MemoryStore::new()),
            Box::new(MemoryStore::new()),
        );

        let artifact = bootstrap::Artifact::new(b"test".to_vec(), &SimpleHasher);
        
        // In strict mode, both storages should work identically
        let result = paired.dual_store(artifact.clone());
        assert!(result.is_ok());
    }

    #[test]
    fn test_paired_artifact() {
        let artifact = bootstrap::Artifact::new(b"test".to_vec(), &SimpleHasher);
        let primary_hash = SimpleHasher.hash(b"test");
        let secondary_hash = ChecksumHasher.hash(b"test");

        let paired = PairedArtifact::new(artifact.clone(), primary_hash.clone(), secondary_hash.clone());

        assert_eq!(paired.artifact().content, artifact.content);
        assert_eq!(paired.primary_hash(), &primary_hash);
        assert_eq!(paired.secondary_hash(), &secondary_hash);
        assert!(!paired.is_verified()); // Different hashers produce different hashes
    }
} 