use std::collections::HashMap;

// https://leetcode.com/problems/single-number/
// https://leetcode.com/problems/single-number-ii/
// more generic to cater to more than twice
pub fn find(nums: Vec<i32>) -> i32 {
  return nums
    .into_iter()
    .fold(HashMap::new(), |mut acc: HashMap<i32, i32>, curr| {
      *acc.entry(curr).or_insert(0) += 1;
      return acc;
    })
    .into_iter()
    .find_map(|(n, f)| -> Option<i32> {
      return if f == 1 { Some(n) } else { None };
    })
    .unwrap();
}

// https://leetcode.com/problems/single-number-iii/
// https://stackoverflow.com/questions/28909583/removing-entries-from-a-hashmap-based-on-value
pub fn filter(nums: Vec<i32>) -> Vec<i32> {
  return nums
    .into_iter()
    .fold(HashMap::new(), |mut acc: HashMap<i32, i32>, curr| {
      *acc.entry(curr).or_insert(0) += 1;
      return acc;
    })
    .into_iter()
    .filter_map(|(n, f)| -> Option<i32> {
      return if f == 1 { Some(n) } else { None };
    })
    .collect::<Vec<_>>();
}
