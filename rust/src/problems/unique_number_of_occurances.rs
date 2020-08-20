// https://leetcode.com/problems/unique-number-of-occurrences/
use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
  let frequency: HashMap<i32, i32> = arr.iter().fold(HashMap::new(), |mut acc, curr| {
    *acc.entry(*curr).or_insert(0) += 1;
    return acc;
  });
  let frequency_hash_set: HashSet<i32> = frequency.values().cloned().collect();
  return if frequency.values().len() == frequency_hash_set.len() {
    true
  } else {
    false
  };
}
