use rand::{distributions::Uniform, thread_rng, Rng};

fn shift_right(a: u8, b: u8) -> u8 {
    (a + b) % 26
}

fn shift_left(a: u8, b: u8) -> u8 {
    if a >= b {
        a - b
    } else {
        26 - b + a
    }
}

fn check_key(key: &str) -> bool {
    key.len() > 0 && key.chars().all(|c| c.is_ascii_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !check_key(key) {
        return None;
    }

    let key_iter = key.as_bytes().iter().cycle().map(|b| b - b'a');
    Some(
        s.as_bytes()
            .iter()
            .map(|b| b - b'a')
            .zip(key_iter)
            .map(|(s, k)| shift_right(s, k) + b'a')
            .map(|b| char::from_u32(b.into()).unwrap())
            .collect(),
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !check_key(key) {
        return None;
    }

    let key_iter = key.as_bytes().iter().cycle().map(|b| b - b'a');
    Some(
        s.as_bytes()
            .iter()
            .map(|b| b - b'a')
            .zip(key_iter)
            .map(|(s, k)| shift_left(s, k) + b'a')
            .map(|b| char::from_u32(b.into()).unwrap())
            .collect(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let rng = thread_rng();
    let key = rng
        .sample_iter(Uniform::new_inclusive('a', 'z'))
        .take(100)
        .collect::<String>();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
