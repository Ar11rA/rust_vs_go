package problems

// https://leetcode.com/problems/rotate-array/
func Rotate(nums []int, k int) []int {
	result := make([]int, len(nums))
	for index, _ := range nums {
		result[index] = nums[(index + k) % len(nums)]
	}
	return result
}
