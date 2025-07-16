//! # Gauss Mathematical Module
//!
//! This module is dedicated to a collection of mathematical concepts and formulas
//! from the work of Carl Friedrich Gauss, with a focus on statistics, probability,
//! analysis, and differential geometry.
//!
//! ## Core Components
//!
//! - **`Gauss` Trait**: Defines an interface for various Gauss-related calculations,
//!   such as the Gaussian integral, normal distribution functions, Gaussian
//!   curvature, and basic statistical measures.
//! - **`Gaussian` Struct**: A concrete implementation of the `Gauss` trait.

/// A trait for calculations related to analysis, probability, and geometry,
/// inspired by the work of Carl Friedrich Gauss.
pub trait Gauss {
    // Analysis
    /// Calculates the Gaussian integral.
    fn gaussian_integral(&self, a: f64) -> f64;
    /// Calculates the error function (erf).
    fn error_function(&self, x: f64) -> f64;
    /// Calculates the probability density function (PDF) of the normal distribution.
    fn normal_pdf(&self, x: f64, mu: f64, sigma: f64) -> f64;
    /// Calculates the cumulative distribution function (CDF) of the normal distribution.
    fn normal_cdf(&self, x: f64, mu: f64, sigma: f64) -> f64;

    // Geometry
    /// Calculates the Gaussian curvature of a surface from its principal curvatures.
    fn gaussian_curvature(&self, k1: f64, k2: f64) -> f64;
    /// Applies the Gauss-Bonnet theorem, relating curvature to topology.
    fn gauss_bonnet(&self, total_curvature: f64, euler_characteristic: isize) -> f64;

    // Probability
    /// Calculates the arithmetic mean of a slice of data.
    fn mean(&self, data: &[f64]) -> f64;
    /// Calculates the variance of a slice of data.
    fn variance(&self, data: &[f64]) -> f64;
    /// Calculates the standard deviation of a slice of data.
    fn stddev(&self, data: &[f64]) -> f64;
}

/// A concrete implementation of the `Gauss` trait.
pub struct Gaussian;

impl Default for Gaussian {
    fn default() -> Self { Self }
}

impl Gauss for Gaussian {
    fn gaussian_integral(&self, a: f64) -> f64 {
        (std::f64::consts::PI).sqrt() * a
    }
    fn error_function(&self, x: f64) -> f64 {
        // Approximation
        (2.0 / std::f64::consts::PI.sqrt()) * x
    }
    fn normal_pdf(&self, x: f64, mu: f64, sigma: f64) -> f64 {
        let z = (x - mu) / sigma;
        (-(z * z) / 2.0).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
    }
    fn normal_cdf(&self, x: f64, mu: f64, sigma: f64) -> f64 {
        0.5 * (1.0 + (x - mu) / (sigma * 2.0_f64.sqrt()))
    }
    fn gaussian_curvature(&self, k1: f64, k2: f64) -> f64 {
        k1 * k2
    }
    fn gauss_bonnet(&self, total_curvature: f64, euler_characteristic: isize) -> f64 {
        total_curvature * euler_characteristic as f64
    }
    fn mean(&self, data: &[f64]) -> f64 {
        data.iter().sum::<f64>() / data.len() as f64
    }
    fn variance(&self, data: &[f64]) -> f64 {
        let m = self.mean(data);
        data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / data.len() as f64
    }
    fn stddev(&self, data: &[f64]) -> f64 {
        self.variance(data).sqrt()
    }
}