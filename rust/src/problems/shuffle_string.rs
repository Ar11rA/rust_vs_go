// https://leetcode.com/problems/shuffle-string/
pub fn restore_string(s: String, indices: Vec<i32>) -> String {
  let mut ans = vec!['_'; s.len()];
  let v = s.as_bytes();
  for i in 0..indices.len() {
    ans[indices[i] as usize] = v[i] as char;
  }
  return ans.iter().collect();
}
