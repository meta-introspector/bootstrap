use super::clifford_multivector_struct::CliffordMultivector;
use super::clifford_dyn_trait::CliffordDyn;

impl<S: Default + Clone + PartialEq> CliffordDyn for CliffordMultivector<S> {
    type Scalar = S;
    
    fn dimension(&self) -> usize {
        self.dimension
    }
    
    fn grade(&self) -> usize {
        // Simplified - in practice this would compute the actual grade
        0
    }
    
    fn norm(&self) -> Self::Scalar {
        // Simplified - in practice this would compute the actual norm
        S::default()
    }
    
    fn coefficients(&self) -> Vec<Self::Scalar> {
        self.coefficients.clone()
    }
    
    fn from_coefficients(dimension: usize, coeffs: Vec<Self::Scalar>) -> Self {
        Self::from_coefficients(dimension, coeffs)
    }
    
    fn add(&self, other: &Self) -> Self {
        // Simplified - in practice this would add coefficients
        self.clone()
    }
    
    fn multiply(&self, other: &Self) -> Self {
        // Simplified - in practice this would multiply using geometric product
        self.clone()
    }
    
    fn scalar_part(&self) -> Self::Scalar {
        if !self.coefficients.is_empty() {
            self.coefficients[0].clone()
        } else {
            S::default()
        }
    }
    
    fn vector_part(&self) -> Vec<Self::Scalar> {
        if self.coefficients.len() > 1 {
            self.coefficients[1..=self.dimension].to_vec()
        } else {
            vec![]
        }
    }
    
    fn bivector_part(&self) -> Vec<Self::Scalar> {
        let start = self.dimension + 1;
        let end = start + (self.dimension * (self.dimension - 1)) / 2;
        if self.coefficients.len() > start {
            self.coefficients[start..end.min(self.coefficients.len())].to_vec()
        } else {
            vec![]
        }
    }
} 