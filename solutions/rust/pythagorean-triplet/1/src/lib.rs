use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    let mut results = HashSet::new();
    for b in 1..n / 2 {
        let denom = 2 * (n - b);
        let num = n * (n - 2 * b);
        let a = num / denom;
        let c = n - a - b;
        if !(a < b && b < c) {
            continue;
        }
        if a * a + b * b != c * c {
            continue;
        }
        results.insert([a, b, c]);
    }
    results
}

// Two conditions:
// (1): a + b + c = N
// (2): a^2 + b^2 = c^2

// Rewrite (1):
// (N - a - b)^2 = c^2

// Equal (1) and (2) (both are equal to c^2):
// (N - a - b)^2 = a^2 + b^2

// Simplify the expression:
// a^2 + 2ab - 2aN + b^2 - 2bN + N^2 = a^2 + b^2
// 2ab - 2aN - 2bN + N^2 = 0
// 2a(b - N) - 2bN + N^2 = 0
// a = (2bN - N^2) / (2b - 2N)
// a = (N^2 - 2bN) / (2N - 2b)
// a = N(N - 2b) / (2(N - b))
