use std::collections::HashSet;

static ASCII_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut missing = HashSet::from(ASCII_CHARS);
    for c in sentence.chars() {
        missing.remove(&c.to_ascii_lowercase());
        if missing.is_empty() {
            return true;
        }
    }
    
    false
}
