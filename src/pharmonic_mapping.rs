
//! # Pharmonic Mapping
//!
//! Pharmonic Mapping relates every integer from 1 to 42 to the resonant skeleton (factors, primes, Fibonacci numbers) via harmonic, prime, or Fibonacci relationships.
//! Each number is contextualized by its connections to these core sets, forming a fully connected pharmonic lattice.
//! This mapping enables the system to traverse or reference any state through its relationship to the skeleton.

/// Returns all numbers from 1 to 42 and their relationships to the skeleton sets.
pub fn pharmonic_map() -> Vec<(u32, Vec<&'static str>)> {
    let factors = vec![1, 2, 3, 6, 7, 14, 21, 42];
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    let fibs = vec![1, 2, 3, 5, 8, 13, 21, 34];
    (1..=42)
        .map(|n| {
            let mut tags = Vec::new();
            if factors.contains(&n) { tags.push("factor"); }
            if primes.contains(&n) { tags.push("prime"); }
            if fibs.contains(&n) { tags.push("fibonacci"); }
            (n, tags)
        })
        .collect()
} 