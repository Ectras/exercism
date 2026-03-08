pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    let slices = digits.len().checked_sub(len);
    for i in 0..slices.map_or(0, |x| x + 1) {
        result.push(digits[i..i + len].to_string());
    }
    result
}
