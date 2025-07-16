//! # Emoji Stage Display
//!
//! This module implements the `Display` trait for the `EmojiStage` enum.
//! It maps each stage variant to its corresponding emoji character, allowing
//! the stages to be printed as their symbolic representations.

use std::fmt;
use super::emoji_stage_enum::EmojiStage;

impl fmt::Display for EmojiStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let emoji = match self {
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
            EmojiStage::Physical => "🔌",
            EmojiStage::DataLink => "🔗",
            EmojiStage::Network => "🌐",
            EmojiStage::Transport => "📦",
            EmojiStage::Session => "🤝",
            EmojiStage::Presentation => "🎨",
            EmojiStage::Application => "💻",
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
            EmojiStage::Unknown(s) => s.as_str(),
            _ => "❓",
        };
        write!(f, "{}", emoji)
    }
}