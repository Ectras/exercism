pub fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if (n % i == 0 || n % (i + 2) == 0) {
            return false;
        }
        i += 6;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut counter = 0u32;
    let mut i = 1u32;
    while counter < n + 1 {
        i += 1;
        if is_prime(i) {
            counter += 1;
        }
    }
    i
}
