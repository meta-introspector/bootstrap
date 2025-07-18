//! # EmojiStage Module: Symbolic Representation within the Code-Math Manifold
//! 
//! This module serves as the central hub for defining and managing `EmojiStage` entities 
//! within the `solfunmeme-dioxus` system. `EmojiStage` represents a crucial bridge between 
//! abstract mathematical concepts, numerical "vibes," and concrete symbolic representations 
//! (emojis). Each `EmojiStage` embodies a unique aspect or "quasifiber" of the system, 
//! allowing for a visually intuitive and semantically rich mapping of the Code-Math Manifold.
//! 
//! ## Philosophical Significance
//! 
//! The use of emojis as symbolic representations aligns with the project's goal of creating 
//! a self-describing system where meaning is inherent and visually accessible. By associating 
//! mathematical properties (like primality, Fibonacci membership, or resonance frequency) 
//! with specific emojis, the system gains a layer of intuitive understanding and allows for 
//! the "visualization" of abstract relationships and emergent properties.
//! 
//! ## Core Functionality
//! 
//! This module re-exports various sub-modules that collectively define the behavior and 
//! properties of an `EmojiStage`:
//! 
//! - `emoji_stage_enum`: Defines the `EmojiStage` enum itself, enumerating the distinct stages.
//! - `emoji_stage_impl`: Provides core implementations and methods for `EmojiStage`.
//! - `stage_properties`: Defines traits and structures for various mathematical and conceptual properties of each stage.
//! - `is_prime`, `is_fibonacci`, `is_perfect_square`, `get_factors`: Utility functions for determining mathematical characteristics.
//! - `calculate_resonance_frequency`: Computes a unique resonance frequency for each stage, contributing to its "vibe."
//! - `emoji_stage_display`: Handles the display and string representation of `EmojiStage`.
//! - `emoji_stage_godel`: Integrates `EmojiStage` with the Gödel numbering system, allowing stages to be encoded numerically.
//! 
//! Through these components, `EmojiStage` contributes significantly to the system's ability 
//! to self-describe and reflect upon its own symbolic and mathematical structure.

pub mod emoji_stage_enum;
/// Implementation of the EmojiStage enum.
pub mod emoji_stage_impl;
pub mod stage_properties;
/// Utility function to check if a number is prime.
pub mod is_prime;
/// Utility function to check if a number is a Fibonacci number.
pub mod is_fibonacci;
/// Utility function to check if a number is a perfect square.
pub mod is_perfect_square;
/// Utility function to get the factors of a number.
pub mod get_factors;
pub mod calculate_resonance_frequency;
pub mod emoji_stage_display;
pub mod emoji_stage_godel;

pub use emoji_stage_enum::EmojiStage;
pub use stage_properties::StageProperties;
pub use is_prime::is_prime;
pub use is_fibonacci::is_fibonacci;
pub use is_perfect_square::is_perfect_square;
pub use get_factors::get_factors;
pub use calculate_resonance_frequency::calculate_resonance_frequency; 