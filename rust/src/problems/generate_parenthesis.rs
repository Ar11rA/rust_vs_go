// https://leetcode.com/problems/generate-parentheses
pub fn generate_parenthesis(n: i32) -> Vec<String> {
  let mut results: Vec<String> = Vec::new();
  backtrack(&mut results, n * 2, 0, 0, "".to_string());
  return results;
}

fn backtrack(results: &mut Vec<String>, max: i32, open: i32, close: i32, result: String) {
  if max == result.len() as i32 {
    results.push(result.clone())
  }
  if open < max/2 {
    backtrack(results, max, open + 1, close, result.clone() + "(")
  }
  if close < open {
    backtrack(results, max, open, close + 1, result.clone() + ")")
  }
}