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