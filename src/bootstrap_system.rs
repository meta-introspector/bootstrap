use solfunmeme_clifford::SolMultivector;
use super::stage_management::{get_all_stages, run_stage0, StageInfo};

/// The main system configuration
#[derive(Debug)]
pub struct BootstrapSystem {
    pub stages: Vec<StageInfo>,
    pub mathematical_foundation: String,
    pub architecture: String,
    pub flow_multivector: SolMultivector,
}

impl BootstrapSystem {
    /// Create a new bootstrap system
    pub fn new() -> Self {
        Self {
            stages: get_all_stages(),
            mathematical_foundation: "OEIS Sequences, Harmonic Lattice, Pharmonic Mapping".to_string(),
            architecture: "OSI Layer System with Nash Equilibrium".to_string(),
            flow_multivector: SolMultivector::default(), // Initialize with a default multivector
        }
    }
    
    /// Run the complete system
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Update flow_multivector for system initialization (e1)
        self.flow_multivector = self.flow_multivector + solfunmeme_clifford::SolMultivector::from_e(1.0, 1);
        println!("Flow Multivector after initialization: {:?}", self.flow_multivector);

        run_stage0(self)
    }
}
