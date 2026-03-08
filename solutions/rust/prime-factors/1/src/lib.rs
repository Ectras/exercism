pub fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn next_prime(start: u64) -> u64 {
    let mut i = start;
    while !is_prime(i) {
        i += 1;
    }
    i
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut primes = Vec::new();
    let mut nm = n;
    while nm > 1 {
        let mut prime = 2u64;

        // First, check the prime numbers in the cache
        let mut i = 0;
        while nm % prime != 0 && i < primes.len() {
            prime = primes[i];
            i += 1;
        }

        // Then generate new prime numbers
        while nm % prime != 0 {
            prime = next_prime(prime + 1);
            primes.push(prime);
        }

        // Divide by the found prime factor
        nm /= prime;
        factors.push(prime);
    };
    factors
}
