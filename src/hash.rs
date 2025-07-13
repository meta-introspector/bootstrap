//! Hash functions and content identification

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher as StdHasher};

/// A hash value that identifies content
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hash {
    /// The hash bytes
    pub bytes: Vec<u8>,
}

impl Hash {
    /// Creates a new hash from bytes
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Creates a hash from a u64 value
    pub fn from_u64(value: u64) -> Self {
        Self {
            bytes: value.to_be_bytes().to_vec(),
        }
    }

    /// Returns the hash as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Trait for hash functions
pub trait Hasher {
    /// Computes the hash of the given data
    fn hash(&self, data: &[u8]) -> Hash;
}

/// A simple hash implementation using Rust's default hasher
///
/// This is a basic, non-cryptographic hash suitable for bootstrap.
/// For production use, consider implementing more secure hash functions.
#[derive(Debug, Clone)]
pub struct SimpleHasher;

impl Hasher for SimpleHasher {
    fn hash(&self, data: &[u8]) -> Hash {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        Hash::from_u64(hasher.finish())
    }
}

/// A checksum-based hash for very simple content identification
///
/// This is the most basic hash implementation, suitable only for
/// basic integrity checking and deduplication.
#[derive(Debug, Clone)]
pub struct ChecksumHasher;

impl Hasher for ChecksumHasher {
    fn hash(&self, data: &[u8]) -> Hash {
        let checksum = data.iter().fold(0u64, |acc, &byte| {
            acc.wrapping_add(byte as u64)
        });
        Hash::from_u64(checksum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_creation() {
        let hash = Hash::new(vec![1, 2, 3, 4]);
        assert_eq!(hash.bytes, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_hash_from_u64() {
        let value = 0x1234567890abcdef;
        let hash = Hash::from_u64(value);
        assert_eq!(hash.bytes, value.to_be_bytes().to_vec());
    }

    #[test]
    fn test_simple_hasher() {
        let hasher = SimpleHasher;
        let data1 = b"test data";
        let data2 = b"test data";
        let data3 = b"different data";

        let hash1 = hasher.hash(data1);
        let hash2 = hasher.hash(data2);
        let hash3 = hasher.hash(data3);

        // Same data should have same hash
        assert_eq!(hash1, hash2);
        
        // Different data should have different hash
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_checksum_hasher() {
        let hasher = ChecksumHasher;
        let data1 = b"abc";
        let data2 = b"cba"; // Same bytes, different order

        let hash1 = hasher.hash(data1);
        let hash2 = hasher.hash(data2);

        // Checksum should be the same for same bytes
        assert_eq!(hash1, hash2);
    }
} 