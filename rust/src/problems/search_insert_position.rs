// https://leetcode.com/problems/search-insert-position/
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = nums.iter().len() as i32;
    let mut mid: i32;
    while start != end {
        mid = (start + end) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] > target {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
        if end < 0 {
            return 0;
        }
    }
    return start;
}