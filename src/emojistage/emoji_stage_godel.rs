//! # Emoji Stage Gödel Numbering
//!
//! This module implements the `Godel` trait for the `EmojiStage` enum.
//! It provides the logic for converting each symbolic stage into a unique
//! prime number, allowing systems of stages to be represented as a single
//! composite Gödel number.

use super::emoji_stage_enum::EmojiStage;
use crate::godel::Godel;

impl Godel for EmojiStage {
    fn godel_number(&self) -> u64 {
        match self {
            EmojiStage::Unity => 2,
            EmojiStage::Prime => 3,
            EmojiStage::Triangle => 5,
            EmojiStage::Square => 7,
            EmojiStage::Pentagon => 11,
            EmojiStage::Hexagon => 13,
            EmojiStage::Lucky => 17,
            EmojiStage::Cube => 19,
            EmojiStage::Magic => 23,
            EmojiStage::Decimal => 29,
            _ => 1,
        }
    }
    
    fn from_godel_number(n: u64) -> Option<Self> {
        match n {
            2 => Some(EmojiStage::Unity),
            3 => Some(EmojiStage::Prime),
            5 => Some(EmojiStage::Triangle),
            7 => Some(EmojiStage::Square),
            11 => Some(EmojiStage::Pentagon),
            13 => Some(EmojiStage::Hexagon),
            17 => Some(EmojiStage::Lucky),
            19 => Some(EmojiStage::Cube),
            23 => Some(EmojiStage::Magic),
            29 => Some(EmojiStage::Decimal),
            _ => None,
        }
    }
    
    fn all_known_godel_numbers() -> Vec<u64> {
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    }
    
    fn compose_godel(items: &[Self]) -> u64 {
        items.iter().map(|item| item.godel_number()).product()
    }
    
    fn decompose_godel(number: u64) -> Vec<Self> {
        let mut result = Vec::new();
        let mut remaining = number;
        
        for &prime in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29] {
            while remaining % prime == 0 {
                if let Some(stage) = Self::from_godel_number(prime) {
                    result.push(stage);
                }
                remaining /= prime;
            }
        }
        
        result
    }
    fn is_godel_prime(&self) -> bool {
        true
    }
    fn godel_factors(&self) -> Vec<u64> {
        vec![self.godel_number()]
    }
    fn godel_equivalent(&self, other: &Self) -> bool {
        self.godel_number() == other.godel_number()
    }
}