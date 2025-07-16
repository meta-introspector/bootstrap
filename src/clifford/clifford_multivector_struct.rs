//! # Clifford Multivector Struct
//!
//! This module defines the core `CliffordMultivector` struct, which is the
//! central data structure for representing elements in a Clifford algebra.

/// Represents a multivector in a Clifford algebra.
///
/// A multivector is a linear combination of blades of different grades (scalars,
/// vectors, bivectors, etc.). This struct holds the components (coefficients)
/// of the multivector in a vector.
#[derive(Debug, Clone, PartialEq)]
pub struct CliffordMultivector<S> {
    /// The coefficients of the multivector, typically ordered by grade.
    pub coefficients: Vec<S>,
    /// The dimension of the vector space the algebra is built on.
    pub dimension: usize,
    /// Phantom data to hold the scalar type `S`.
    pub _phantom: std::marker::PhantomData<S>,
}