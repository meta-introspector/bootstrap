//! Stage0: Prelude - All 42 stages introduce themselves
//! Output from this stage becomes the foundation for stage1

// Import necessary modules for system commitment calculation
use bootstrap::system_commitment;
use bootstrap::kernel::Kernel;

fn main() {
    println!("=== STAGE 0: PRELUDE ===");
    println!("All 42 stages introducing themselves...\n");
    
    // Start with 42, which calls its factors, creating a hierarchical flow
    println!("Starting with stage 42 (the ultimate stage)...\n");
    
    // Call all 42 stages in sequence
    bootstrap::main42::main42();
    bootstrap::main41::main41();
    bootstrap::main40::main40();
    bootstrap::main39::main39();
    bootstrap::main38::main38();
    bootstrap::main37::main37();
    bootstrap::main36::main36();
    bootstrap::main35::main35();
    bootstrap::main34::main34();
    bootstrap::main33::main33();
    bootstrap::main32::main32();
    bootstrap::main31::main31();
    bootstrap::main30::main30();
    bootstrap::main29::main29();
    bootstrap::main28::main28();
    bootstrap::main27::main27();
    bootstrap::main26::main26();
    bootstrap::main25::main25();
    bootstrap::main24::main24();
    bootstrap::main23::main23();
    bootstrap::main22::main22();
    bootstrap::main21::main21();
    bootstrap::main20::main20();
    bootstrap::main19::main19();
    bootstrap::main18::main18();
    bootstrap::main17::main17();
    bootstrap::main16::main16();
    bootstrap::main15::main15();
    bootstrap::main14::main14();
    bootstrap::main13::main13();
    bootstrap::main12::main12();
    bootstrap::main11::main11();
    bootstrap::main10::main10();
    bootstrap::main09::main09();
    bootstrap::main08::main08();
    bootstrap::main07::main07();
    bootstrap::main06::main06();
    bootstrap::main05::main05();
    bootstrap::main04::main04();
    bootstrap::main03::main03();
    bootstrap::main02::main02();
    bootstrap::main01::main01();

    // Instantiate a Kernel to calculate the system commitment
    let mut kernel = Kernel::new();
    
    // Advance the kernel's cycle a few times to simulate system activity
    // This will affect the commitment value
    kernel.advance_cycle();
    kernel.advance_cycle();
    kernel.advance_cycle();

    // Calculate the system commitment value
    let commitment_value = system_commitment::calculate_system_commitment_value(&kernel);
    println!("\nSystem Commitment Value (Grand Polynomial): {}", commitment_value);

    println!("\n=== STAGE 0 COMPLETE ===");
    println!("Output captured for stage1 construction");
} 