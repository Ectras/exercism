use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 {
        return vec![];
    }

    let mut marked = HashSet::new();
    let mut primes = Vec::new();

    for i in 2..=upper_bound {
        if marked.contains(&i) {
            continue;
        }

        primes.push(i);
        for j in (i + i..=upper_bound).step_by(i.try_into().unwrap()) {
            marked.insert(j);
        }
    }

    primes
}
