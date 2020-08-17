// https://leetcode.com/problems/is-subsequence/submissions/
pub fn is_subsequence(s: String, t: String) -> bool {
    let mut ctr = 0;
    let mut idx = 0;
    for c in t.chars() {
        if s.as_bytes()[idx] == c as u8 {
            ctr += 1;
            idx += 1;
        }
        if ctr == s.len() {
            return true;
        }
    }
    return false;
}