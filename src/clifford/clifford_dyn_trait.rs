/// Dyn-compatible interface for Clifford operations
/// This trait provides a simplified interface that can be used with trait objects
pub trait CliffordDyn {
    type Scalar;
    
    /// Get the dimension of the underlying vector space
    fn dimension(&self) -> usize;
    
    /// Get the grade of this multivector
    fn grade(&self) -> usize;
    
    /// Get the norm of this multivector
    fn norm(&self) -> Self::Scalar;
    
    /// Get the coefficients as a vector
    fn coefficients(&self) -> Vec<Self::Scalar>;
    
    /// Create a new Clifford multivector from coefficients
    fn from_coefficients(dimension: usize, coeffs: Vec<Self::Scalar>) -> Self where Self: Sized;
    
    /// Add two multivectors
    fn add(&self, other: &Self) -> Self where Self: Sized;
    
    /// Multiply two multivectors
    fn multiply(&self, other: &Self) -> Self where Self: Sized;
    
    /// Get the scalar part
    fn scalar_part(&self) -> Self::Scalar;
    
    /// Get the vector part
    fn vector_part(&self) -> Vec<Self::Scalar>;
    
    /// Get the bivector part
    fn bivector_part(&self) -> Vec<Self::Scalar>;
} 