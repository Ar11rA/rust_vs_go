// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    for (i, _) in nums.iter().enumerate() {
        for j in i + 1..nums.len() {
            if (nums[i] - 1) * (nums[j] - 1) > max {
                max = (nums[i] - 1) * (nums[j] - 1);
            }
        }
    }
    return max;
}