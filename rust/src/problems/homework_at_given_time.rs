// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/

pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
  let mut result: i32 = 0;
  for (i, start) in start_time.iter().enumerate() {
    if end_time[i] > query_time && *start < query_time {
      result += 1
    }
  }
  return result;
}
