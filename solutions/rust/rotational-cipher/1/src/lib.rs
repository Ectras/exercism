fn shift(val: char, lower: char, upper: char, amount: u8) -> char {
    let mut val = val as u8;
    let lower = lower as u8;
    let upper = upper as u8;
    val -= lower;
    val += amount;
    val %= upper - lower + 1;
    val += lower;
    val as char
}

pub fn rotate(input: &str, key: u8) -> String {
    assert!(key <= 26);

    input
        .chars()
        .map(|c| match c {
            'a'..='z' => shift(c, 'a', 'z', key),
            'A'..='Z' => shift(c, 'A', 'Z', key),
            _ => c,
        })
        .collect()
}
