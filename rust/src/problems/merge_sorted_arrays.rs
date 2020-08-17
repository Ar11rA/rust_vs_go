// https://leetcode.com/problems/merge-sorted-array/

pub fn merge(nums1: &mut Vec<i32>, _m: i32, nums2: &mut Vec<i32>, _n: i32) -> Vec<i32> {
    nums1.append(nums2);
    nums1.sort();
    return nums1.to_vec();
}