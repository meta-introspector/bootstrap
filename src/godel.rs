/// Trait for types that can be assigned Gödel numbers
/// 
/// Gödel numbering is a way to encode mathematical objects (like formulas, 
/// expressions, or in our case, emoji stages) as natural numbers using 
/// prime factorization.
pub trait Godel {
    /// Get the Gödel number for this item
    /// Each item should have a unique prime number as its Gödel number
    fn godel_number(&self) -> u64;
    
    /// Create an item from its Gödel number
    /// Returns None if the number doesn't correspond to a known item
    fn from_godel_number(n: u64) -> Option<Self> where Self: Sized;
    
    /// Compose multiple items into a single Gödel number
    /// Uses prime factorization: item1^1 * item2^1 * item3^1 * ...
    fn compose_godel(items: &[Self]) -> u64 where Self: Sized {
        items.iter()
            .map(|item| item.godel_number())
            .product()
    }
    
    /// Decompose a Gödel number back into items
    /// Returns the list of items that compose this number
    fn decompose_godel(n: u64) -> Vec<Self> where Self: Sized {
        let mut items = Vec::new();
        let mut remaining = n;
        
        // Get all known Gödel numbers, sorted in descending order
        let mut godel_numbers: Vec<u64> = Self::all_known_godel_numbers();
        godel_numbers.sort_by(|a, b| b.cmp(a)); // Sort descending
        
        // Try to factor out each Gödel number
        for &godel_num in &godel_numbers {
            while remaining % godel_num == 0 {
                if let Some(item) = Self::from_godel_number(godel_num) {
                    items.push(item);
                }
                remaining /= godel_num;
            }
        }
        
        items
    }
    
    /// Get all known Gödel numbers for this type
    /// This is used for decomposition
    fn all_known_godel_numbers() -> Vec<u64> where Self: Sized;
    
    /// Check if a number is a valid Gödel number for this type
    fn is_valid_godel_number(n: u64) -> bool where Self: Sized {
        Self::from_godel_number(n).is_some()
    }
    
    /// Get the prime factors of a Gödel number
    /// Returns a vector of (prime, exponent) pairs
    fn prime_factors(n: u64) -> Vec<(u64, u32)> {
        let mut factors = Vec::new();
        let mut remaining = n;
        let mut divisor = 2;
        
        while remaining > 1 {
            let mut exponent = 0;
            while remaining % divisor == 0 {
                exponent += 1;
                remaining /= divisor;
            }
            if exponent > 0 {
                factors.push((divisor, exponent));
            }
            divisor += 1;
        }
        
        factors
    }
    
    /// Check if a Gödel number represents a composition of multiple items
    fn is_composition(n: u64) -> bool where Self: Sized {
        let factors = Self::prime_factors(n);
        factors.iter().any(|(_, exp)| *exp > 1) || factors.len() > 1
    }
    
    /// Get the number of items in a composition
    fn composition_length(n: u64) -> usize where Self: Sized {
        let factors = Self::prime_factors(n);
        factors.iter().map(|(_, exp)| *exp as usize).sum()
    }
}

/// Extension trait for additional Gödel operations
pub trait GodelExt: Godel {
    /// Encode a sequence of items as a single number
    /// This is useful for transmitting or storing complex expressions
    fn encode_sequence(items: &[Self]) -> u64 where Self: Sized {
        Self::compose_godel(items)
    }
    
    /// Decode a sequence from a number
    fn decode_sequence(n: u64) -> Vec<Self> where Self: Sized {
        Self::decompose_godel(n)
    }
    
    /// Check if two Gödel numbers represent the same composition
    fn godel_equivalent(a: u64, b: u64) -> bool where Self: Sized {
        Self::decompose_godel(a) == Self::decompose_godel(b)
    }
    
    /// Get a human-readable representation of a Gödel number
    fn godel_to_string(n: u64) -> String where Self: Sized {
        let items = Self::decompose_godel(n);
        if items.is_empty() {
            format!("Gödel({})", n)
        } else {
            let item_strings: Vec<String> = items.iter()
                .map(|item| format!("{:?}", item))
                .collect();
            format!("Gödel({}) = [{}]", n, item_strings.join(", "))
        }
    }
}

// Implement GodelExt for all types that implement Godel
impl<T: Godel> GodelExt for T {}

/// Helper struct for working with Gödel numbers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GodelNumber<T: Godel> {
    pub value: u64,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Godel> GodelNumber<T> {
    /// Create a new Gödel number
    pub fn new(value: u64) -> Self {
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Create from an item
    pub fn from_item(item: &T) -> Self {
        Self::new(item.godel_number())
    }
    
    /// Create from multiple items
    pub fn from_items(items: &[T]) -> Self {
        Self::new(T::compose_godel(items))
    }
    
    /// Decompose into items
    pub fn decompose(&self) -> Vec<T> {
        T::decompose_godel(self.value)
    }
    
    /// Check if this represents a single item
    pub fn is_single(&self) -> bool {
        !T::is_composition(self.value)
    }
    
    /// Check if this represents multiple items
    pub fn is_composition(&self) -> bool {
        T::is_composition(self.value)
    }
    
    /// Get the number of items in this composition
    pub fn length(&self) -> usize {
        T::composition_length(self.value)
    }
    
    /// Get a string representation
    pub fn to_string(&self) -> String {
        T::godel_to_string(self.value)
    }
}

impl<T: Godel> From<u64> for GodelNumber<T> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl<T: Godel> From<&T> for GodelNumber<T> {
    fn from(item: &T) -> Self {
        Self::from_item(item)
    }
}

impl<T: Godel> std::fmt::Display for GodelNumber<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
} 