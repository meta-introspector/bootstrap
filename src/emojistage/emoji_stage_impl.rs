use super::emoji_stage_enum::EmojiStage;

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
            _ => "❓", // Default for missing variants
        }
    }

    /// Get all known EmojiStage variants
    pub fn all_known() -> Vec<Self> {
        vec![
            EmojiStage::Unity,
            // ... (other variants as needed)
        ]
    }
    // ... (other methods as in the original impl)
} 