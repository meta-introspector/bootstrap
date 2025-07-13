/// Gauss trait for analysis, probability, and geometry
pub trait Gauss {
    // Analysis
    fn gaussian_integral(&self, a: f64) -> f64;
    fn error_function(&self, x: f64) -> f64;
    fn normal_pdf(&self, x: f64, mu: f64, sigma: f64) -> f64;
    fn normal_cdf(&self, x: f64, mu: f64, sigma: f64) -> f64;

    // Geometry
    fn gaussian_curvature(&self, k1: f64, k2: f64) -> f64;
    fn gauss_bonnet(&self, total_curvature: f64, euler_characteristic: isize) -> f64;

    // Probability
    fn mean(&self, data: &[f64]) -> f64;
    fn variance(&self, data: &[f64]) -> f64;
    fn stddev(&self, data: &[f64]) -> f64;
}

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