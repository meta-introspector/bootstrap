//! Stage0: Prelude - All 42 stages introduce themselves
//! Output from this stage becomes the foundation for stage1

// Import all concept modules
mod harmonic_lattice;
mod prime_stages;
mod fibonacci_stages;
mod pharmonic_mapping;
mod oeis_quasifibers;
mod abbott_periodicity;
mod inference_quasifibers;
mod provability;
mod generate_mains;

// Import all 42 main modules
mod main01; mod main02; mod main03; mod main04; mod main05; mod main06; mod main07; mod main08; mod main09; mod main10;
mod main11; mod main12; mod main13; mod main14; mod main15; mod main16; mod main17; mod main18; mod main19; mod main20;
mod main21; mod main22; mod main23; mod main24; mod main25; mod main26; mod main27; mod main28; mod main29; mod main30;
mod main31; mod main32; mod main33; mod main34; mod main35; mod main36; mod main37; mod main38; mod main39; mod main40;
mod main41; mod main42;

fn main() {
    println!("=== STAGE 0: PRELUDE ===");
    println!("Generating all 42 main modules...\n");
    
    // Generate all main modules first
    if let Err(e) = generate_mains::generate_all_mains() {
        eprintln!("Error generating mains: {}", e);
        return;
    }
    
    println!("\nAll 42 stages introducing themselves...\n");
    
    // Start with 42, which calls its factors, creating a hierarchical flow
    println!("Starting with stage 42 (the ultimate stage)...\n");
    main42::main42();
    
    println!("\n=== STAGE 0 COMPLETE ===");
    println!("Output captured for stage1 construction");
} 