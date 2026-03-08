use itertools::Itertools;
use modinverse::modinverse;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

fn encode_char(c: char, a: i32, b: i32) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }

    let i = c as u8 - b'a';
    let val = (a * (i as i32) + b).rem_euclid(M) as u8;
    (val + b'a').into()
}

fn decode_char(c: char, a: i32, b: i32) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }

    let y = c as u8 - b'a';
    let ainv = modinverse(a, M).unwrap();
    let val = (ainv * (y as i32 - b)).rem_euclid(M) as u8;
    (val + b'a').into()
}

fn check_coprime(a: i32) -> Result<(), AffineCipherError> {
    if gcd::binary_u32(a.try_into().unwrap(), M.try_into().unwrap()) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(())
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    check_coprime(a)?;

    let out = plaintext
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .map(|c| encode_char(c, a, b))
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join(" ");
    Ok(out)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    check_coprime(a)?;

    let out = ciphertext
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| decode_char(c, a, b))
        .collect();
    Ok(out)
}
