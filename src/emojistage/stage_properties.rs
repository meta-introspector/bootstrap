#[derive(Debug, Clone)]
pub struct StageProperties {
    pub stage_number: u8,
    pub is_prime: bool,
    pub is_fibonacci: bool,
    pub is_perfect_square: bool,
    pub factors: Vec<u64>,
    pub resonance_frequency: f64,
} 