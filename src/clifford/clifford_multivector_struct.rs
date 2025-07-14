#[derive(Debug, Clone, PartialEq)]
pub struct CliffordMultivector<S> {
    pub coefficients: Vec<S>,
    pub dimension: usize,
    pub _phantom: std::marker::PhantomData<S>,
} 