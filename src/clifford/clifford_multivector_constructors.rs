use super::clifford_multivector_struct::CliffordMultivector;

impl<S: Default + Clone + PartialEq> CliffordMultivector<S> {
    /// Create a new Clifford multivector with the given dimension
    pub fn new(dimension: usize) -> Self {
        let num_coefficients = 1 << dimension; // 2^dimension
        let coefficients = vec![S::default(); num_coefficients];
        Self {
            coefficients,
            dimension,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Create a Clifford multivector from coefficients
    pub fn from_coefficients(dimension: usize, coeffs: Vec<S>) -> Self {
        let expected_size = 1 << dimension;
        let mut coefficients = coeffs;
        if coefficients.len() < expected_size {
            coefficients.extend(std::iter::repeat(S::default()).take(expected_size - coefficients.len()));
        } else if coefficients.len() > expected_size {
            coefficients.truncate(expected_size);
        }
        
        Self {
            coefficients,
            dimension,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Get a coefficient at a specific index
    pub fn get_coefficient(&self, index: usize) -> Option<&S> {
        self.coefficients.get(index)
    }
    
    /// Set a coefficient at a specific index
    pub fn set_coefficient(&mut self, index: usize, value: S) -> bool {
        if index < self.coefficients.len() {
            self.coefficients[index] = value;
            true
        } else {
            false
        }
    }
    
    /// Get all coefficients as a slice
    pub fn all_coefficients(&self) -> &[S] {
        &self.coefficients
    }
    
    /// Get the number of coefficients
    pub fn num_coefficients(&self) -> usize {
        self.coefficients.len()
    }
    
    /// Check if this multivector is zero
    pub fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|c| *c == S::default())
    }
} 