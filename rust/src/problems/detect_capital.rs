// https://leetcode.com/problems/detect-capital
pub fn detect_capital_use(word: String) -> bool {
  return word
    .chars()
    .all(|x| x.is_uppercase())
    || word
    .chars()
    .skip(1)
    .all(|x| x.is_lowercase());
}