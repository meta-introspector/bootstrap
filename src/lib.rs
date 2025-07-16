#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

// Re-export all concept modules for documentation
pub mod godel;
pub mod system_commitment;
pub mod kernel;
pub mod emojistage;
pub mod bott;
pub mod clifford;
pub mod bach;
pub mod harmonic_lattice;
pub mod prime_stages;
pub mod fibonacci_stages;
pub mod pharmonic_mapping;
pub mod oeis_quasifibers;
pub mod abbott_periodicity;
pub mod inference_quasifibers;
pub mod provability;
pub mod generate_mains;
pub mod escher;
pub mod ns;
pub mod euler;
pub mod gauss;
pub mod mach;
pub mod penrose;
pub mod oeis;
pub mod vectos;
pub mod phase2;
pub mod number_embedding;
pub mod function_number_linkage;
pub mod phase_mapping;

// Re-export all stage modules
pub mod main01;
pub mod main02;
pub mod main03;
pub mod main04;
pub mod main05;
pub mod main06;
pub mod main07;
pub mod main08;
pub mod main09;
pub mod main10;
pub mod main11;
pub mod main12;
pub mod main13;
pub mod main14;
pub mod main15;
pub mod main16;
pub mod main17;
pub mod main18;
pub mod main19;
pub mod main20;
pub mod main21;
pub mod main22;
pub mod main23;
pub mod main24;
pub mod main25;
pub mod main26;
pub mod main27;
pub mod main28;
pub mod main29;
pub mod main30;
pub mod main31;
pub mod main32;
pub mod main33;
pub mod main34;
pub mod main35;
pub mod main36;
pub mod main37;
pub mod main38;
pub mod main39;
pub mod main40;
pub mod main41;
pub mod main42;

use solfunmeme_clifford::SolMultivector;

pub mod bootstrap_system;
pub mod stage_management;

pub use bootstrap_system::BootstrapSystem;
pub use stage_management::{run_stage0, get_all_stages, StageInfo};
 