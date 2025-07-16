use std::env;
use bootstrap::emojistage::EmojiStage;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: abstract_stage <stage_number|name|emoji>");
        std::process::exit(1);
    }
    let input = &args[1];
    let stage = if let Ok(num) = input.parse::<u8>() {
        // Map number to EmojiStage (1-10 for demo, expand as needed)
        match num {
            1 => EmojiStage::Unity,
            2 => EmojiStage::Prime,
            3 => EmojiStage::Triangle,
            4 => EmojiStage::Square,
            5 => EmojiStage::Pentagon,
            6 => EmojiStage::Hexagon,
            7 => EmojiStage::Lucky,
            8 => EmojiStage::Cube,
            9 => EmojiStage::Magic,
            10 => EmojiStage::Decimal,
            _ => EmojiStage::Unknown(input.clone()),
        }
    } else if input.chars().count() == 1 || input.chars().count() == 2 {
        EmojiStage::from_emoji(input)
    } else {
        // Try to match by name (case-insensitive)
        match input.to_lowercase().as_str() {
            "unity" => EmojiStage::Unity,
            "prime" => EmojiStage::Prime,
            "triangle" => EmojiStage::Triangle,
            "square" => EmojiStage::Square,
            "pentagon" => EmojiStage::Pentagon,
            "hexagon" => EmojiStage::Hexagon,
            "lucky" => EmojiStage::Lucky,
            "cube" => EmojiStage::Cube,
            "magic" => EmojiStage::Magic,
            "decimal" => EmojiStage::Decimal,
            _ => EmojiStage::Unknown(input.clone()),
        }
    };
    println!("Stage: {}", stage);
    println!("Emoji: {}", stage.as_emoji());
    println!("Gödel number: {}", stage.godel_number());
    // Add more info as needed (vibe, properties, etc.)
}