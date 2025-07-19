use anyhow::Result;
use bootstrap::run_stage0;
use bootstrap::BootstrapSystem;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut system = BootstrapSystem::new();
    run_stage0(&mut system)?;

    // Now, emit RDF using solfunmeme_rdf_utils
    let rdf_path = PathBuf::from("ontologies/zos/stage_prime_vibes_stage1.ttl");
    let mut graph = RdfGraph::new();

    // Example: Add a triple to the graph
    graph.add_triple(
        "http://example.org/stage1_rdf",
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
        "http://example.org/BootstrappingPhase",
    )?;

    // Serialize and write to file
    fs::write(&rdf_path, graph.serialize_to_turtle_string()?.as_bytes())?;

    println!("RDF emitted to {:?}", rdf_path);

    Ok(())
}
