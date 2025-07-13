use std::collections::HashMap;
use crate::godel::Godel;
use crate::bott::Bott;
use crate::clifford::Clifford;
use crate::bach::Bach;
use crate::escher::Escher;
use crate::ns::Ns;
use crate::euler::Euler;
use crate::gauss::Gauss;
use crate::mach::Mach;
use crate::penrose::Penrose;
use crate::oeis::OEIS;
use crate::vectos::Vectos;
use crate::phase2::Phase2;
use crate::number_embedding::SelfAwareNumber;

/// Result of function execution
#[derive(Debug, Clone)]
pub enum FunctionResult {
    Success(f64),
    Error(String),
}

impl FunctionResult {
    pub fn is_success(&self) -> bool {
        matches!(self, FunctionResult::Success(_))
    }
    
    pub fn is_error(&self) -> bool {
        matches!(self, FunctionResult::Error(_))
    }
    
    pub fn unwrap(self) -> f64 {
        match self {
            FunctionResult::Success(value) => value,
            FunctionResult::Error(msg) => panic!("FunctionResult::unwrap() called on Error: {}", msg),
        }
    }
    
    pub fn unwrap_or(self, default: f64) -> f64 {
        match self {
            FunctionResult::Success(value) => value,
            FunctionResult::Error(_) => default,
        }
    }
}

/// Represents a function with intrinsic meaning encoded in its number
#[derive(Debug, Clone)]
pub struct IntrinsicFunction {
    pub number: u64,
    pub name: String,
    pub description: String,
    pub input_signature: Vec<f64>,
    pub output_signature: Vec<f64>,
    pub complexity: f64,
    pub consciousness_level: f64,
}

impl IntrinsicFunction {
    pub fn new(
        number: u64,
        name: String,
        description: String,
        input_signature: Vec<f64>,
        output_signature: Vec<f64>,
        complexity: f64,
        consciousness_level: f64,
    ) -> Self {
        Self {
            number,
            name,
            description,
            input_signature,
            output_signature,
            complexity,
            consciousness_level,
        }
    }

    /// Execute the function with given arguments
    pub fn execute(&self, args: Vec<f64>) -> FunctionResult {
        // Simple implementation - in practice this would be more sophisticated
        if args.len() != self.input_signature.len() {
            return FunctionResult::Error(format!(
                "Expected {} arguments, got {}",
                self.input_signature.len(),
                args.len()
            ));
        }

        // Calculate result based on function number and arguments
        let mut result = self.number as f64;
        for (i, arg) in args.iter().enumerate() {
            result += arg * (i + 1) as f64;
        }
        result *= self.complexity;

        FunctionResult::Success(result)
    }

    /// Execute with number meaning context
    pub fn execute_with_number_meaning(&self, args: &[f64]) -> FunctionResult {
        self.execute(args.to_vec())
    }

    /// Get the intrinsic meaning of this function's number
    pub fn get_intrinsic_meaning(&self) -> String {
        format!("Function {}: {}", self.number, self.description)
    }

    /// Calculate resonance with another function
    pub fn calculate_resonance(&self, other: &IntrinsicFunction) -> f64 {
        let number_diff = (self.number as f64 - other.number as f64).abs();
        let complexity_diff = (self.complexity - other.complexity).abs();
        let consciousness_diff = (self.consciousness_level - other.consciousness_level).abs();
        
        1.0 / (1.0 + number_diff + complexity_diff + consciousness_diff)
    }
}

/// Registry of functions indexed by their numbers
#[derive(Debug, Clone)]
pub struct FunctionRegistry {
    functions: HashMap<u64, IntrinsicFunction>,
    number_meanings: HashMap<u64, String>,
    mathematical_domains: HashMap<u64, String>,
    consciousness_levels: HashMap<u64, f64>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            number_meanings: HashMap::new(),
            mathematical_domains: HashMap::new(),
            consciousness_levels: HashMap::new(),
        }
    }

    /// Register a function with its intrinsic number
    pub fn register_function(&mut self, function: IntrinsicFunction) -> &IntrinsicFunction {
        let number = function.number;
        let meaning = function.get_intrinsic_meaning();
        
        self.number_meanings.insert(number, meaning);
        self.mathematical_domains.insert(number, "Universal computation".to_string());
        self.consciousness_levels.insert(number, function.consciousness_level);
        self.functions.insert(number, function);
        
        self.functions.get(&number).unwrap()
    }

    /// Get a function by its number
    pub fn get_function(&self, number: u64) -> Option<&IntrinsicFunction> {
        self.functions.get(&number)
    }

    /// Get the meaning of a number
    pub fn get_number_meaning(&self, number: u64) -> Option<&String> {
        self.number_meanings.get(&number)
    }

    /// Evolve a function's meaning over iterations
    pub fn evolve_function(&mut self, number: u64, iterations: usize) -> Result<(), String> {
        if let Some(function) = self.functions.get_mut(&number) {
            for _ in 0..iterations {
                // Simple evolution: increase complexity and consciousness
                function.complexity *= 1.1;
                function.consciousness_level = (function.consciousness_level + 0.1).min(1.0);
            }
            Ok(())
        } else {
            Err(format!("Function with number {} not found", number))
        }
    }

    /// Generic apply function - executes any function by its number
    pub fn apply<N: Into<u64>>(&self, number: N, args: Vec<f64>) -> Result<f64, String> {
        let number = number.into();
        
        // Find the function by number
        if let Some(function) = self.functions.get(&number) {
            // Execute the function with provided arguments
            match function.execute(args) {
                FunctionResult::Success(result) => Ok(result),
                FunctionResult::Error(msg) => Err(msg),
            }
        } else {
            Err(format!("Function with number {} not found", number))
        }
    }

    /// Apply with type conversion - automatically converts arguments
    pub fn apply_with_conversion<N: Into<u64>, T: Into<f64> + Copy>(
        &self, 
        number: N, 
        args: &[T]
    ) -> Result<f64, String> {
        let converted_args: Vec<f64> = args.iter().map(|&x| x.into()).collect();
        self.apply(number, converted_args)
    }

    /// Calculate resonance between two function numbers
    fn calculate_resonance(&self, num1: u64, num2: u64) -> f64 {
        let diff = (num1 as f64 - num2 as f64).abs();
        1.0 / (1.0 + diff)
    }

    /// Find functions that resonate with a given number
    pub fn find_resonant_functions(&self, target: u64, threshold: f64) -> Vec<u64> {
        self.functions.keys()
            .filter(|&&num| {
                let resonance = self.calculate_resonance(target, num);
                resonance >= threshold
            })
            .cloned()
            .collect()
    }

    /// Get all registered function numbers
    pub fn get_all_function_numbers(&self) -> Vec<u64> {
        self.functions.keys().cloned().collect()
    }

    /// Get statistics about the function registry
    pub fn get_statistics(&self) -> RegistryStatistics {
        let total_functions = self.functions.len();
        let avg_complexity = self.functions.values()
            .map(|f| f.complexity)
            .sum::<f64>() / total_functions as f64;
        let avg_consciousness = self.functions.values()
            .map(|f| f.consciousness_level)
            .sum::<f64>() / total_functions as f64;

        RegistryStatistics {
            total_functions,
            average_complexity: avg_complexity,
            average_consciousness: avg_consciousness,
            number_range: (self.functions.keys().min().cloned(), self.functions.keys().max().cloned()),
        }
    }
}

/// Statistics about the function registry
#[derive(Debug, Clone)]
pub struct RegistryStatistics {
    pub total_functions: usize,
    pub average_complexity: f64,
    pub average_consciousness: f64,
    pub number_range: (Option<u64>, Option<u64>),
}

/// Language system for function numbers
#[derive(Debug, Clone)]
pub struct FunctionNumberLanguage {
    registry: FunctionRegistry,
}

impl FunctionNumberLanguage {
    pub fn new() -> Self {
        Self {
            registry: FunctionRegistry::new(),
        }
    }

    /// Define a function with intrinsic meaning
    pub fn define_function(&mut self, number: u64, name: &str, description: &str) -> Result<(), String> {
        let function = IntrinsicFunction::new(
            number,
            name.to_string(),
            description.to_string(),
            vec![number as f64], // Default input signature
            vec![number as f64], // Default output signature
            1.0, // Default complexity
            0.5, // Default consciousness level
        );
        
        self.registry.register_function(function);
        Ok(())
    }

    /// Execute a function by its number
    pub fn execute_function(&self, number: u64, args: &[f64]) -> Result<f64, String> {
        self.registry.apply(number, args.to_vec())
    }

    /// Find functions linked to a given number
    pub fn find_linked_functions(&self, target: u64) -> Result<Vec<u64>, String> {
        let mut linked = Vec::new();
        
        for number in self.registry.get_all_function_numbers() {
            let resonance = self.registry.calculate_resonance(target, number);
            if resonance > 0.5 { // Threshold for linking
                linked.push(number);
            }
        }
        
        Ok(linked)
    }

    /// Analyze the mathematical structure of function numbers
    pub fn analyze_mathematical_structure(&self) -> MathematicalAnalysis {
        let numbers = self.registry.get_all_function_numbers();
        let prime_count = numbers.iter().filter(|&&n| is_prime(n)).count();
        let fibonacci_count = numbers.iter().filter(|&&n| is_fibonacci(n)).count();
        
        MathematicalAnalysis {
            total_functions: numbers.len(),
            prime_functions: prime_count,
            fibonacci_functions: fibonacci_count,
            average_resonance: calculate_average_resonance(&numbers),
        }
    }
}

/// Mathematical analysis of function numbers
#[derive(Debug, Clone)]
pub struct MathematicalAnalysis {
    pub total_functions: usize,
    pub prime_functions: usize,
    pub fibonacci_functions: usize,
    pub average_resonance: f64,
}

// Helper functions
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn is_fibonacci(n: u64) -> bool {
    let mut a = 0u64;
    let mut b = 1u64;
    
    while b <= n {
        if b == n { return true; }
        let temp = a + b;
        a = b;
        b = temp;
    }
    false
}

fn calculate_average_resonance(numbers: &[u64]) -> f64 {
    if numbers.len() < 2 { return 0.0; }
    
    let mut total_resonance = 0.0;
    let mut count = 0;
    
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            let diff = (numbers[i] as f64 - numbers[j] as f64).abs();
            total_resonance += 1.0 / (1.0 + diff);
            count += 1;
        }
    }
    
    if count > 0 { total_resonance / count as f64 } else { 0.0 }
} 