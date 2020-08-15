// https://leetcode.com/problems/running-sum-of-1d-array/
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum: i32 = 0;
    return nums
        .iter()
        .map(|x| {
            sum = sum + x;
            return sum;
        })
        .collect();
}