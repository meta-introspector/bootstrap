use std::collections::HashMap;
use std::hash::Hash;

/// Represents one of the 42 phases in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Phase(pub u8);

impl Phase {
    pub fn new(phase: u8) -> Option<Self> {
        if phase <= 42 {
            Some(Phase(phase))
        } else {
            None
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    /// Get the mathematical properties of this phase
    pub fn properties(&self) -> PhaseProperties {
        PhaseProperties {
            phase: self.0,
            is_prime: self.is_prime(),
            is_fibonacci: self.is_fibonacci(),
            is_perfect_square: self.is_perfect_square(),
            factors: self.factors(),
            resonance_frequency: self.calculate_resonance_frequency(),
        }
    }

    fn is_prime(&self) -> bool {
        if self.0 < 2 { return false; }
        if self.0 == 2 { return true; }
        if self.0 % 2 == 0 { return false; }
        
        let sqrt_n = (self.0 as f64).sqrt() as u8;
        for i in (3..=sqrt_n).step_by(2) {
            if self.0 % i == 0 { return false; }
        }
        true
    }

    fn is_fibonacci(&self) -> bool {
        let mut a = 0u8;
        let mut b = 1u8;
        
        while b <= self.0 {
            if b == self.0 { return true; }
            let temp = a + b;
            a = b;
            b = temp;
        }
        false
    }

    fn is_perfect_square(&self) -> bool {
        let sqrt = (self.0 as f64).sqrt() as u8;
        sqrt * sqrt == self.0
    }

    fn factors(&self) -> Vec<u8> {
        let mut factors = Vec::new();
        for i in 1..=self.0 {
            if self.0 % i == 0 {
                factors.push(i);
            }
        }
        factors
    }

    fn calculate_resonance_frequency(&self) -> f64 {
        let base_freq = self.0 as f64;
        let prime_bonus = if self.is_prime() { 1.5 } else { 1.0 };
        let fibonacci_bonus = if self.is_fibonacci() { 1.3 } else { 1.0 };
        let square_bonus = if self.is_perfect_square() { 1.2 } else { 1.0 };
        
        base_freq * prime_bonus * fibonacci_bonus * square_bonus
    }
}

/// Mathematical properties of a phase
#[derive(Debug, Clone)]
pub struct PhaseProperties {
    pub phase: u8,
    pub is_prime: bool,
    pub is_fibonacci: bool,
    pub is_perfect_square: bool,
    pub factors: Vec<u8>,
    pub resonance_frequency: f64,
}

/// Trait for dimensionality reduction algorithms
pub trait DimensionalityReducer {
    fn reduce_to_phase(&self, embedding: &[f64]) -> Phase;
    fn calculate_confidence(&self, embedding: &[f64], phase: Phase) -> f64;
    fn calculate_harmonic_resonance(&self, embedding: &[f64], phase: Phase) -> f64;
}

/// Hash-based dimensionality reducer
pub struct HashReducer;

impl DimensionalityReducer for HashReducer {
    fn reduce_to_phase(&self, embedding: &[f64]) -> Phase {
        // Simple hash-based mapping
        let mut hash = 0u64;
        for (i, &val) in embedding.iter().enumerate() {
            hash = hash.wrapping_add((val * 1000.0) as u64 * (i + 1) as u64);
        }
        
        let phase_num = (hash % 42) as u8 + 1;
        Phase::new(phase_num).unwrap_or(Phase(1))
    }

    fn calculate_confidence(&self, _embedding: &[f64], _phase: Phase) -> f64 {
        0.8 // Hash-based mapping has consistent confidence
    }

    fn calculate_harmonic_resonance(&self, _embedding: &[f64], _phase: Phase) -> f64 {
        0.8 // Hash-based mapping has consistent resonance
    }
}

/// Harmonic dimensionality reducer
pub struct HarmonicReducer;

impl DimensionalityReducer for HarmonicReducer {
    fn reduce_to_phase(&self, embedding: &[f64]) -> Phase {
        // Harmonic mapping based on mathematical resonance
        let mut best_phase = Phase(1);
        let mut best_resonance = 0.0;
        
        for phase_num in 1..=42 {
            if let Some(phase) = Phase::new(phase_num) {
                let resonance = self.calculate_harmonic_resonance(embedding, phase);
                if resonance > best_resonance {
                    best_resonance = resonance;
                    best_phase = phase;
                }
            }
        }
        
        best_phase
    }

    fn calculate_confidence(&self, embedding: &[f64], phase: Phase) -> f64 {
        self.calculate_harmonic_resonance(embedding, phase)
    }

    fn calculate_harmonic_resonance(&self, embedding: &[f64], phase: Phase) -> f64 {
        let phase_props = phase.properties();
        let embedding_sum = embedding.iter().sum::<f64>();
        let embedding_norm = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if embedding_norm == 0.0 { return 0.0; }
        
        let normalized_sum = embedding_sum / embedding_norm;
        let phase_frequency = phase_props.resonance_frequency;
        
        // Calculate harmonic resonance
        let harmonic_ratio = normalized_sum / phase_frequency;
        let resonance = 1.0 / (1.0 + (harmonic_ratio - 1.0).abs());
        
        // Apply mathematical bonuses
        let prime_bonus = if phase_props.is_prime { 1.2 } else { 1.0 };
        let fibonacci_bonus = if phase_props.is_fibonacci { 1.1 } else { 1.0 };
        
        (resonance * prime_bonus * fibonacci_bonus).min(1.0)
    }
}

/// Entity that can be mapped to a phase
pub trait PhaseEntity {
    fn get_name(&self) -> &str;
    fn get_embedding(&self) -> &[f64];
}

/// Function entity
#[derive(Debug, Clone)]
pub struct FunctionEntity {
    pub name: String,
    pub embedding: Vec<f64>,
    pub semantic_type: String,
}

impl PhaseEntity for FunctionEntity {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_embedding(&self) -> &[f64] {
        &self.embedding
    }
}

/// Module entity
#[derive(Debug, Clone)]
pub struct ModuleEntity {
    pub name: String,
    pub embedding: Vec<f64>,
    pub functions: Vec<String>,
}

impl PhaseEntity for ModuleEntity {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_embedding(&self) -> &[f64] {
        &self.embedding
    }
}

/// Statistics for a phase
#[derive(Debug, Clone)]
pub struct PhaseStatistics {
    pub entity_count: usize,
    pub entities: Vec<String>,
    pub properties: PhaseProperties,
}

/// Phase mapping system
pub struct PhaseMappingSystem {
    reducer: Box<dyn DimensionalityReducer>,
    entity_phases: HashMap<String, Phase>,
    phase_entities: HashMap<Phase, Vec<String>>,
}

impl PhaseMappingSystem {
    pub fn new(reducer: Box<dyn DimensionalityReducer>) -> Self {
        Self {
            reducer,
            entity_phases: HashMap::new(),
            phase_entities: HashMap::new(),
        }
    }

    /// Map an entity to a phase
    pub fn map_entity(&mut self, entity: &dyn PhaseEntity) -> Phase {
        let embedding = entity.get_embedding();
        let phase = self.reducer.reduce_to_phase(embedding);
        
        // Store the mapping
        let entity_name = entity.get_name().to_string();
        self.entity_phases.insert(entity_name.clone(), phase);
        
        self.phase_entities.entry(phase).or_insert_with(Vec::new).push(entity_name);
        
        phase
    }

    /// Get the phase for an entity
    pub fn get_entity_phase(&self, entity_name: &str) -> Option<Phase> {
        self.entity_phases.get(entity_name).cloned()
    }

    /// Get entities in a specific phase
    pub fn get_phase_entities(&self, phase: Phase) -> Vec<String> {
        self.phase_entities.get(&phase).cloned().unwrap_or_default()
    }

    /// Calculate mapping confidence for an entity-phase pair
    pub fn get_mapping_confidence(&self, entity: &dyn PhaseEntity, phase: Phase) -> f64 {
        let embedding = entity.get_embedding();
        self.reducer.calculate_confidence(embedding, phase)
    }

    /// Find entities that resonate with a target phase
    pub fn find_resonant_entities(&self, target_phase: Phase, threshold: f64) -> Vec<String> {
        let mut resonant = Vec::new();
        
        for (entity_name, &phase) in &self.entity_phases {
            if phase == target_phase {
                resonant.push(entity_name.clone());
            } else {
                // Check cross-phase resonance
                let phase_diff = (phase.value() as f64 - target_phase.value() as f64).abs();
                let resonance = 1.0 / (1.0 + phase_diff);
                if resonance >= threshold {
                    resonant.push(entity_name.clone());
                }
            }
        }
        
        resonant
    }

    /// Get statistics for all phases
    pub fn get_phase_statistics(&self) -> HashMap<Phase, PhaseStatistics> {
        let mut stats = HashMap::new();
        
        for phase_num in 1..=42 {
            if let Some(phase) = Phase::new(phase_num) {
                let entities = self.get_phase_entities(phase);
                let properties = phase.properties();
                
                stats.insert(phase, PhaseStatistics {
                    entity_count: entities.len(),
                    entities: entities.clone(),
                    properties,
                });
            }
        }
        
        stats
    }

    /// Calculate average embedding for entities in a phase
    pub fn get_phase_average_embedding(&self, phase: Phase) -> Option<Vec<f64>> {
        let entities = self.get_phase_entities(phase);
        if entities.is_empty() { return None; }
        
        // Get embeddings for all entities in this phase
        let embeddings: Vec<&[f64]> = entities.iter()
            .filter_map(|name| self.entity_phases.get(name))
            .map(|_| &[0.0][..]) // Placeholder - in real implementation, you'd store embeddings
            .collect();
        
        if embeddings.is_empty() { return None; }
        
        // Calculate average embedding
        let embedding_size = embeddings[0].len();
        let mut average = vec![0.0; embedding_size];
        
        for embedding in &embeddings {
            for (i, &val) in embedding.iter().enumerate() {
                average[i] += val;
            }
        }
        
        for val in &mut average {
            *val /= embeddings.len() as f64;
        }
        
        Some(average)
    }

    /// Analyze phase distribution
    pub fn analyze_phase_distribution(&self) -> PhaseDistributionAnalysis {
        let mut distribution = HashMap::new();
        let mut total_entities = 0;
        
        for phase_num in 1..=42 {
            if let Some(phase) = Phase::new(phase_num) {
                let entities = self.get_phase_entities(phase);
                let count = entities.len();
                distribution.insert(phase, count);
                total_entities += count;
            }
        }
        
        let most_populated_phase = distribution.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&phase, _)| phase);
        
        let least_populated_phase = distribution.iter()
            .min_by_key(|(_, &count)| count)
            .map(|(&phase, _)| phase);
        
        PhaseDistributionAnalysis {
            total_entities,
            distribution,
            most_populated_phase,
            least_populated_phase,
        }
    }
}

/// Analysis of phase distribution
#[derive(Debug, Clone)]
pub struct PhaseDistributionAnalysis {
    pub total_entities: usize,
    pub distribution: HashMap<Phase, usize>,
    pub most_populated_phase: Option<Phase>,
    pub least_populated_phase: Option<Phase>,
}