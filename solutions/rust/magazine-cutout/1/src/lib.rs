// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_pool = magazine.to_vec();
    for searched_word in note {
        let hit = word_pool.iter().position(|x| x == searched_word);
        if let Some(idx) = hit {
            word_pool.swap_remove(idx);
        } else {
            return false;
        }
    }
    true
}
