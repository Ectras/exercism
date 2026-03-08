/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&c| c != '-')
        .enumerate()
        .map(|(i, c)| match c {
            '0'..='9' => char::to_digit(c, 10),
            'X' if i == 9 => Some(10),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .filter(|vec| vec.len() == 10)
        .map(|vec| {
            vec.iter()
                .zip((1..=10).rev())
                .map(|(a, b)| a * b)
                .sum::<u32>()
                % 11
                == 0
        })
        .unwrap_or_default()
}
