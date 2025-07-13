use crate::godel::{Godel, GodelExt, GodelNumber};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmojiStage {
    // Core Mathematical Vibes (1-10)
    Unity,      // 1 - The fundamental unit
    Prime,      // 2 - First prime
    Triangle,   // 3 - First triangular number
    Square,     // 4 - First perfect square
    Pentagon,   // 5 - First pentagonal number
    Hexagon,    // 6 - First hexagonal number
    Lucky,      // 7 - Lucky number
    Cube,       // 8 - First cube
    Magic,      // 9 - Magic square base
    Decimal,    // 10 - Decimal system base

    // System Infrastructure Vibes (11-20)
    Network,    // 11 - Network protocols
    Time,       // 12 - Time divisions
    Unlucky,    // 13 - Unlucky number
    Composite,  // 14 - Composite number
    Pentadecagon, // 15 - Pentadecagon
    Hexadecagon,  // 16 - Hexadecagon
    Prime17,    // 17 - Prime number
    Composite18, // 18 - Composite number
    Prime19,    // 19 - Prime number
    Vigesimal,  // 20 - Vigesimal system

    // Higher-Level Vibes (21-30)
    Triangle21, // 21 - Triangular number
    Composite22, // 22 - Composite number
    Prime23,    // 23 - Prime number
    Factorial,  // 24 - 4! (factorial)
    Square25,   // 25 - 5²
    Composite26, // 26 - Composite number
    Prime27,    // 27 - 3³ (cube)
    Composite28, // 28 - Perfect number
    Prime29,    // 29 - Prime number
    Composite30, // 30 - Composite number

    // Advanced System Vibes (31-40)
    Prime31,    // 31 - Prime number
    Power32,    // 32 - 2⁵
    Prime33,    // 33 - Composite number
    Composite34, // 34 - Composite number
    Composite35, // 35 - Composite number
    Square36,   // 36 - 6²
    Prime37,    // 37 - Prime number
    Composite38, // 38 - Composite number
    Prime39,    // 39 - Composite number
    Composite40, // 40 - Composite number

    // Ultimate Vibes (41-42)
    Prime41,    // 41 - Prime number
    TheWorld,   // 42 - The answer to life, universe, everything

    // OSI Layer Vibes (mapped to specific stages)
    Physical,   // OSI Layer 1 - Physical layer (hardware, signals)
    DataLink,   // OSI Layer 2 - Data link layer (frames, MAC)
    Network,    // OSI Layer 3 - Network layer (routing, IP)
    Transport,  // OSI Layer 4 - Transport layer (TCP/UDP)
    Session,    // OSI Layer 5 - Session layer (connections)
    Presentation, // OSI Layer 6 - Presentation layer (encoding)
    Application, // OSI Layer 7 - Application layer (protocols)

    // Emojilang Vibes
    Rocket,     // 🚀 - Launch/execute
    Crab,       // 🦀 - Rust/compile
    Dna,        // 🧬 - Genetic/evolution
    Coin,       // 🪙 - Cryptocurrency/blockchain
    Brain,      // 🧠 - AI/neural networks
    Heart,      // ❤️ - Love/emotion
    Star,       // ⭐ - Excellence/rating
    Fire,       // 🔥 - Hot/trending
    Lightning,  // ⚡ - Fast/energy
    Shield,     // 🛡️ - Security/protection

    // Add more as needed
    Unknown(String), // fallback for unrecognized emojis
}

impl EmojiStage {
    /// Convert from emoji string to enum
    pub fn from_emoji(s: &str) -> Self {
        match s {
            // OSI Layers
            "🔌" => EmojiStage::Physical,
            "🔗" => EmojiStage::DataLink,
            "🌐" => EmojiStage::Network,
            "📦" => EmojiStage::Transport,
            "🤝" => EmojiStage::Session,
            "🎨" => EmojiStage::Presentation,
            "💻" => EmojiStage::Application,
            
            // Emojilang Vibes
            "🚀" => EmojiStage::Rocket,
            "🦀" => EmojiStage::Crab,
            "🧬" => EmojiStage::Dna,
            "🪙" => EmojiStage::Coin,
            "🧠" => EmojiStage::Brain,
            "❤️" => EmojiStage::Heart,
            "⭐" => EmojiStage::Star,
            "🔥" => EmojiStage::Fire,
            "⚡" => EmojiStage::Lightning,
            "🛡️" => EmojiStage::Shield,
            
            // Numeric stages (1-42)
            "1️⃣" => EmojiStage::Unity,
            "2️⃣" => EmojiStage::Prime,
            "3️⃣" => EmojiStage::Triangle,
            "4️⃣" => EmojiStage::Square,
            "5️⃣" => EmojiStage::Pentagon,
            "6️⃣" => EmojiStage::Hexagon,
            "7️⃣" => EmojiStage::Lucky,
            "8️⃣" => EmojiStage::Cube,
            "9️⃣" => EmojiStage::Magic,
            "🔟" => EmojiStage::Decimal,
            // ... continue for 11-42
            
            other => EmojiStage::Unknown(other.to_string()),
        }
    }

    /// Get the emoji string for this stage
    pub fn as_emoji(&self) -> &str {
        match self {
            // OSI Layers
            EmojiStage::Physical => "🔌",
            EmojiStage::DataLink => "🔗",
            EmojiStage::Network => "🌐",
            EmojiStage::Transport => "📦",
            EmojiStage::Session => "🤝",
            EmojiStage::Presentation => "🎨",
            EmojiStage::Application => "💻",
            
            // Emojilang Vibes
            EmojiStage::Rocket => "🚀",
            EmojiStage::Crab => "🦀",
            EmojiStage::Dna => "🧬",
            EmojiStage::Coin => "🪙",
            EmojiStage::Brain => "🧠",
            EmojiStage::Heart => "❤️",
            EmojiStage::Star => "⭐",
            EmojiStage::Fire => "🔥",
            EmojiStage::Lightning => "⚡",
            EmojiStage::Shield => "🛡️",
            
            // Numeric stages
            EmojiStage::Unity => "1️⃣",
            EmojiStage::Prime => "2️⃣",
            EmojiStage::Triangle => "3️⃣",
            EmojiStage::Square => "4️⃣",
            EmojiStage::Pentagon => "5️⃣",
            EmojiStage::Hexagon => "6️⃣",
            EmojiStage::Lucky => "7️⃣",
            EmojiStage::Cube => "8️⃣",
            EmojiStage::Magic => "9️⃣",
            EmojiStage::Decimal => "🔟",
            // ... continue for 11-42
            
            EmojiStage::Unknown(s) => s.as_str(),
        }
    }

    /// Get all known EmojiStage variants
    pub fn all_known() -> Vec<Self> {
        vec![
            EmojiStage::Unity,
            EmojiStage::Duality,
            EmojiStage::Trinity,
            EmojiStage::Tetrad,
            EmojiStage::Pentad,
            EmojiStage::Hexad,
            EmojiStage::Septad,
            EmojiStage::Octad,
            EmojiStage::Ennead,
            EmojiStage::Decad,
            EmojiStage::Time,
            EmojiStage::Unlucky,
            EmojiStage::Composite,
            EmojiStage::Pentadecagon,
            EmojiStage::Hexadecagon,
            EmojiStage::Heptadecagon,
            EmojiStage::Octadecagon,
            EmojiStage::Enneadecagon,
            EmojiStage::Icosagon,
            EmojiStage::Icosihenagon,
            EmojiStage::Icosidigon,
            EmojiStage::Icositrigon,
            EmojiStage::Icositetragon,
            EmojiStage::Icosipentagon,
            EmojiStage::Icosihexagon,
            EmojiStage::Icosiheptagon,
            EmojiStage::Icosioctagon,
            EmojiStage::Icosienneagon,
            EmojiStage::Triacontagon,
            EmojiStage::Triacontahenagon,
            EmojiStage::Triacontadigon,
            EmojiStage::Triacontatrigon,
            EmojiStage::Triacontatetragon,
            EmojiStage::Triacontapentagon,
            EmojiStage::Triacontahexagon,
            EmojiStage::Triacontaheptagon,
            EmojiStage::Triacontaoctagon,
            EmojiStage::Triacontenneagon,
            EmojiStage::Tetracontagon,
            EmojiStage::Tetracontahenagon,
            EmojiStage::Answer,
            // OSI Layer mappings
            EmojiStage::Physical,
            EmojiStage::DataLink,
            EmojiStage::Network,
            EmojiStage::Transport,
            EmojiStage::Session,
            EmojiStage::Presentation,
            EmojiStage::Application,
        ]
    }

    /// Get the stage number (1-42)
    pub fn stage_number(&self) -> u8 {
        match self {
            EmojiStage::Unity => 1,
            EmojiStage::Duality => 2,
            EmojiStage::Trinity => 3,
            EmojiStage::Tetrad => 4,
            EmojiStage::Pentad => 5,
            EmojiStage::Hexad => 6,
            EmojiStage::Septad => 7,
            EmojiStage::Octad => 8,
            EmojiStage::Ennead => 9,
            EmojiStage::Decad => 10,
            EmojiStage::Time => 12,
            EmojiStage::Unlucky => 13,
            EmojiStage::Composite => 14,
            EmojiStage::Pentadecagon => 15,
            EmojiStage::Hexadecagon => 16,
            EmojiStage::Heptadecagon => 17,
            EmojiStage::Octadecagon => 18,
            EmojiStage::Enneadecagon => 19,
            EmojiStage::Icosagon => 20,
            EmojiStage::Icosihenagon => 21,
            EmojiStage::Icosidigon => 22,
            EmojiStage::Icositrigon => 23,
            EmojiStage::Icositetragon => 24,
            EmojiStage::Icosipentagon => 25,
            EmojiStage::Icosihexagon => 26,
            EmojiStage::Icosiheptagon => 27,
            EmojiStage::Icosioctagon => 28,
            EmojiStage::Icosienneagon => 29,
            EmojiStage::Triacontagon => 30,
            EmojiStage::Triacontahenagon => 31,
            EmojiStage::Triacontadigon => 32,
            EmojiStage::Triacontatrigon => 33,
            EmojiStage::Triacontatetragon => 34,
            EmojiStage::Triacontapentagon => 35,
            EmojiStage::Triacontahexagon => 36,
            EmojiStage::Triacontaheptagon => 37,
            EmojiStage::Triacontaoctagon => 38,
            EmojiStage::Triacontenneagon => 39,
            EmojiStage::Tetracontagon => 40,
            EmojiStage::Tetracontahenagon => 41,
            EmojiStage::Answer => 42,
            // OSI Layer mappings (use special numbers)
            EmojiStage::Physical => 101,
            EmojiStage::DataLink => 102,
            EmojiStage::Network => 103,
            EmojiStage::Transport => 104,
            EmojiStage::Session => 105,
            EmojiStage::Presentation => 106,
            EmojiStage::Application => 107,
            EmojiStage::Unknown(_) => 0,
        }
    }

    /// Get the OSI layer number (1-7) for OSI-related stages
    pub fn osi_layer(&self) -> Option<u8> {
        match self {
            EmojiStage::Physical => Some(1),
            EmojiStage::DataLink => Some(2),
            EmojiStage::Network => Some(3),
            EmojiStage::Transport => Some(4),
            EmojiStage::Session => Some(5),
            EmojiStage::Presentation => Some(6),
            EmojiStage::Application => Some(7),
            _ => None,
        }
    }

    /// Check if this stage is an OSI layer
    pub fn is_osi_layer(&self) -> bool {
        self.osi_layer().is_some()
    }

    /// Get the mathematical properties of this stage
    pub fn mathematical_properties(&self) -> StageProperties {
        let stage_num = self.stage_number();
        StageProperties {
            stage_number: stage_num,
            is_prime: is_prime(stage_num as u64),
            is_fibonacci: is_fibonacci(stage_num as u64),
            is_perfect_square: is_perfect_square(stage_num as u64),
            factors: get_factors(stage_num as u64),
            resonance_frequency: calculate_resonance_frequency(stage_num as u64),
        }
    }
}

/// Mathematical properties of a stage
#[derive(Debug, Clone)]
pub struct StageProperties {
    pub stage_number: u8,
    pub is_prime: bool,
    pub is_fibonacci: bool,
    pub is_perfect_square: bool,
    pub factors: Vec<u64>,
    pub resonance_frequency: f64,
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

fn is_perfect_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
}

fn get_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
    }
    factors
}

fn calculate_resonance_frequency(n: u64) -> f64 {
    let base_freq = n as f64;
    let prime_bonus = if is_prime(n) { 1.5 } else { 1.0 };
    let fibonacci_bonus = if is_fibonacci(n) { 1.3 } else { 1.0 };
    let square_bonus = if is_perfect_square(n) { 1.2 } else { 1.0 };
    
    base_freq * prime_bonus * fibonacci_bonus * square_bonus
}

impl std::fmt::Display for EmojiStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            EmojiStage::Unity => "1️⃣",
            EmojiStage::Duality => "2️⃣",
            EmojiStage::Trinity => "3️⃣",
            EmojiStage::Tetrad => "4️⃣",
            EmojiStage::Pentad => "5️⃣",
            EmojiStage::Hexad => "6️⃣",
            EmojiStage::Septad => "7️⃣",
            EmojiStage::Octad => "8️⃣",
            EmojiStage::Ennead => "9️⃣",
            EmojiStage::Decad => "🔟",
            EmojiStage::Time => "⏰",
            EmojiStage::Unlucky => "😱",
            EmojiStage::Composite => "🔢",
            EmojiStage::Pentadecagon => "15️⃣",
            EmojiStage::Hexadecagon => "16️⃣",
            EmojiStage::Heptadecagon => "17️⃣",
            EmojiStage::Octadecagon => "18️⃣",
            EmojiStage::Enneadecagon => "19️⃣",
            EmojiStage::Icosagon => "20️⃣",
            EmojiStage::Icosihenagon => "21️⃣",
            EmojiStage::Icosidigon => "22️⃣",
            EmojiStage::Icositrigon => "23️⃣",
            EmojiStage::Icositetragon => "24️⃣",
            EmojiStage::Icosipentagon => "25️⃣",
            EmojiStage::Icosihexagon => "26️⃣",
            EmojiStage::Icosiheptagon => "27️⃣",
            EmojiStage::Icosioctagon => "28️⃣",
            EmojiStage::Icosienneagon => "29️⃣",
            EmojiStage::Triacontagon => "30️⃣",
            EmojiStage::Triacontahenagon => "31️⃣",
            EmojiStage::Triacontadigon => "32️⃣",
            EmojiStage::Triacontatrigon => "33️⃣",
            EmojiStage::Triacontatetragon => "34️⃣",
            EmojiStage::Triacontapentagon => "35️⃣",
            EmojiStage::Triacontahexagon => "36️⃣",
            EmojiStage::Triacontaheptagon => "37️⃣",
            EmojiStage::Triacontaoctagon => "38️⃣",
            EmojiStage::Triacontenneagon => "39️⃣",
            EmojiStage::Tetracontagon => "40️⃣",
            EmojiStage::Tetracontahenagon => "41️⃣",
            EmojiStage::Answer => "42️⃣",
            // OSI Layer mappings
            EmojiStage::Physical => "🔌",    // OSI Layer 1 - Physical layer (cables, signals)
            EmojiStage::DataLink => "🔗",    // OSI Layer 2 - Data link layer (frames, MAC)
            EmojiStage::Network => "🌐",     // OSI Layer 3 - Network layer (routing, IP)
            EmojiStage::Transport => "🚚",   // OSI Layer 4 - Transport layer (TCP/UDP)
            EmojiStage::Session => "🤝",     // OSI Layer 5 - Session layer (connections)
            EmojiStage::Presentation => "🎨", // OSI Layer 6 - Presentation layer (formatting)
            EmojiStage::Application => "💻", // OSI Layer 7 - Application layer (apps)
            EmojiStage::Unknown(s) => s.as_str(),
        };
        write!(f, "{}", emoji)
    }
}

// Implement the Godel trait for EmojiStage
impl Godel for EmojiStage {
    fn godel_number(&self) -> u64 {
        match self {
            EmojiStage::Unity => 2,        // 1st prime
            EmojiStage::Duality => 3,      // 2nd prime
            EmojiStage::Trinity => 5,      // 3rd prime
            EmojiStage::Tetrad => 7,       // 4th prime
            EmojiStage::Pentad => 11,      // 5th prime
            EmojiStage::Hexad => 13,       // 6th prime
            EmojiStage::Septad => 17,      // 7th prime
            EmojiStage::Octad => 19,       // 8th prime
            EmojiStage::Ennead => 23,      // 9th prime
            EmojiStage::Decad => 29,       // 10th prime
            EmojiStage::Time => 31,        // 11th prime
            EmojiStage::Unlucky => 37,     // 12th prime
            EmojiStage::Composite => 41,   // 13th prime
            EmojiStage::Pentadecagon => 43, // 14th prime
            EmojiStage::Hexadecagon => 47,  // 15th prime
            EmojiStage::Heptadecagon => 53, // 16th prime
            EmojiStage::Octadecagon => 59,  // 17th prime
            EmojiStage::Enneadecagon => 61, // 18th prime
            EmojiStage::Icosagon => 67,     // 19th prime
            EmojiStage::Icosihenagon => 71, // 20th prime
            EmojiStage::Icosidigon => 73,   // 21st prime
            EmojiStage::Icositrigon => 79,  // 22nd prime
            EmojiStage::Icositetragon => 83, // 23rd prime
            EmojiStage::Icosipentagon => 89, // 24th prime
            EmojiStage::Icosihexagon => 97,  // 25th prime
            EmojiStage::Icosiheptagon => 101, // 26th prime
            EmojiStage::Icosioctagon => 103,  // 27th prime
            EmojiStage::Icosienneagon => 107, // 28th prime
            EmojiStage::Triacontagon => 109,  // 29th prime
            EmojiStage::Triacontahenagon => 113, // 30th prime
            EmojiStage::Triacontadigon => 127,   // 31st prime
            EmojiStage::Triacontatrigon => 131,  // 32nd prime
            EmojiStage::Triacontatetragon => 137, // 33rd prime
            EmojiStage::Triacontapentagon => 139, // 34th prime
            EmojiStage::Triacontahexagon => 149,  // 35th prime
            EmojiStage::Triacontaheptagon => 151, // 36th prime
            EmojiStage::Triacontaoctagon => 157,  // 37th prime
            EmojiStage::Triacontenneagon => 163,  // 38th prime
            EmojiStage::Tetracontagon => 167,     // 39th prime
            EmojiStage::Tetracontahenagon => 173, // 40th prime
            EmojiStage::Answer => 179,            // 41st prime
            // OSI Layer mappings
            EmojiStage::Physical => 181,    // 42nd prime
            EmojiStage::DataLink => 191,    // 43rd prime
            EmojiStage::Network => 193,     // 44th prime
            EmojiStage::Transport => 197,   // 45th prime
            EmojiStage::Session => 199,     // 46th prime
            EmojiStage::Presentation => 211, // 47th prime
            EmojiStage::Application => 223,  // 48th prime
            EmojiStage::Unknown(_) => 1,    // Default for unknown
        }
    }

    fn compose_godel(items: &[Self]) -> u64 {
        items.iter().map(|item| item.godel_number()).product()
    }

    fn decompose_godel(number: u64) -> Vec<Self> {
        let mut result = Vec::new();
        let mut remaining = number;
        
        // Try to match each prime to an EmojiStage
        for stage in Self::all_known() { // Changed to all_known()
            let godel_num = stage.godel_number();
            while remaining % godel_num == 0 {
                result.push(stage.clone());
                remaining /= godel_num;
            }
        }
        
        result
    }

    fn is_godel_prime(&self) -> bool {
        // All EmojiStage variants are assigned unique primes
        true
    }

    fn godel_factors(&self) -> Vec<u64> {
        vec![self.godel_number()]
    }

    fn godel_equivalent(&self, other: &Self) -> bool {
        self.godel_number() == other.godel_number()
    }
} 