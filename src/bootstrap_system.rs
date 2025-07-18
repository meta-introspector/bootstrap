use solfunmeme_clifford::SolMultivector;
use super::stage_management::{run_stage0, StageInfo};
use super::prime_vibe_ontology::PrimeVibeOntology;
use anyhow::Result;

/// The main system configuration for the bootstrap process.
/// This struct holds the overall state and metadata of the running system.
#[derive(Debug)]
pub struct BootstrapSystem {
    /// Information about all 42 stages of the system.
    pub stages: Vec<StageInfo>,
    /// A description of the mathematical foundations underpinning the system.
    pub mathematical_foundation: String,
    /// A description of the system's architecture.
    pub architecture: String,
    /// A `SolMultivector` representing the current flow and state of the system.
    /// This multivector evolves as the system progresses through its stages and operations.
    pub flow_multivector: SolMultivector,
    /// The loaded prime vibe ontology.
    pub prime_vibe_ontology: PrimeVibeOntology,
}

impl BootstrapSystem {
    /// Create a new bootstrap system
    pub fn new() -> Result<Self> {
        Ok(Self {
            stages: super::stage_management::get_all_stages(&PrimeVibeOntology::new()?),
            mathematical_foundation: "OEIS Sequences, Harmonic Lattice, Pharmonic Mapping".to_string(),
            architecture: "OSI Layer System with Nash Equilibrium".to_string(),
            flow_multivector: SolMultivector::default(), // Initialize with a default multivector
            prime_vibe_ontology: PrimeVibeOntology::new()?,
        })
    }
    
    /// Run the complete system
    pub fn run(&mut self) -> Result<()> {
        // Update flow_multivector for system initialization (e1)
        self.flow_multivector = self.flow_multivector.clone() + solfunmeme_clifford::SolMultivector::from_indexed_iter([(1 << (1 - 1), 1.0)].into_iter()).unwrap();
        println!("Flow Multivector after initialization: {:?}", self.flow_multivector);

        run_stage0(self)
    }
}
