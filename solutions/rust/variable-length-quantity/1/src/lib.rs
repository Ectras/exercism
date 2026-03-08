#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

fn value_to_bytes(mut value: u32) -> Vec<u8> {
    let mut out = vec![];
    while value > 0 {
        let mut x = (value & 0b01111111) as u8;
        if !out.is_empty() {
            x |= 0b10000000;
        }
        out.push(x);
        value >>= 7;
    }
    if out.is_empty() {
        vec![0]
    } else {
        out.reverse();
        out
    }
}

fn bytes_to_value(bytes: impl Iterator<Item = u8>) -> Result<u32, Error> {
    let mut number_bytes = vec![];
    for byte in bytes {
        let expects_more = byte & 0b10000000 > 0;
        number_bytes.push(byte & 0b01111111);
        if !expects_more {
            let number = number_bytes
                .iter()
                .copied()
                .fold(0u32, |acc, val| (acc << 7) + val as u32);
            return Ok(number);
        }
    }
    Err(Error::IncompleteNumber)
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().copied().flat_map(value_to_bytes).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut it = bytes.iter().copied();
    let mut out = vec![];
    while it.len() > 0 {
        out.push(bytes_to_value(&mut it)?);
    }
    Ok(out)
}
