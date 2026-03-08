mod intersperse;

use intersperse::IntersperseN;

fn reverse(c: char) -> char {
    if c.is_ascii_alphabetic() {
        let val = c as u8;
        let val_rev = b'z' - val;
        (val_rev + b'a').into()
    } else {
        c
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .map(reverse)
        .intersperse_n(5, ' ')
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(reverse)
        .collect()
}
