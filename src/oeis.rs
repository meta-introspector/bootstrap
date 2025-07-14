/// OEIS trait for Online Encyclopedia of Integer Sequences operations
pub trait OEIS {
    // Core sequence operations
    fn generate_sequence(&self, sequence_id: &str, length: usize) -> Vec<i64>;
    fn sequence_name(&self, sequence_id: &str) -> String;
    fn sequence_description(&self, sequence_id: &str) -> String;
    
    // Mathematical sequence generators
    fn fibonacci_sequence(&self, n: usize) -> Vec<i64>;
    fn prime_sequence(&self, n: usize) -> Vec<i64>;
    fn triangular_numbers(&self, n: usize) -> Vec<i64>;
    fn square_numbers(&self, n: usize) -> Vec<i64>;
    fn catalan_numbers(&self, n: usize) -> Vec<i64>;
    fn bell_numbers(&self, n: usize) -> Vec<i64>;
    fn stirling_numbers(&self, n: usize, k: usize) -> Vec<Vec<i64>>;
    
    // Sequence analysis
    fn sequence_growth_rate(&self, sequence: &[i64]) -> f64;
    fn sequence_pattern(&self, sequence: &[i64]) -> String;
    fn find_recurrence(&self, sequence: &[i64]) -> Option<String>;
    fn sequence_divergence(&self, sequence: &[i64]) -> bool;
    
    // Sequence transformations
    fn cumulative_sum(&self, sequence: &[i64]) -> Vec<i64>;
    fn differences(&self, sequence: &[i64]) -> Vec<i64>;
    fn convolution(&self, seq1: &[i64], seq2: &[i64]) -> Vec<i64>;
    fn binomial_transform(&self, sequence: &[i64]) -> Vec<i64>;
    
    // Famous sequences
    fn a000045(&self, n: usize) -> Vec<i64>; // Fibonacci
    fn a000040(&self, n: usize) -> Vec<i64>; // Primes
    fn a000217(&self, n: usize) -> Vec<i64>; // Triangular
    fn a000290(&self, n: usize) -> Vec<i64>; // Squares
    fn a000108(&self, n: usize) -> Vec<i64>; // Catalan
    fn a000110(&self, n: usize) -> Vec<i64>; // Bell
    fn a000142(&self, n: usize) -> Vec<i64>; // Factorials
    fn a000720(&self, n: usize) -> Vec<i64>; // Prime counting
    fn a000041(&self, n: usize) -> Vec<i64>; // Partition numbers
    fn a000079(&self, n: usize) -> Vec<i64>; // Powers of 2
    
    // Helper methods
    fn is_prime(&self, n: i64) -> bool;
    fn binomial_coefficient(&self, n: usize, k: usize) -> i64;
}

pub struct OEISDatabase;

impl Default for OEISDatabase {
    fn default() -> Self { Self }
}

impl OEIS for OEISDatabase {
    fn generate_sequence(&self, sequence_id: &str, length: usize) -> Vec<i64> {
        match sequence_id {
            "A000045" => self.a000045(length),
            "A000040" => self.a000040(length),
            "A000217" => self.a000217(length),
            "A000290" => self.a000290(length),
            "A000108" => self.a000108(length),
            "A000110" => self.a000110(length),
            "A000142" => self.a000142(length),
            "A000720" => self.a000720(length),
            "A000041" => self.a000041(length),
            "A000079" => self.a000079(length),
            _ => vec![0; length],
        }
    }
    
    fn sequence_name(&self, sequence_id: &str) -> String {
        match sequence_id {
            "A000045" => "Fibonacci numbers".to_string(),
            "A000040" => "Prime numbers".to_string(),
            "A000217" => "Triangular numbers".to_string(),
            "A000290" => "Square numbers".to_string(),
            "A000108" => "Catalan numbers".to_string(),
            "A000110" => "Bell numbers".to_string(),
            "A000142" => "Factorial numbers".to_string(),
            "A000720" => "Prime counting function".to_string(),
            "A000041" => "Partition numbers".to_string(),
            "A000079" => "Powers of 2".to_string(),
            _ => "Unknown sequence".to_string(),
        }
    }
    
    fn sequence_description(&self, sequence_id: &str) -> String {
        match sequence_id {
            "A000045" => "F(n) = F(n-1) + F(n-2) with F(0) = 0, F(1) = 1".to_string(),
            "A000040" => "The prime numbers: numbers n > 1 that have no positive divisors other than 1 and n".to_string(),
            "A000217" => "Triangular numbers: a(n) = binomial(n+1,2) = n(n+1)/2".to_string(),
            "A000290" => "The squares: a(n) = n^2".to_string(),
            "A000108" => "Catalan numbers: C(n) = binomial(2n,n)/(n+1)".to_string(),
            "A000110" => "Bell numbers: number of partitions of a set of n labeled elements".to_string(),
            "A000142" => "Factorial numbers: n! = 1*2*3*4*...*n".to_string(),
            "A000720" => "pi(n), the number of primes <= n".to_string(),
            "A000041" => "a(n) is the number of partitions of n".to_string(),
            "A000079" => "Powers of 2: a(n) = 2^n".to_string(),
            _ => "No description available".to_string(),
        }
    }
    
    fn fibonacci_sequence(&self, n: usize) -> Vec<i64> {
        self.a000045(n)
    }
    
    fn prime_sequence(&self, n: usize) -> Vec<i64> {
        self.a000040(n)
    }
    
    fn triangular_numbers(&self, n: usize) -> Vec<i64> {
        self.a000217(n)
    }
    
    fn square_numbers(&self, n: usize) -> Vec<i64> {
        self.a000290(n)
    }
    
    fn catalan_numbers(&self, n: usize) -> Vec<i64> {
        self.a000108(n)
    }
    
    fn bell_numbers(&self, n: usize) -> Vec<i64> {
        self.a000110(n)
    }
    
    fn stirling_numbers(&self, n: usize, k: usize) -> Vec<Vec<i64>> {
        let mut stirling = vec![vec![0; k + 1]; n + 1];
        stirling[0][0] = 1;
        
        for i in 1..=n {
            for j in 1..=k {
                stirling[i][j] = j as i64 * stirling[i-1][j] + stirling[i-1][j-1];
            }
        }
        stirling
    }
    
    fn sequence_growth_rate(&self, sequence: &[i64]) -> f64 {
        if sequence.len() < 2 {
            return 0.0;
        }
        let mut ratios = Vec::new();
        for i in 1..sequence.len() {
            if sequence[i-1] != 0 {
                ratios.push(sequence[i] as f64 / sequence[i-1] as f64);
            }
        }
        if ratios.is_empty() {
            return 0.0;
        }
        ratios.iter().sum::<f64>() / ratios.len() as f64
    }
    
    fn sequence_pattern(&self, sequence: &[i64]) -> String {
        if sequence.len() < 3 {
            return "Too short to analyze".to_string();
        }
        
        let differences = self.differences(sequence);
        if differences.iter().all(|&x| x == differences[0]) {
            return "Arithmetic sequence".to_string();
        }
        
        let ratios: Vec<f64> = sequence.windows(2)
            .map(|w| if w[0] != 0 { w[1] as f64 / w[0] as f64 } else { 0.0 })
            .collect();
        
        if ratios.iter().all(|&x| x == ratios[0]) {
            return "Geometric sequence".to_string();
        }
        
        "Complex pattern".to_string()
    }
    
    fn find_recurrence(&self, sequence: &[i64]) -> Option<String> {
        if sequence.len() < 4 {
            return None;
        }
        
        // Check for linear recurrence of order 2: a(n) = c1*a(n-1) + c2*a(n-2)
        let a = sequence[0] as f64;
        let b = sequence[1] as f64;
        let c = sequence[2] as f64;
        let d = sequence[3] as f64;
        
        // Solve: c = c1*b + c2*a and d = c1*c + c2*b
        let det = b * c - a * d;
        if det.abs() > 1e-10 {
            let c1 = (c * c - b * d) / det;
            let c2 = (b * c - a * d) / det;
            Some(format!("a(n) = {}*a(n-1) + {}*a(n-2)", c1, c2))
        } else {
            None
        }
    }
    
    fn sequence_divergence(&self, sequence: &[i64]) -> bool {
        if sequence.len() < 2 {
            return false;
        }
        
        let last_few = &sequence[sequence.len().saturating_sub(5)..];
        last_few.windows(2).any(|w| (w[1] - w[0]).abs() > 1000)
    }
    
    fn cumulative_sum(&self, sequence: &[i64]) -> Vec<i64> {
        let mut sum = 0;
        sequence.iter().map(|&x| {
            sum += x;
            sum
        }).collect()
    }
    
    fn differences(&self, sequence: &[i64]) -> Vec<i64> {
        sequence.windows(2).map(|w| w[1] - w[0]).collect()
    }
    
    fn convolution(&self, seq1: &[i64], seq2: &[i64]) -> Vec<i64> {
        let n = seq1.len() + seq2.len() - 1;
        let mut result = vec![0; n];
        
        for i in 0..seq1.len() {
            for j in 0..seq2.len() {
                result[i + j] += seq1[i] * seq2[j];
            }
        }
        result
    }
    
    fn binomial_transform(&self, sequence: &[i64]) -> Vec<i64> {
        let n = sequence.len();
        let mut result = vec![0; n];
        
        for i in 0..n {
            for j in 0..=i {
                result[i] += sequence[j] * self.binomial_coefficient(i, j);
            }
        }
        result
    }
    
    fn a000045(&self, n: usize) -> Vec<i64> {
        let mut fib = vec![0; n];
        if n > 0 { fib[0] = 0; }
        if n > 1 { fib[1] = 1; }
        for i in 2..n {
            fib[i] = fib[i-1] + fib[i-2];
        }
        fib
    }
    
    fn a000040(&self, n: usize) -> Vec<i64> {
        let mut primes = Vec::new();
        let mut num = 2;
        while primes.len() < n {
            if self.is_prime(num) {
                primes.push(num);
            }
            num += 1;
        }
        primes
    }
    
    fn a000217(&self, n: usize) -> Vec<i64> {
        (0..n).map(|i| (i * (i + 1)) as i64 / 2).collect()
    }
    
    fn a000290(&self, n: usize) -> Vec<i64> {
        (0..n).map(|i| (i * i) as i64).collect()
    }
    
    fn a000108(&self, n: usize) -> Vec<i64> {
        let mut catalan = vec![0; n];
        if n > 0 { catalan[0] = 1; }
        for i in 1..n {
            for j in 0..i {
                catalan[i] += catalan[j] * catalan[i-1-j];
            }
        }
        catalan
    }
    
    fn a000110(&self, n: usize) -> Vec<i64> {
        let mut bell = vec![0; n];
        if n > 0 { bell[0] = 1; }
        for i in 1..n {
            bell[i] = 1;
            for j in (1..=i).rev() {
                bell[j-1] += bell[j];
            }
        }
        bell
    }
    
    fn a000142(&self, n: usize) -> Vec<i64> {
        let mut factorial = vec![1; n];
        for i in 1..n {
            factorial[i] = factorial[i-1] * (i as i64);
        }
        factorial
    }
    
    fn a000720(&self, n: usize) -> Vec<i64> {
        let mut pi = vec![0; n];
        for i in 2..n {
            pi[i] = pi[i-1] + if self.is_prime(i as i64) { 1 } else { 0 };
        }
        pi
    }
    
    fn a000041(&self, n: usize) -> Vec<i64> {
        let mut partitions = vec![0; n];
        partitions[0] = 1;
        for i in 1..n {
            let mut sum = 0;
            let mut k = 1;
            let mut j = k * (3 * k - 1) / 2;
            while j <= i {
                let sign = if k % 2 == 0 { -1 } else { 1 };
                sum += sign * partitions[i - j];
                k = if k > 0 { 
                    if k <= i32::MAX as usize { -(k as i32) as usize } else { 0 }
                } else { 
                    if k <= i32::MAX as usize { (-(k as i32) + 1) as usize } else { 1 }
                };
                j = k * (3 * k - 1) / 2;
            }
            partitions[i] = sum;
        }
        partitions
    }
    
    fn a000079(&self, n: usize) -> Vec<i64> {
        (0..n).map(|i| 1 << i).collect()
    }
    
    // Helper methods
    fn is_prime(&self, n: i64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        let sqrt_n = (n as f64).sqrt() as i64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
    
    fn binomial_coefficient(&self, n: usize, k: usize) -> i64 {
        if k > n { return 0; }
        if k == 0 || k == n { return 1; }
        let mut result = 1;
        for i in 0..k {
            result = result * (n - i) as i64 / (i + 1) as i64;
        }
        result
    }
} 