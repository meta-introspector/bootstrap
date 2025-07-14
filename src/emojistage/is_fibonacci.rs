pub fn is_fibonacci(n: u64) -> bool {
    let mut a = 0u64;
    let mut b = 1u64;
    while b <= n {
        if b == n { return true; }
        let temp = a + b;
        a = b;
        b = temp;
    }
    false
} 