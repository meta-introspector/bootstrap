//! # Clifford Algebra Module
//!
//! This module provides a comprehensive framework for working with Clifford Algebra,
//! a powerful mathematical system that generalizes complex numbers and quaternions.
//! It is used within the bootstrap system to model geometric and quantum-like
//! relationships between different stages and concepts.
//!
//! ## Core Components
//!
//! - **`Clifford` Trait**: Defines the fundamental operations for any Clifford algebra object.
//! - **`CliffordMultivector`**: A struct representing a multivector, the central element
//!   in Clifford algebra.
//! - **`CliffordGodel` Trait**: Integrates Clifford algebra concepts with the project's
//!   Gödel numbering system, allowing algebraic structures to be encoded as unique numbers.
//! - **Dynamic Trait (`CliffordDyn`)**: A `dyn`-safe version for use with trait objects.

pub mod clifford_trait;
pub mod clifford_godel_trait;
pub mod clifford_multivector_struct;
pub mod clifford_multivector_constructors;
pub mod clifford_multivector_impl;
pub mod clifford_multivector_default;
pub mod clifford_dyn_trait;
pub mod clifford_multivector_dyn_impl;

pub use clifford_trait::Clifford;
pub use clifford_godel_trait::CliffordGodel;
pub use clifford_multivector_struct::CliffordMultivector;
pub use clifford_dyn_trait::CliffordDyn;