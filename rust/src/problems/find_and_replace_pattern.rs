use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    return words
        .into_iter()
        .filter(|w| is_same_pattern(w.parse().unwrap(), pattern.clone()))
        .collect();
}

fn is_same_pattern(word: String, pattern: String) -> bool {
    let mut hash: HashMap<char, char> = HashMap::new();
    for (p, w) in word.chars().into_iter().zip(pattern.chars().into_iter()) {
        if hash.insert(p, w).unwrap_or(w) != w {
            return false;
        }
    }
    return HashSet::<&char>::from_iter(hash.values()).len() == hash.values().len();
}