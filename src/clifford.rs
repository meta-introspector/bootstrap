//! # Clifford Algebra Module
//!
//! This module provides a comprehensive framework for working with Clifford Algebra,
//! a powerful mathematical system that generalizes complex numbers and quaternions.
//! It is used within the bootstrap system to model geometric and quantum-like
//! relationships between different stages and concepts.
//!
//! ## Core Components
//!
//! - **`SolMultivector`**: A struct representing a multivector from `solfunmeme_clifford`.
//! - **`BertCliffordEncoder`**: Encoder to map BERT embeddings to Clifford algebra representation.

pub use solfunmeme_clifford::{SolMultivector, BertCliffordEncoder, SolCl, BertConfig as CliffordBertConfig};
