// https://leetcode.com/problems/reverse-only-letters/
pub fn reverse_only_letters(s: String) -> String {
  let mut end: usize = s.len() - 1;
  let reversed: String = s
    .chars()
    .map(|letter| {
      let mut tmp: char = letter;
      if letter.is_alphabetic() {
        while !s.chars().nth(end).unwrap().is_alphabetic() {
          end -= 1;
        }
        tmp = s.chars().nth(end).unwrap();
        if end != 0 {
          end -= 1;
        }
      }
      return tmp;
    })
    .collect::<String>();
  return reversed;
}