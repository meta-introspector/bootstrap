use bootstrap::function_number_linkage::*;

fn main() {
    println!("=== Generic Apply Function Demo ===\n");

    // Create a function registry with some example functions
    let mut registry = FunctionRegistry::new();
    
    // Register some functions with their intrinsic numbers
    let function1 = IntrinsicFunction::new(
        42,
        "The Answer".to_string(),
        "Computes the meaning of life".to_string(),
        vec![42.0],
        vec![42.0],
        1.0,
        1.0,
    );
    
    let function2 = IntrinsicFunction::new(
        7,
        "Lucky Number".to_string(),
        "Multiplies input by 7".to_string(),
        vec![7.0],
        vec![7.0],
        0.8,
        0.9,
    );
    
    let function3 = IntrinsicFunction::new(
        13,
        "Unlucky Number".to_string(),
        "Adds 13 to input".to_string(),
        vec![13.0],
        vec![13.0],
        0.6,
        0.7,
    );
    
    registry.register_function(function1);
    registry.register_function(function2);
    registry.register_function(function3);
    
    // Create the language system
    let language = FunctionNumberLanguage::new(registry);
    
    println!("Registered functions:");
    for (number, function) in &language.registry.functions {
        println!("  {}: {}", number, function.name);
    }
    println!();
    
    // Demonstrate direct application
    println!("1. Direct application:");
    match language.registry.apply(42, vec![10.0]) {
        Ok(result) => println!("   apply(42, [10.0]) = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    match language.registry.apply(7, vec![5.0]) {
        Ok(result) => println!("   apply(7, [5.0]) = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();
    
    // Demonstrate chained application
    println!("2. Chained application:");
    let operations = vec![
        (7, vec![2.0]),
        (13, vec![0.0]),
        (42, vec![0.0]),
    ];
    
    match language.registry.chain_apply(operations) {
        Ok(result) => println!("   chain_apply([7, 13, 42]) = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();
    
    // Demonstrate resonant application
    println!("3. Resonant application:");
    match language.registry.apply_with_resonance(42, vec![1.0], 0.1) {
        Ok(results) => println!("   apply_with_resonance(42, [1.0], 0.1) = {:?}", results),
        Err(e) => println!("   Error: {}", e),
    }
    println!();
    
    // Demonstrate universal apply
    println!("4. Universal apply:");
    
    // Direct mode
    match language.registry.universal_apply(7, vec![3.0], ApplyMode::Direct) {
        Ok(ApplyResult::Value(result)) => println!("   universal_apply(7, Direct) = {}", result),
        Ok(_) => println!("   Unexpected result type"),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Resonant mode
    match language.registry.universal_apply(42, vec![1.0], ApplyMode::Resonant(0.05)) {
        Ok(ApplyResult::Multiple(results)) => println!("   universal_apply(42, Resonant) = {:?}", results),
        Ok(_) => println!("   Unexpected result type"),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Contextual mode
    match language.registry.universal_apply(13, vec![5.0], ApplyMode::Contextual("test".to_string())) {
        Ok(ApplyResult::Value(result)) => println!("   universal_apply(13, Contextual) = {}", result),
        Ok(_) => println!("   Unexpected result type"),
        Err(e) => println!("   Error: {}", e),
    }
    println!();
    
    // Demonstrate module linking
    println!("5. Module linking:");
    match language.link_module(42) {
        Ok(_) => println!("   Successfully linked module 42"),
        Err(e) => println!("   Error linking module 42: {}", e),
    }
    
    match language.link_modules(vec![7, 13, 42]) {
        Ok(_) => println!("   Successfully linked modules [7, 13, 42]"),
        Err(e) => println!("   Error linking modules: {}", e),
    }
    println!();
    
    // Demonstrate module chain execution
    println!("6. Module chain execution:");
    match language.execute_module_chain(vec![7, 13, 42], vec![2.0]) {
        Ok(results) => println!("   execute_module_chain([7, 13, 42], [2.0]) = {:?}", results),
        Err(e) => println!("   Error: {}", e),
    }
    println!();
    
    // Demonstrate resonant module linking
    println!("7. Resonant module linking:");
    match language.link_resonant_modules(42, 0.1) {
        Ok(linked) => println!("   link_resonant_modules(42, 0.1) = {:?}", linked),
        Err(e) => println!("   Error: {}", e),
    }
    println!();
    
    println!("=== Demo Complete ===");
} 