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
        // Map number to EmojiStage (1-42 complete mapping)
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
            11 => EmojiStage::Network,
            12 => EmojiStage::Time,
            13 => EmojiStage::Unlucky,
            14 => EmojiStage::Composite,
            15 => EmojiStage::Pentadecagon,
            16 => EmojiStage::Hexadecagon,
            17 => EmojiStage::Prime17,
            18 => EmojiStage::Composite18,
            19 => EmojiStage::Prime19,
            20 => EmojiStage::Vigesimal,
            21 => EmojiStage::Triangle21,
            22 => EmojiStage::Composite22,
            23 => EmojiStage::Prime23,
            24 => EmojiStage::Factorial,
            25 => EmojiStage::Square25,
            26 => EmojiStage::Composite26,
            27 => EmojiStage::Prime27,
            28 => EmojiStage::Composite28,
            29 => EmojiStage::Prime29,
            30 => EmojiStage::Composite30,
            31 => EmojiStage::Prime31,
            32 => EmojiStage::Power32,
            33 => EmojiStage::Prime33,
            34 => EmojiStage::Composite34,
            35 => EmojiStage::Composite35,
            36 => EmojiStage::Square36,
            37 => EmojiStage::Prime37,
            38 => EmojiStage::Composite38,
            39 => EmojiStage::Prime39,
            40 => EmojiStage::Composite40,
            41 => EmojiStage::Prime41,
            42 => EmojiStage::TheWorld,
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
            "network" => EmojiStage::Network,
            "time" => EmojiStage::Time,
            "unlucky" => EmojiStage::Unlucky,
            "composite" => EmojiStage::Composite,
            "pentadecagon" => EmojiStage::Pentadecagon,
            "hexadecagon" => EmojiStage::Hexadecagon,
            "prime17" => EmojiStage::Prime17,
            "composite18" => EmojiStage::Composite18,
            "prime19" => EmojiStage::Prime19,
            "vigesimal" => EmojiStage::Vigesimal,
            "triangle21" => EmojiStage::Triangle21,
            "composite22" => EmojiStage::Composite22,
            "prime23" => EmojiStage::Prime23,
            "factorial" => EmojiStage::Factorial,
            "square25" => EmojiStage::Square25,
            "composite26" => EmojiStage::Composite26,
            "prime27" => EmojiStage::Prime27,
            "composite28" => EmojiStage::Composite28,
            "prime29" => EmojiStage::Prime29,
            "composite30" => EmojiStage::Composite30,
            "prime31" => EmojiStage::Prime31,
            "power32" => EmojiStage::Power32,
            "prime33" => EmojiStage::Prime33,
            "composite34" => EmojiStage::Composite34,
            "composite35" => EmojiStage::Composite35,
            "square36" => EmojiStage::Square36,
            "prime37" => EmojiStage::Prime37,
            "composite38" => EmojiStage::Composite38,
            "prime39" => EmojiStage::Prime39,
            "composite40" => EmojiStage::Composite40,
            "prime41" => EmojiStage::Prime41,
            "theworld" => EmojiStage::TheWorld,
            _ => EmojiStage::Unknown(input.clone()),
        }
    };
    println!("Stage: {}", stage);
    println!("Emoji: {}", stage.as_emoji());
    println!("Gödel number: {}", stage.godel_number());
    // Add more info as needed (vibe, properties, etc.)
}