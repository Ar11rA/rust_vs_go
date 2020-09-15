use std::cmp::Ordering;

// https://leetcode.com/problems/reorder-data-in-log-files/
pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
  let (mut word_logs, mut digit_logs) = split_word_and_digit_logs(logs);
  word_logs.sort_by(|log1, log2| {
    let l1: Vec<&str> = log1.splitn(2, ' ').collect();
    let l2: Vec<&str> = log2.splitn(2, ' ').collect();
    let ordering = l1[1].cmp(l2[1]);
    if let Ordering::Equal = ordering {
      return l1[0].cmp(l2[0]);
    }
    return ordering;
  });
  word_logs.append(&mut digit_logs);
  return word_logs;
}

fn split_word_and_digit_logs(logs: Vec<String>) -> (Vec<String>, Vec<String>) {
  let mut digit_logs: Vec<String> = Vec::new();
  let mut word_logs: Vec<String> = Vec::new();
  logs
    .into_iter()
    .for_each(|log| {
      let words: Vec<String> = log
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|w| w.to_string())
        .collect();
      let parsed_word = words[1]
        .chars()
        .nth(0)
        .unwrap()
        .to_string()
        .parse::<i64>();
      match parsed_word {
        Ok(_) => digit_logs.push(log),
        Err(_) => word_logs.push(log),
      }
    });
  return (word_logs, digit_logs);
}