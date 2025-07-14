//! The core kernel that orchestrates the bootstrap system

/// The core kernel that manages system state
///
/// The kernel maintains a 42-step cycle that advances with each
/// operation, providing a sense of system evolution and renewal.
pub struct Kernel {
    /// Current step in the 42-step cycle (0-41)
    pub step: u64,
}

impl Kernel {
    /// Creates a new kernel
    pub fn new() -> Self {
        Self {
            step: 0,
        }
    }

    /// Returns the current cycle step (0-41)
    pub fn cycle_step(&self) -> u64 {
        self.step
    }

    /// Advances the cycle by one step, wrapping at 42
    pub fn advance_cycle(&mut self) {
        self.step = (self.step + 1) % 42;
    }

    /// Sets the cycle step
    pub fn set_step(&mut self, step: u64) {
        self.step = step % 42;
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_creation() {
        let kernel = Kernel::new();
        assert_eq!(kernel.cycle_step(), 0);
    }

    #[test]
    fn test_cycle_advancement() {
        let mut kernel = Kernel::new();
        assert_eq!(kernel.cycle_step(), 0);
        
        kernel.advance_cycle();
        assert_eq!(kernel.cycle_step(), 1);
        
        kernel.advance_cycle();
        assert_eq!(kernel.cycle_step(), 2);
    }

    #[test]
    fn test_cycle_wrapping() {
        let mut kernel = Kernel::new();
        
        // Advance to step 41
        for _ in 0..41 {
            kernel.advance_cycle();
        }
        assert_eq!(kernel.cycle_step(), 41);
        
        // Next operation should wrap to 0
        kernel.advance_cycle();
        assert_eq!(kernel.cycle_step(), 0);
    }

    #[test]
    fn test_set_step() {
        let mut kernel = Kernel::new();
        kernel.set_step(25);
        assert_eq!(kernel.cycle_step(), 25);
        
        // Test wrapping
        kernel.set_step(50);
        assert_eq!(kernel.cycle_step(), 8); // 50 % 42 = 8
    }
} 