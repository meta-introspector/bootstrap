use super::bootstrap_system::BootstrapSystem;
use solfunmeme_clifford::SolMultivector;
use super::main01;
use super::godel;


/// Run the complete stage0 process
/// 
/// This function generates rustdoc, runs all stages, and combines the output
/// into a comprehensive HTML file that documents the entire system.
pub fn run_stage0(system: &mut BootstrapSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Call main01 and pass the system
    main01::main01(system);

    // Update flow_multivector for a Gödel operation (e4)
    godel::update_flow_for_godel_operation(system);

    // Update flow_multivector for function execution (e3)
    system.flow_multivector = system.flow_multivector.clone() + SolMultivector::from_indexed_iter([(1 << (3 - 1), 1.0)].into_iter()).unwrap();
    println!("Flow Multivector after run_stage0 execution: {:?}", system.flow_multivector);

    Ok(())
}

/// Get information about all 42 stages
/// 
/// Returns a vector of stage information including their mathematical properties,
/// OEIS sequences, and relationships to other stages.
pub fn get_all_stages() -> Vec<StageInfo> {
    let mut stages = Vec::new();
    for i in 1..=42 {
        let osi_layer = match i {
            1 => Some(1), // Physical Layer
            2 => Some(2), // Data Link Layer
            3 => Some(3), // Network Layer
            4 => Some(4), // Transport Layer
            5 => Some(5), // Session Layer
            6 => Some(6), // Presentation Layer
            7 => Some(7), // Application Layer
            _ => None,
        };

        let mut vibe_tags = Vec::new();
        if i % 2 == 0 { vibe_tags.push("even".to_string()); }
        if i % 3 == 0 { vibe_tags.push("divisible_by_3".to_string()); }
        if i % 5 == 0 { vibe_tags.push("divisible_by_5".to_string()); }
        if (i as u32).is_power_of_two() { vibe_tags.push("power_of_two".to_string()); }
        // Add more complex logic for other tags based on prime factors, etc.

        stages.push(StageInfo {
            number: i,
            name: format!("Stage {}", i),
            description: format!("Description for Stage {}", i),
            oeis_sequences: Vec::new(), // Placeholder
            is_prime: is_prime(i),
            is_fibonacci: is_fibonacci(i),
            is_factor_of_42: 42 % i == 0,
            prime_factors: get_prime_factors(i),
            osi_layer,
            vibe_tags,
        });
    }
    stages
}

fn is_prime(n: u32) -> bool {
    if n <= 1 { return false; }
    for i in 2..=((n as f32).sqrt() as u32) {
        if n % i == 0 { return false; }
    }
    true
}

fn is_fibonacci(n: u32) -> bool {
    let mut a = 0;
    let mut b = 1;
    while a < n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a == n
}

fn get_prime_factors(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut num = n;
    for i in 2..=num {
        while num % i == 0 {
            factors.push(i);
            num /= i;
        }
    }
    factors
}

/// Information about a single stage
#[derive(Debug, Clone)]
pub struct StageInfo {
    /// The stage number.
    pub number: u32,
    /// The name of the stage.
    pub name: String,
    /// A description of the stage.
    pub description: String,
    /// OEIS sequences associated with the stage number.
    pub oeis_sequences: Vec<String>,
    /// True if the stage number is prime.
    pub is_prime: bool,
    /// True if the stage number is a Fibonacci number.
    pub is_fibonacci: bool,
    /// True if the stage number is a factor of 42.
    pub is_factor_of_42: bool,
    /// The prime factors of the stage number.
    pub prime_factors: Vec<u32>,
    /// The OSI layer associated with the stage, if applicable.
    pub osi_layer: Option<u8>,
    /// Tags describing the vibe or conceptual role of the stage.
    pub vibe_tags: Vec<String>,
}
