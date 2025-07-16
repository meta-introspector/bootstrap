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
    // This would return information about all 42 stages
    vec![]
}

/// Information about a single stage
#[derive(Debug, Clone)]
pub struct StageInfo {
    pub number: u32,
    pub name: String,
    pub description: String,
    pub oeis_sequences: Vec<String>,
    pub is_prime: bool,
    pub is_fibonacci: bool,
    pub is_factor_of_42: bool,
    pub prime_factors: Vec<u32>,
}
