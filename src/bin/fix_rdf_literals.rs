//! This binary is a work in progress for automatically fixing RDF Turtle files.
//! Currently, it performs an aggressive string-based escaping of double quotes,
//! which can lead to over-escaping. It is intended for future development to
//! correctly parse and re-serialize RDF literals.

use anyhow::Result;
use std::path::Path;
use std::fs;

fn main() -> Result<()> {
    let path = Path::new("ontologies/zos/prime_numbers.ttl");
    println!("Attempting to aggressively escape literals in {:?}...", path);

    let content = fs::read_to_string(path)?;
    let corrected_content = content.replace("\"", "\\\"");

    fs::write(path, corrected_content)?;
    println!("Successfully aggressively escaped literals in {:?}", path);
    Ok(())
}
