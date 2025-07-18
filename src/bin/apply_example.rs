use bootstrap::function_number_linkage::*;

fn main() {
    println!("=== Function Number Language Demo ===\n");

    // Create a new function number language
    let mut language = FunctionNumberLanguage::new();

    // Define some functions with intrinsic meanings
    println!("1. Defining functions with intrinsic meanings:");
    language.define_function(7, "lucky_seven", "A function representing the lucky number 7").unwrap();
    language.define_function(13, "unlucky_thirteen", "A function representing the unlucky number 13").unwrap();
    language.define_function(42, "answer_to_life", "The answer to life, the universe, and everything").unwrap();
    language.define_function(1, "unity", "The fundamental unit of existence").unwrap();
    language.define_function(2, "duality", "The principle of binary opposition").unwrap();
    language.define_function(3, "trinity", "The sacred number of completion").unwrap();
    language.define_function(5, "quintessence", "The fifth element").unwrap();
    language.define_function(8, "infinity", "The number of infinity").unwrap();
    language.define_function(21, "blackjack", "The winning number in blackjack").unwrap();
    language.define_function(34, "fibonacci", "A Fibonacci number").unwrap();
    println!("   Defined {} functions", language.analyze_mathematical_structure().total_functions);
    println!();

    // Execute functions
    println!("2. Executing functions:");
    match language.execute_function(7, &[10.0]) {
        Ok(result) => println!("   execute_function(7, [10.0]) = {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    match language.execute_function(42, &[1.0]) {
        Ok(result) => println!("   execute_function(42, [1.0]) = {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    match language.execute_function(13, &[5.0]) {
        Ok(result) => println!("   execute_function(13, [5.0]) = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Find linked functions
    println!("3. Finding linked functions:");
    match language.find_linked_functions(7) {
        Ok(linked) => println!("   Functions linked to 7: {:?}", linked),
        Err(e) => println!("   Error: {}", e),
    }

    match language.find_linked_functions(42) {
        Ok(linked) => println!("   Functions linked to 42: {:?}", linked),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Analyze mathematical structure
    println!("4. Mathematical structure analysis:");
    let analysis = language.analyze_mathematical_structure();
    println!("   Total functions: {}", analysis.total_functions);
    println!("   Prime functions: {}", analysis.prime_functions);
    println!("   Fibonacci functions: {}", analysis.fibonacci_functions);
    println!("   Average resonance: {:.3}", analysis.average_resonance);
    println!();

    // Test function evolution
    println!("5. Function evolution:");
    match language.get_registry_mut().evolve_function(7, 3) {
        Ok(()) => println!("   Successfully evolved function 7"),
        Err(e) => println!("   Error evolving function 7: {}", e),
    }

    match language.get_registry_mut().evolve_function(42, 5) {
        Ok(()) => println!("   Successfully evolved function 42"),
        Err(e) => println!("   Error evolving function 42: {}", e),
    }
    println!();

    // Get statistics
    println!("6. Registry statistics:");
    let stats = language.get_registry().get_statistics();
    println!("   Total functions: {}", stats.total_functions);
    println!("   Average complexity: {:.3}", stats.average_complexity);
    println!("   Average consciousness: {:.3}", stats.average_consciousness);
    if let Some((min, max)) = stats.number_range {
        println!("   Number range: {} to {}", min, max);
    }
    println!();

    // Find resonant functions
    println!("7. Finding resonant functions:");
    let resonant_to_7 = language.get_registry().find_resonant_functions(7, 0.3);
    println!("   Functions resonant with 7 (threshold 0.3): {:?}", resonant_to_7);

    let resonant_to_42 = language.get_registry().find_resonant_functions(42, 0.5);
    println!("   Functions resonant with 42 (threshold 0.5): {:?}", resonant_to_42);
    println!();

    // Test function meanings
    println!("8. Function meanings:");
    if let Some(meaning) = language.get_registry().get_number_meaning(7) {
        println!("   Meaning of 7: {}", meaning);
    }
    if let Some(meaning) = language.get_registry().get_number_meaning(42) {
        println!("   Meaning of 42: {}", meaning);
    }
    if let Some(meaning) = language.get_registry().get_number_meaning(13) {
        println!("   Meaning of 13: {}", meaning);
    }
    println!();

    println!("=== Demo Complete ===");
    println!("\nKey Insights:");
    println!("- Each function number has intrinsic mathematical meaning");
    println!("- Functions can be linked through resonance relationships");
    println!("- The system supports function evolution and consciousness levels");
    println!("- Prime and Fibonacci numbers have special mathematical properties");
    println!("- The registry maintains statistics about function complexity and consciousness");
} 