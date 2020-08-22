// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses

pub fn min_remove_to_make_valid(s: String) -> String {
  let mut stack = Vec::new();
  let mut chars: Vec<char> = s.chars().collect();

  for i in 0..chars.len() {
    match chars[i] {
      '(' => stack.push(i),
      ')' => {
        if let Some(_) = stack.pop() {
          continue;
        } else {
          chars[i] = '\0';
        }
      },
      _ => continue,
    }
  }
  while let Some(i) = stack.pop() {
    chars[i] = '\0';
  }
  return chars.iter().collect::<String>()
}



