//! # Bootstrap Binary
//!
//! This is the main entry point for the bootstrap system. It executes the "Stage 0 Prelude,"
//! a process where all 42 stages of the system introduce themselves and their mathematical properties.
//!
//! The output of this binary is a foundational report that can be used as input for subsequent stages
//! of the system's lifecycle. It concludes by calculating a "System Commitment Value," a unique
//! signature representing the state of the entire system.
//!
//! Copyright (C) 2025 James Michael Dupont <h4@solfunmeme.com>
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Affero General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Affero General Public License for more details.
//!
//! You should have received a copy of the GNU Affero General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Import necessary modules for system commitment calculation
use bootstrap::bootstrap_system::BootstrapSystem;

fn main() {
    let mut system = BootstrapSystem::new();
    if let Err(e) = system.run() {
        eprintln!("Bootstrap system encountered an error: {}", e);
    }
} 