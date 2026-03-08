use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let original = word.to_lowercase().chars().collect::<Vec<_>>();
    let mut sorted = original.clone();
    sorted.sort_unstable();

    let mut out = HashSet::new();
    for &candidate in possible_anagrams {
        if word.len() != candidate.len() {
            // Words differ in length
            continue;
        }

        let mut cand = candidate.to_lowercase().chars().collect::<Vec<_>>();
        if original == cand {
            // It's the same word
            continue;
        }

        cand.sort_unstable();
        if sorted == cand {
            out.insert(candidate);
        }
    }
    out
}
