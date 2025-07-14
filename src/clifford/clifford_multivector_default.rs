use super::clifford_multivector_struct::CliffordMultivector;

impl<S: Default + Clone + PartialEq> Default for CliffordMultivector<S> {
    fn default() -> Self {
        Self::new(3) // Default 3D Clifford algebra
    }
} 