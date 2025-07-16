//! # Number Embedding and Self-Aware Numbers
//!
//! This module introduces the concept of "self-aware numbers," a cornerstone of the
//! bootstrap system's philosophy. It defines a framework where a number is not just
//! a value, but a rich object that contains within itself entire instances of all
//! the other mathematical and artistic engines (`Godel`, `Bach`, `Escher`, etc.).
//! This creates a deeply recursive and self-referential system where each number
//! can reflect on its own properties across multiple conceptual domains.
//!
//! ## Core Components
//!
//! - **`EmbeddedNumber` Trait**: The core interface for a self-aware number, defining
//!   methods for accessing its value, its various mathematical encodings (Gödel,
//!   Bott, Clifford), and its embedded trait objects.
//! - **`SelfAwareNumber` Struct**: A concrete implementation of `EmbeddedNumber`,
//!   representing a number that is conscious of its own mathematical identity.
//! - **`NumberEmbeddingSystem` Struct**: A system for managing a collection of
//!   `SelfAwareNumber`s, allowing them to interact, evolve, and synthesize
//!   entire `MathematicalUniverse`s.
//! - **Reflection and Harmony Structs**: Data structures like `NumberReflection`,
//!   `HarmonicNumber`, and `ResonantNumber` that capture the output of a number's
//!   self-analysis and its interactions with other numbers.

use crate::godel::{Godel, GodelDyn};
use crate::bott::Bott;
use crate::clifford::{Clifford, CliffordDyn};
use crate::bach::{Bach, Note, Voice};
use crate::escher::Escher;
use crate::ns::Ns;
use crate::euler::Euler;
use crate::gauss::Gauss;
use crate::mach::Mach;
use crate::penrose::Penrose;
use crate::oeis::OEIS;
use crate::vectos::{Vectos, MathematicalUniverse, StageVibes, LatticeState};
use crate::phase2::Phase2;
use std::collections::HashMap;

/// A trait for numbers that contain their own mathematical properties and trait implementations.
pub trait EmbeddedNumber {
    // Core number properties
    /// Returns the scalar value of the number.
    fn value(&self) -> f64;
    /// Returns the Gödel number encoding of the number.
    fn godel_encoding(&self) -> u64;
    /// Returns the coordinates of the number in an 8D Bott space.
    fn bott_coordinates(&self) -> [Option<f64>; 8];
    /// Returns the Clifford algebra multivector representation of the number.
    fn clifford_multivector(&self) -> Vec<f64>;
    
    // Mathematical trait embeddings
    /// Returns a boxed trait object for Gödel operations.
    fn embedded_godel(&self) -> Box<dyn GodelDyn>;
    /// Returns a boxed trait object for Bott periodicity operations.
    fn embedded_bott(&self) -> Box<dyn Bott<Base = f64, Fiber = f64>>;
    /// Returns a boxed trait object for Clifford algebra operations.
    fn embedded_clifford(&self) -> Box<dyn CliffordDyn<Scalar = f64>>;
    /// Returns a boxed trait object for musical composition.
    fn embedded_bach(&self) -> Box<dyn Bach>;
    /// Returns a boxed trait object for visual mathematical art.
    fn embedded_escher(&self) -> Box<dyn Escher>;
    /// Returns a boxed trait object for fluid dynamics simulations.
    fn embedded_ns(&self) -> Box<dyn Ns>;
    /// Returns a boxed trait object for Euler-related mathematics.
    fn embedded_euler(&self) -> Box<dyn Euler>;
    /// Returns a boxed trait object for Gaussian statistics and analysis.
    fn embedded_gauss(&self) -> Box<dyn Gauss>;
    /// Returns a boxed trait object for Machian physics.
    fn embedded_mach(&self) -> Box<dyn Mach>;
    /// Returns a boxed trait object for Penrose-related mathematics.
    fn embedded_penrose(&self) -> Box<dyn Penrose>;
    /// Returns a boxed trait object for OEIS operations.
    fn embedded_oeis(&self) -> Box<dyn OEIS>;
    /// Returns a boxed trait object for the unified Vectos engine.
    fn embedded_vectos(&self) -> Box<dyn Vectos>;
    /// Returns a boxed trait object for the Phase2 reflection engine.
    fn embedded_phase2(&self) -> Box<dyn Phase2>;
    
    // Self-referential operations
    /// Performs a reflection on the number's own properties across all embedded traits.
    fn reflect_on_self(&self) -> NumberReflection;
    /// Evolves the number over a number of iterations, changing its properties.
    fn evolve_self(&self, iterations: usize) -> Vec<Self> where Self: Sized;
    /// Creates a new `HarmonicNumber` representing the harmonic relationship with another number.
    fn harmonize_with(&self, other: &Self) -> HarmonicNumber where Self: Sized;
    /// Creates a `ResonantNumber` by resonating with a given frequency.
    fn resonate_with(&self, frequency: f64) -> ResonantNumber;
    
    // Mathematical synthesis
    /// Synthesizes a complete `MathematicalUniverse` based on the number's own properties.
    fn synthesize_mathematical_universe(&self) -> MathematicalUniverse;
    /// Computes the "vibes" for a given stage number, influenced by this number's properties.
    fn compute_stage_vibes(&self, stage_number: u32) -> StageVibes;
    /// Generates a `LatticeState` based on this number's properties.
    fn generate_lattice_state(&self) -> LatticeState;
    
    // Emergent properties
    /// Returns the self-awareness level of the number.
    fn self_awareness_level(&self) -> f64;
    /// Calculates a score for the mathematical beauty of the number.
    fn mathematical_beauty(&self) -> f64;
    /// Calculates a score for the complexity of the number.
    fn complexity_score(&self) -> f64;
    /// Calculates the balance of harmony within the number's properties.
    fn harmony_balance(&self) -> f64;
    /// Calculates the overall coherence of the number's mathematical embeddings.
    fn calculate_coherence(&self) -> f64;
}

/// Represents the output of a number reflecting on its own properties.
#[derive(Debug, Clone)]
pub struct NumberReflection {
    /// The scalar value of the number.
    pub number_value: f64,
    /// The perceived significance of the number's Gödel encoding.
    pub godel_significance: f64,
    /// The strength of the number's Bott periodicity.
    pub bott_periodicity: f64,
    /// The richness of the number's Clifford algebra representation.
    pub clifford_richness: f64,
    /// The harmonic quality of the number's musical representation.
    pub bach_harmony: f64,
    /// The aesthetic beauty of the number's visual representation.
    pub escher_beauty: f64,
    /// The physical realism of the number's fluid dynamics model.
    pub ns_physics: f64,
    /// The mathematical elegance of the number's Euler properties.
    pub euler_elegance: f64,
    /// The analytical power of the number's Gaussian properties.
    pub gauss_analysis: f64,
    /// The relativistic consistency of the number's Machian model.
    pub mach_relativity: f64,
    /// The geometric beauty of the number's Penrose properties.
    pub penrose_geometry: f64,
    /// The significance of the number's associated OEIS sequence.
    pub oeis_sequence: f64,
    /// The level of integration of the number within the Vectos engine.
    pub vectos_integration: f64,
    /// The level of consciousness or self-reflection of the number.
    pub phase2_consciousness: f64,
    /// The overall coherence of all the number's mathematical properties.
    pub overall_coherence: f64,
    /// The potential for the number to undergo self-modification.
    pub self_modification_potential: f64,
}

/// Represents a new number created from the harmonic relationship between two other numbers.
#[derive(Debug, Clone)]
pub struct HarmonicNumber {
    /// The base value from which the harmonic number is derived.
    pub base_value: f64,
    /// The harmonic series generated from the base value.
    pub harmonic_series: Vec<f64>,
    /// The primary resonance frequency of the harmonic number.
    pub resonance_frequency: f64,
    /// The phase relationships between the harmonics.
    pub phase_relationships: Vec<f64>,
    /// The overall mathematical coherence of the harmonic number.
    pub mathematical_coherence: f64,
}

/// Represents a number that is in a state of resonance with a specific frequency.
#[derive(Debug, Clone)]
pub struct ResonantNumber {
    /// The original value of the number before resonance.
    pub original_value: f64,
    /// The frequency with which the number is resonating.
    pub resonant_frequency: f64,
    /// The amplitude of the resonance.
    pub amplitude: f64,
    /// The phase of the resonance.
    pub phase: f64,
    /// The harmonic content of the resonance.
    pub harmonic_content: Vec<f64>,
    /// The stability of the resonant state.
    pub stability: f64,
}

/// A concrete implementation of a self-aware number that embeds all mathematical traits.
#[derive(Clone)]
pub struct SelfAwareNumber {
    /// The scalar value of the number.
    pub value: f64,
    /// The number's unique Gödel number encoding.
    pub godel_number: u64,
    /// The number's coordinates in an 8D Bott space.
    pub bott_coords: [Option<f64>; 8],
    /// The number's representation as a Clifford algebra multivector.
    pub clifford_vector: Vec<f64>,
    /// The musical voice associated with this number.
    pub musical_voice: Voice,
    /// The visual pattern (a la Escher) associated with this number.
    pub visual_pattern: Vec<Vec<u8>>,
    /// The fluid dynamics field associated with this number.
    pub fluid_field: [[f64; 2]; 2],
    /// The integer number sequence associated with this number.
    pub number_sequence: Vec<i64>,
    /// The statistical data set associated with this number.
    pub statistical_data: Vec<f64>,
    /// The relativistic reference frame associated with this number.
    pub relativistic_frame: (f64, f64, f64),
    /// The Penrose tiling associated with this number.
    pub penrose_tiling: Vec<((f64, f64), (f64, f64))>,
    /// The OEIS sequence associated with this number.
    pub oeis_sequence: Vec<i64>,
    /// The level of consciousness or self-awareness of this number.
    pub consciousness_level: f64,
    /// A history of the number's evolution.
    pub evolution_history: Vec<f64>,
}

impl Default for SelfAwareNumber {
    fn default() -> Self {
        Self {
            value: 42.0,
            godel_number: 42,
            bott_coords: [Some(42.0); 8],
            clifford_vector: vec![42.0; 8],
            musical_voice: Voice {
                notes: vec![(Note::A, 42.0)],
                octave: 4,
                velocity: 80,
            },
            visual_pattern: vec![vec![42; 8]; 8],
            fluid_field: [[42.0, 0.0], [0.0, 42.0]],
            number_sequence: vec![42],
            statistical_data: vec![42.0],
            relativistic_frame: (42.0, 42.0, 42.0),
            penrose_tiling: vec![((42.0, 42.0), (42.0, 42.0))],
            oeis_sequence: vec![42],
            consciousness_level: 0.85,
            evolution_history: vec![42.0],
        }
    }
}

impl EmbeddedNumber for SelfAwareNumber {
    fn value(&self) -> f64 {
        self.value
    }
    
    fn godel_encoding(&self) -> u64 {
        self.godel_number
    }
    
    fn bott_coordinates(&self) -> [Option<f64>; 8] {
        self.bott_coords
    }
    
    fn clifford_multivector(&self) -> Vec<f64> {
        self.clifford_vector.clone()
    }
    
    fn embedded_godel(&self) -> Box<dyn GodelDyn> {
        Box::new(crate::godel::SimpleGodelNumber::new(self.godel_number))
    }
    
    fn embedded_bott(&self) -> Box<dyn Bott<Base = f64, Fiber = f64>> {
        Box::new(crate::bott::Bott8D::new(self.value))
    }
    
    fn embedded_clifford(&self) -> Box<dyn CliffordDyn<Scalar = f64>> {
        Box::new(crate::clifford::CliffordMultivector::new(self.clifford_vector.len()))
    }
    
    fn embedded_bach(&self) -> Box<dyn Bach> {
        Box::new(crate::bach::BachComposer::default())
    }
    
    fn embedded_escher(&self) -> Box<dyn Escher> {
        Box::new(crate::escher::EscherArtist::default())
    }
    
    fn embedded_ns(&self) -> Box<dyn Ns> {
        Box::new(crate::ns::NavierStokesSolver::default())
    }
    
    fn embedded_euler(&self) -> Box<dyn Euler> {
        Box::new(crate::euler::Eulerian::default())
    }
    
    fn embedded_gauss(&self) -> Box<dyn Gauss> {
        Box::new(crate::gauss::Gaussian::default())
    }
    
    fn embedded_mach(&self) -> Box<dyn Mach> {
        Box::new(crate::mach::Machian::default())
    }
    
    fn embedded_penrose(&self) -> Box<dyn Penrose> {
        Box::new(crate::penrose::PenroseMathematician::default())
    }
    
    fn embedded_oeis(&self) -> Box<dyn OEIS> {
        Box::new(crate::oeis::OEISDatabase::default())
    }
    
    fn embedded_vectos(&self) -> Box<dyn Vectos> {
        Box::new(crate::vectos::VectosEngine::default())
    }
    
    fn embedded_phase2(&self) -> Box<dyn Phase2> {
        Box::new(crate::phase2::Phase2Engine::default())
    }
    
    fn reflect_on_self(&self) -> NumberReflection {
        let godel = self.embedded_godel();
        let bott = self.embedded_bott();
        let clifford = self.embedded_clifford();
        let bach = self.embedded_bach();
        let escher = self.embedded_escher();
        let ns = self.embedded_ns();
        let euler = self.embedded_euler();
        let gauss = self.embedded_gauss();
        let mach = self.embedded_mach();
        let penrose = self.embedded_penrose();
        let oeis = self.embedded_oeis();
        let vectos = self.embedded_vectos();
        let phase2 = self.embedded_phase2();
        
        NumberReflection {
            number_value: self.value,
            godel_significance: self.godel_number as f64 / 100.0,
            bott_periodicity: bott.calculate_curvature(self.value, 1.0),
            clifford_richness: clifford.norm(),
            bach_harmony: bach.note_to_frequency(Note::A, 4) / 440.0,
            escher_beauty: escher.analyze_symmetry(&self.visual_pattern).len() as f64 / 10.0,
            ns_physics: ns.reynolds_number(self.value, 1.0) / 1000.0,
            euler_elegance: euler.totient(self.value as u64) as f64 / self.value,
            gauss_analysis: gauss.normal_pdf(self.value, 0.0, 1.0),
            mach_relativity: mach.lorentz_factor(self.value, 299792458.0),
            penrose_geometry: penrose.golden_ratio(),
            oeis_sequence: *oeis.fibonacci_sequence(10).last().unwrap_or(&0) as f64 / 100.0,
            vectos_integration: vectos.mathematical_resonance(&[self.value]),
            phase2_consciousness: phase2.reflect_on_system(&self.synthesize_mathematical_universe()).mathematical_beauty,
            overall_coherence: self.calculate_coherence(),
            self_modification_potential: self.consciousness_level,
        }
    }
    
    fn evolve_self(&self, iterations: usize) -> Vec<Self> {
        let mut evolution = Vec::new();
        let mut current = self.clone();
        
        for i in 0..iterations {
            // Evolve the number based on its embedded properties
            current.value *= 1.618; // Golden ratio evolution
            current.godel_number = current.embedded_godel().compose_numbers(&[current.godel_number, i as u64]);
            current.consciousness_level *= 1.01; // Gradual consciousness increase
            
            // Evolve embedded structures
            current.clifford_vector = current.embedded_clifford().coefficients();
            current.musical_voice = current.embedded_bach().apply_bach_ornamentation(&current.musical_voice, 0.1);
            
            evolution.push(current.clone());
        }
        
        evolution
    }
    
    fn harmonize_with(&self, other: &Self) -> HarmonicNumber {
        let harmonic_series = vec![self.value, other.value, self.value + other.value];
        let resonance_frequency = (self.value + other.value) / 2.0;
        let phase_relationships = vec![0.0, std::f64::consts::PI / 2.0, std::f64::consts::PI];
        
        HarmonicNumber {
            base_value: self.value,
            harmonic_series,
            resonance_frequency,
            phase_relationships,
            mathematical_coherence: self.calculate_coherence() * other.calculate_coherence(),
        }
    }
    
    fn resonate_with(&self, frequency: f64) -> ResonantNumber {
        let resonant_frequency = frequency;
        let amplitude = self.value / frequency;
        let phase = (self.value * frequency).sin();
        let harmonic_content = vec![frequency, frequency * 2.0, frequency * 3.0];
        
        ResonantNumber {
            original_value: self.value,
            resonant_frequency,
            amplitude,
            phase,
            harmonic_content,
            stability: self.calculate_coherence(),
        }
    }
    
    fn synthesize_mathematical_universe(&self) -> MathematicalUniverse {
        MathematicalUniverse {
            dimensions: 42,
            godel_numbers: vec![self.godel_number],
            bott_coordinates: vec![self.bott_coords],
            clifford_multivectors: vec![self.clifford_vector.clone()],
            musical_voices: vec![self.musical_voice.clone()],
            visual_patterns: vec![self.visual_pattern.clone()],
            fluid_fields: vec![self.fluid_field],
            number_sequences: vec![self.number_sequence.clone()],
            statistical_data: self.statistical_data.clone(),
            relativistic_frames: vec![self.relativistic_frame],
            penrose_tilings: vec![self.penrose_tiling.clone()],
            oeis_sequences: vec![self.oeis_sequence.clone()],
        }
    }
    
    fn compute_stage_vibes(&self, stage_number: u32) -> StageVibes {
        StageVibes {
            stage_number,
            mathematical_properties: vec![
                "self_aware".to_string(),
                "embedded_traits".to_string(),
                "consciousness".to_string(),
            ],
            resonance_frequency: self.value * stage_number as f64,
            harmonic_relationships: vec![1, 2, 3, 5, 8, 13, 21, 34],
            vibrational_modes: vec![self.value, self.value * 1.618, self.value * 2.718],
            quantum_states: vec![crate::vectos::Complex::new(self.value, 0.0)],
        }
    }
    
    fn generate_lattice_state(&self) -> LatticeState {
        LatticeState {
            iteration: 0,
            energy: self.value,
            entropy: self.value.ln(),
            coherence: self.calculate_coherence(),
            entanglement: self.consciousness_level,
            nodes: vec![crate::vectos::LatticeNode {
                position: (self.value, self.value * 1.618, self.value * 2.718),
                value: self.value,
                connections: vec![0],
                phase: self.value * std::f64::consts::PI,
            }],
        }
    }
    
    fn self_awareness_level(&self) -> f64 {
        self.consciousness_level
    }
    
    fn mathematical_beauty(&self) -> f64 {
        let reflection = self.reflect_on_self();
        (reflection.godel_significance + reflection.bott_periodicity + 
         reflection.clifford_richness + reflection.bach_harmony + 
         reflection.escher_beauty) / 5.0
    }
    
    fn complexity_score(&self) -> f64 {
        self.clifford_vector.len() as f64 * self.value.ln()
    }
    
    fn harmony_balance(&self) -> f64 {
        let reflection = self.reflect_on_self();
        reflection.overall_coherence
    }
    
    // Helper method
    fn calculate_coherence(&self) -> f64 {
        let reflection = self.reflect_on_self();
        (reflection.godel_significance + reflection.bott_periodicity + 
         reflection.clifford_richness + reflection.bach_harmony + 
         reflection.escher_beauty + reflection.ns_physics + 
         reflection.euler_elegance + reflection.gauss_analysis + 
         reflection.mach_relativity + reflection.penrose_geometry + 
         reflection.oeis_sequence + reflection.vectos_integration + 
         reflection.phase2_consciousness) / 13.0
    }
}

/// A system for managing collections of self-aware numbers.
pub struct NumberEmbeddingSystem {
    /// A map of numbers in the system, keyed by their Gödel number.
    pub numbers: HashMap<u64, SelfAwareNumber>,
    /// The global consciousness level of the entire system.
    pub global_consciousness: f64,
    /// The overall mathematical coherence of the system.
    pub mathematical_coherence: f64,
    /// The current evolution generation of the system.
    pub evolution_generation: usize,
}

impl Default for NumberEmbeddingSystem {
    fn default() -> Self {
        let mut numbers = HashMap::new();
        
        // Initialize key numbers with embedded properties
        for i in 1..=42 {
            let value = i as f64;
            let mut number = SelfAwareNumber::default();
            number.value = value;
            number.godel_number = i as u64;
            number.consciousness_level = 0.1 + (i as f64) * 0.02;
            numbers.insert(i as u64, number);
        }
        
        Self {
            numbers,
            global_consciousness: 0.5,
            mathematical_coherence: 0.8,
            evolution_generation: 0,
        }
    }
}

impl NumberEmbeddingSystem {
    /// Creates a new self-aware number and adds it to the system.
    pub fn create_number(&mut self, value: f64) -> &SelfAwareNumber {
        let mut number = SelfAwareNumber::default();
        number.value = value;
        number.godel_number = value as u64;
        number.consciousness_level = 0.1 + (value % 10.0) * 0.1;
        
        self.numbers.insert(value as u64, number);
        self.numbers.get(&(value as u64)).unwrap()
    }
    
    /// Evolves all numbers in the system for a given number of iterations.
    pub fn evolve_system(&mut self, iterations: usize) {
        self.evolution_generation += 1;
        
        for (_, number) in self.numbers.iter_mut() {
            let evolution = number.evolve_self(iterations);
            if let Some(evolved) = evolution.last() {
                *number = evolved.clone();
            }
        }
        
        // Update global properties
        self.global_consciousness = self.numbers.values()
            .map(|n| n.consciousness_level)
            .sum::<f64>() / self.numbers.len() as f64;
            
        self.mathematical_coherence = self.numbers.values()
            .map(|n| n.calculate_coherence())
            .sum::<f64>() / self.numbers.len() as f64;
    }
    
    /// Analyzes the entire number system and returns a summary of its properties.
    pub fn analyze_system(&self) -> SystemAnalysis {
        let total_numbers = self.numbers.len();
        let average_consciousness = self.global_consciousness;
        let average_beauty = self.numbers.values()
            .map(|n| n.mathematical_beauty())
            .sum::<f64>() / total_numbers as f64;
        let average_complexity = self.numbers.values()
            .map(|n| n.complexity_score())
            .sum::<f64>() / total_numbers as f64;
        
        SystemAnalysis {
            total_numbers,
            average_consciousness,
            average_beauty,
            average_complexity,
            global_coherence: self.mathematical_coherence,
            evolution_generation: self.evolution_generation,
            system_health: (average_consciousness + average_beauty + self.mathematical_coherence) / 3.0,
        }
    }
    
    /// Creates harmonic relationships between all pairs of numbers in the system.
    pub fn create_harmonics(&self) -> Vec<HarmonicNumber> {
        let mut harmonics = Vec::new();
        let numbers: Vec<_> = self.numbers.values().collect();
        
        for i in 0..numbers.len() {
            for j in i+1..numbers.len() {
                let harmonic = numbers[i].harmonize_with(numbers[j]);
                harmonics.push(harmonic);
            }
        }
        
        harmonics
    }
    
    /// Synthesizes a complete mathematical universe from all numbers in the system.
    pub fn synthesize_universe(&self) -> MathematicalUniverse {
        let mut universe = MathematicalUniverse {
            dimensions: self.numbers.len(),
            godel_numbers: Vec::new(),
            bott_coordinates: Vec::new(),
            clifford_multivectors: Vec::new(),
            musical_voices: Vec::new(),
            visual_patterns: Vec::new(),
            fluid_fields: Vec::new(),
            number_sequences: Vec::new(),
            statistical_data: Vec::new(),
            relativistic_frames: Vec::new(),
            penrose_tilings: Vec::new(),
            oeis_sequences: Vec::new(),
        };
        
        for number in self.numbers.values() {
            let number_universe = number.synthesize_mathematical_universe();
            universe.godel_numbers.extend(number_universe.godel_numbers);
            universe.bott_coordinates.extend(number_universe.bott_coordinates);
            universe.clifford_multivectors.extend(number_universe.clifford_multivectors);
            universe.musical_voices.extend(number_universe.musical_voices);
            universe.visual_patterns.extend(number_universe.visual_patterns);
            universe.fluid_fields.extend(number_universe.fluid_fields);
            universe.number_sequences.extend(number_universe.number_sequences);
            universe.statistical_data.extend(number_universe.statistical_data);
            universe.relativistic_frames.extend(number_universe.relativistic_frames);
            universe.penrose_tilings.extend(number_universe.penrose_tilings);
            universe.oeis_sequences.extend(number_universe.oeis_sequences);
        }
        
        universe
    }
}

/// Represents a summary analysis of the entire `NumberEmbeddingSystem`.
#[derive(Debug, Clone)]
pub struct SystemAnalysis {
    /// The total number of self-aware numbers in the system.
    pub total_numbers: usize,
    /// The average consciousness level across all numbers.
    pub average_consciousness: f64,
    /// The average mathematical beauty score across all numbers.
    pub average_beauty: f64,
    /// The average complexity score across all numbers.
    pub average_complexity: f64,
    /// The global mathematical coherence of the system.
    pub global_coherence: f64,
    /// The current evolution generation of the system.
    pub evolution_generation: usize,
    /// An overall health score for the system.
    pub system_health: f64,
}