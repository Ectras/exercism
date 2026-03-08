use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
   mod_pow(b_pub, a, p)
}

fn mod_pow(base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus <= 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = base.pow(2) % modulus
    }
    result
}