#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    let digits = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<_>, _>>()?;

    let result = digits
        .windows(span)
        .map(|w| w.iter().map(|&d| d as u64).product())
        .max()
        .unwrap();

    Ok(result)
}
