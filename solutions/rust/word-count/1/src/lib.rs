use std::collections::HashMap;

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() && c != '\'' || c.is_ascii_whitespace()
}

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut frequencies = HashMap::new();
    for word in words
        .to_ascii_lowercase()
        .split(is_separator)
        .map(|w| w.trim_matches('\''))
        .filter(|w| !w.is_empty())
    {
        // Adapted from https://users.rust-lang.org/t/efficient-string-hashmaps-for-a-frequency-count/7752
        // This avoid using `.to_string` for each key lookup
        frequencies
            .get_mut(word)
            .map(|count| *count += 1)
            .unwrap_or_else(|| {
                frequencies.insert(word.into(), 1);
            });
    }
    frequencies
}
