// https://leetcode.com/problems/rotate-array/
pub fn rotate(nums: Vec<i32>, k: i32) -> Vec<i32> {
  return nums
    .iter()
    .enumerate()
    .map(|(i, _)| return nums[(i + k as usize) % nums.len()])
    .collect();
}
