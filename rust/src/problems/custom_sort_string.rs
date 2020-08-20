use std::collections::HashMap;

//https://leetcode.com/problems/custom-sort-string/
pub fn sort(s: String, t: String) -> String {
  // build your custom criteria for sort
  let sort_criteria: HashMap<char, i32> =
    s.chars()
      .enumerate()
      .fold(HashMap::new(), |mut acc, (index, letter)| {
        *acc.entry(letter).or_insert(index as i32) = index as i32;
        return acc;
      });

  // vectorize t
  let mut vector_t: Vec<char> = t.chars().collect::<Vec<char>>();

  // sort vector
  vector_t.sort_by(|l1, l2| {
    let c1 = sort_criteria.get(l1);
    let c2 = sort_criteria.get(l2);
    let c1_val: i32 = match c1 {
      Some(x) => *x as i32,
      None => i32::MAX,
    };
    let c2_val: i32 = match c2 {
      Some(x) => *x as i32,
      None => i32::MAX,
    };
    return c1_val.cmp(&c2_val);
  });

  // stringify vector
  let sorted: String = vector_t.iter().collect::<String>();
  return sorted;
}
