use itertools::Itertools;

pub fn check(candidate: &str) -> bool {
    candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c != ' ' && c != '-')
        .all_unique()
}
