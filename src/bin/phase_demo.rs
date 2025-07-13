use bootstrap::phase_mapping::*;

fn main() {
    println!("=== Phase Mapping Demo ===\n");

    // Create different reducers
    let hash_reducer = Box::new(HashReducer);
    let harmonic_reducer = Box::new(HarmonicReducer);
    
    // Create sample embeddings for different types of entities
    let sample_embeddings = vec![
        vec![0.1, 0.2, 0.3, 0.4, 0.5], // Function embedding
        vec![0.8, 0.7, 0.6, 0.5, 0.4], // Module embedding
        vec![0.3, 0.1, 0.9, 0.2, 0.8], // Trait embedding
        vec![0.5, 0.5, 0.5, 0.5, 0.5], // Balanced embedding
        vec![0.9, 0.1, 0.1, 0.1, 0.1], // Skewed embedding
    ];

    // Create sample entities
    let entities = vec![
        FunctionEntity {
            name: "analyze_project".to_string(),
            embedding: sample_embeddings[0].clone(),
            semantic_type: "function".to_string(),
        },
        FunctionEntity {
            name: "calculate_distance".to_string(),
            embedding: sample_embeddings[1].clone(),
            semantic_type: "function".to_string(),
        },
        ModuleEntity {
            name: "phase_mapping".to_string(),
            embedding: sample_embeddings[2].clone(),
            functions: vec!["map_entity".to_string(), "get_phase_statistics".to_string()],
        },
        FunctionEntity {
            name: "harmonic_reducer".to_string(),
            embedding: sample_embeddings[3].clone(),
            semantic_type: "reducer".to_string(),
        },
        FunctionEntity {
            name: "hash_reducer".to_string(),
            embedding: sample_embeddings[4].clone(),
            semantic_type: "reducer".to_string(),
        },
    ];

    println!("1. Testing Hash-based Phase Mapping:");
    let mut hash_system = PhaseMappingSystem::new(hash_reducer);
    
    for entity in &entities {
        let phase = hash_system.map_entity(entity);
        let confidence = hash_system.get_mapping_confidence(entity, phase);
        println!("   {} -> Phase {} (confidence: {:.3})", 
                entity.get_name(), phase.value(), confidence);
    }
    println!();

    println!("2. Testing Harmonic Phase Mapping:");
    let mut harmonic_system = PhaseMappingSystem::new(harmonic_reducer);
    
    for entity in &entities {
        let phase = harmonic_system.map_entity(entity);
        let confidence = harmonic_system.get_mapping_confidence(entity, phase);
        println!("   {} -> Phase {} (confidence: {:.3})", 
                entity.get_name(), phase.value(), confidence);
    }
    println!();

    println!("3. Phase Properties Analysis:");
    for phase_num in [1, 2, 3, 5, 7, 8, 13, 21, 34, 42] {
        let phase = Phase::new(phase_num).unwrap();
        let props = phase.properties();
        println!("   Phase {}: prime={}, fibonacci={}, square={}, factors={:?}, resonance={:.3}",
                phase_num, props.is_prime, props.is_fibonacci, props.is_perfect_square, 
                props.factors, props.resonance_frequency);
    }
    println!();

    println!("4. Phase Statistics:");
    let stats = harmonic_system.get_phase_statistics();
    
    // Show statistics for phases that have entities
    for (phase, stat) in stats.iter() {
        if stat.entity_count > 0 {
            println!("   Phase {}: {} entities", phase.value(), stat.entity_count);
            println!("     Entities: {:?}", stat.entities);
            println!("     Resonance: {:.3}", stat.properties.resonance_frequency);
        }
    }
    println!();

    println!("5. Resonant Entity Discovery:");
    let target_phase = Phase::new(7).unwrap(); // Prime phase
    let resonant_entities = harmonic_system.find_resonant_entities(target_phase, 0.3);
    println!("   Entities resonant with Phase {} (threshold 0.3):", target_phase.value());
    for entity_name in resonant_entities {
        println!("     - {}", entity_name);
    }
    println!();

    println!("6. Cross-Phase Resonance Analysis:");
    for phase_num in [1, 7, 13, 21, 42] {
        let phase = Phase::new(phase_num).unwrap();
        let entities_in_phase = harmonic_system.get_phase_entities(phase);
        if !entities_in_phase.is_empty() {
            println!("   Phase {} entities: {:?}", phase_num, entities_in_phase);
        }
    }
    println!();

    println!("7. Mathematical Phase Relationships:");
    let prime_phases: Vec<Phase> = (1..=42)
        .filter_map(|n| Phase::new(n))
        .filter(|p| p.properties().is_prime)
        .collect();
    
    let fibonacci_phases: Vec<Phase> = (1..=42)
        .filter_map(|n| Phase::new(n))
        .filter(|p| p.properties().is_fibonacci)
        .collect();
    
    println!("   Prime phases: {:?}", prime_phases.iter().map(|p| p.value()).collect::<Vec<_>>());
    println!("   Fibonacci phases: {:?}", fibonacci_phases.iter().map(|p| p.value()).collect::<Vec<_>>());
    
    // Find intersection
    let intersection: Vec<u8> = prime_phases.iter()
        .filter(|p| fibonacci_phases.contains(p))
        .map(|p| p.value())
        .collect();
    println!("   Prime + Fibonacci phases: {:?}", intersection);
    println!();

    println!("8. Dimensionality Reduction Comparison:");
    println!("   Comparing different reduction methods for the same entity:");
    let test_entity = &entities[0]; // analyze_project
    
    let hash_phase = hash_system.get_entity_phase(&test_entity.get_name()).unwrap();
    let harmonic_phase = harmonic_system.get_entity_phase(&test_entity.get_name()).unwrap();
    
    let hash_conf = hash_system.get_mapping_confidence(test_entity, hash_phase);
    let harmonic_conf = harmonic_system.get_mapping_confidence(test_entity, harmonic_phase);
    
    println!("   Entity: {}", test_entity.get_name());
    println!("   Hash reducer: Phase {} (confidence: {:.3})", hash_phase.value(), hash_conf);
    println!("   Harmonic reducer: Phase {} (confidence: {:.3})", harmonic_phase.value(), harmonic_conf);
    println!();

    println!("=== Demo Complete ===");
    println!("\nKey Insights:");
    println!("- Hash-based mapping provides deterministic, consistent phase assignments");
    println!("- Harmonic mapping considers mathematical resonance properties");
    println!("- Prime phases (2,3,5,7,11,13,17,19,23,29,31,37,41) have higher resonance");
    println!("- Fibonacci phases (1,1,2,3,5,8,13,21,34) have special mathematical properties");
    println!("- The system can discover resonant relationships between entities across phases");
    println!("- Each phase has unique mathematical properties that influence entity placement");
} 